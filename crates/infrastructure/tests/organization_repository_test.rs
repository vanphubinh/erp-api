//! Repository tests for Organization
//!
//! Uses shared test database with #[tokio::test].

mod common;

use application::ports::OrganizationRepository;
use common::{
    OrganizationRepositoryImpl,
    fixtures::{fake_org, fake_org_full, seed_known, seed_n, seed_one},
    get_test_pool,
};
use domain::organization::OrganizationName;

// ============================================================================
// CRUD Tests
// ============================================================================

#[tokio::test]
async fn create_and_find() {
    let pool = get_test_pool().await;
    let repo = OrganizationRepositoryImpl::new();

    let org = fake_org();
    repo.create(&pool, &org).await.unwrap();

    let found = repo.find_by_id(&pool, org.id()).await.unwrap().unwrap();

    assert_eq!(found.id(), org.id());
    assert_eq!(found.name().value(), org.name().value());
    assert!(found.is_active());
}

#[tokio::test]
async fn create_with_all_fields() {
    let pool = get_test_pool().await;
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

#[tokio::test]
async fn find_nonexistent_returns_none() {
    let pool = get_test_pool().await;
    let repo = OrganizationRepositoryImpl::new();

    let result = repo.find_by_id(&pool, uuid::Uuid::now_v7()).await.unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn update_organization() {
    let pool = get_test_pool().await;
    let repo = OrganizationRepositoryImpl::new();

    let mut org = seed_one(&pool, &repo).await;
    let new_name = format!("Updated_{}", uuid::Uuid::now_v7());

    org.update_name(OrganizationName::new(&new_name).unwrap());
    repo.update(&pool, &org).await.unwrap();

    let found = repo.find_by_id(&pool, org.id()).await.unwrap().unwrap();
    assert_eq!(found.name().value(), new_name);
}

#[tokio::test]
async fn delete_organization() {
    let pool = get_test_pool().await;
    let repo = OrganizationRepositoryImpl::new();

    let org = seed_one(&pool, &repo).await;

    repo.delete(&pool, org.id()).await.unwrap();

    let found = repo.find_by_id(&pool, org.id()).await.unwrap();
    assert!(found.is_none());
}

// ============================================================================
// Query Tests
// ============================================================================

#[tokio::test]
async fn find_seeded_organizations() {
    let pool = get_test_pool().await;
    let repo = OrganizationRepositoryImpl::new();

    let (acme, wayne, stark) = seed_known(&pool, &repo).await;

    for org in [&acme, &wayne, &stark] {
        let found = repo.find_by_id(&pool, org.id()).await.unwrap().unwrap();
        assert_eq!(found.name().value(), org.name().value());
    }
}

#[tokio::test]
async fn pagination_basic() {
    let pool = get_test_pool().await;
    let repo = OrganizationRepositoryImpl::new();

    seed_n(&pool, &repo, 15).await;

    // Get paginated results
    let (items, meta) = repo.find_paginated(&pool, 1, 10).await.unwrap();

    // Should have items (at least what we seeded)
    assert!(!items.is_empty());
    assert!(meta.total >= 15);
}

#[tokio::test]
async fn pagination_page_size() {
    let pool = get_test_pool().await;
    let repo = OrganizationRepositoryImpl::new();

    let (items, _) = repo.find_paginated(&pool, 1, 5).await.unwrap();

    assert!(items.len() <= 5);
}
