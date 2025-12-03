mod common;

use application::ports::OrganizationRepository;
use common::{
    fixtures::{fake_org, fake_org_full, seed_known, seed_n, seed_one},
    OrganizationRepositoryImpl,
};
use domain::organization::OrganizationName;
use sqlx::PgPool;

// ============================================================================
// CRUD Tests
// ============================================================================

#[sqlx::test(migrations = "../../migrations")]
async fn create_and_find(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();

    let org = fake_org();
    repo.create(&pool, &org).await.unwrap();

    let found = repo.find_by_id(&pool, org.id()).await.unwrap().unwrap();

    assert_eq!(found.id(), org.id());
    assert_eq!(found.name().value(), org.name().value());
    assert!(found.is_active());
}

#[sqlx::test(migrations = "../../migrations")]
async fn create_with_all_fields(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();

    let org = fake_org_full();
    repo.create(&pool, &org).await.unwrap();

    let found = repo.find_by_id(&pool, org.id()).await.unwrap().unwrap();

    assert!(found.email().is_some());
    assert!(found.phone().is_some());
    assert!(found.website().is_some());
    assert!(found.industry().is_some());
    assert!(found.city().is_some());
}

#[sqlx::test(migrations = "../../migrations")]
async fn find_nonexistent_returns_none(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();

    let result = repo.find_by_id(&pool, uuid::Uuid::now_v7()).await.unwrap();

    assert!(result.is_none());
}

#[sqlx::test(migrations = "../../migrations")]
async fn update_organization(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();

    let mut org = seed_one(&pool, &repo).await;

    org.update_name(OrganizationName::new("Updated Name").unwrap());
    repo.update(&pool, &org).await.unwrap();

    let found = repo.find_by_id(&pool, org.id()).await.unwrap().unwrap();
    assert_eq!(found.name().value(), "Updated Name");
}

#[sqlx::test(migrations = "../../migrations")]
async fn delete_organization(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();

    let org = seed_one(&pool, &repo).await;

    repo.delete(&pool, org.id()).await.unwrap();

    let found = repo.find_by_id(&pool, org.id()).await.unwrap();
    assert!(found.is_none());
}

// ============================================================================
// Query Tests
// ============================================================================

#[sqlx::test(migrations = "../../migrations")]
async fn find_seeded_organizations(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();

    let (acme, wayne, stark) = seed_known(&pool, &repo).await;

    for org in [&acme, &wayne, &stark] {
        let found = repo.find_by_id(&pool, org.id()).await.unwrap().unwrap();
        assert_eq!(found.name().value(), org.name().value());
    }
}

#[sqlx::test(migrations = "../../migrations")]
async fn pagination_basic(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();

    seed_n(&pool, &repo, 15).await;

    // First page
    let (page1, meta1) = repo.find_paginated(&pool, 1, 10).await.unwrap();
    assert_eq!(page1.len(), 10);
    assert_eq!(meta1.total, 15);
    assert_eq!(meta1.total_pages, 2);
    assert!(meta1.has_next);
    assert!(!meta1.has_prev);

    // Second page
    let (page2, meta2) = repo.find_paginated(&pool, 2, 10).await.unwrap();
    assert_eq!(page2.len(), 5);
    assert!(!meta2.has_next);
    assert!(meta2.has_prev);
}

// ============================================================================
// Pagination Edge Cases
// ============================================================================

#[sqlx::test(migrations = "../../migrations")]
async fn pagination_first_page(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();
    seed_n(&pool, &repo, 25).await;

    let (items, meta) = repo.find_paginated(&pool, 1, 10).await.unwrap();

    assert_eq!(items.len(), 10);
    assert_eq!(meta.total_pages, 3);
    assert_eq!(meta.total, 25);
}

#[sqlx::test(migrations = "../../migrations")]
async fn pagination_last_page(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();
    seed_n(&pool, &repo, 25).await;

    let (items, meta) = repo.find_paginated(&pool, 3, 10).await.unwrap();

    assert_eq!(items.len(), 5);
    assert_eq!(meta.total_pages, 3);
}

#[sqlx::test(migrations = "../../migrations")]
async fn pagination_small_page_size(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();
    seed_n(&pool, &repo, 12).await;

    let (items, meta) = repo.find_paginated(&pool, 1, 5).await.unwrap();

    assert_eq!(items.len(), 5);
    assert_eq!(meta.total_pages, 3);
    assert_eq!(meta.total, 12);
}

#[sqlx::test(migrations = "../../migrations")]
async fn pagination_large_page_size(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();
    seed_n(&pool, &repo, 50).await;

    let (items, meta) = repo.find_paginated(&pool, 1, 100).await.unwrap();

    assert_eq!(items.len(), 50);
    assert_eq!(meta.total_pages, 1);
}

#[sqlx::test(migrations = "../../migrations")]
async fn pagination_single_item(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();
    seed_n(&pool, &repo, 1).await;

    let (items, meta) = repo.find_paginated(&pool, 1, 10).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(meta.total_pages, 1);
    assert_eq!(meta.total, 1);
}

#[sqlx::test(migrations = "../../migrations")]
async fn pagination_exact_fit(pool: PgPool) {
    let repo = OrganizationRepositoryImpl::new();
    seed_n(&pool, &repo, 10).await;

    let (items, meta) = repo.find_paginated(&pool, 2, 5).await.unwrap();

    assert_eq!(items.len(), 5);
    assert_eq!(meta.total_pages, 2);
}
