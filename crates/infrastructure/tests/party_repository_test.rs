//! Repository tests for Party
//!
//! Uses shared test database with #[tokio::test].

mod common;

use application::ports::PartyRepository;
use common::{
    PartyRepositoryImpl,
    fixtures::{fake_party, fake_party_full, seed_known, seed_n, seed_one},
    get_test_pool,
};
use domain::party::DisplayName;

// ============================================================================
// CRUD Tests
// ============================================================================

#[tokio::test]
async fn create_and_find() {
    let pool = get_test_pool().await;
    let repo = PartyRepositoryImpl::new();

    let party = fake_party();
    repo.create(&pool, &party).await.unwrap();

    let found = repo.find_by_id(&pool, party.id()).await.unwrap().unwrap();

    assert_eq!(found.id(), party.id());
    assert_eq!(found.display_name().value(), party.display_name().value());
}

#[tokio::test]
async fn create_with_all_fields() {
    let pool = get_test_pool().await;
    let repo = PartyRepositoryImpl::new();

    let party = fake_party_full();
    repo.create(&pool, &party).await.unwrap();

    let found = repo.find_by_id(&pool, party.id()).await.unwrap().unwrap();

    assert!(found.legal_name().is_some());
    assert!(found.tin().is_some());
    assert!(found.registration_number().is_some());
    assert!(found.is_active());
}

#[tokio::test]
async fn find_nonexistent_returns_none() {
    let pool = get_test_pool().await;
    let repo = PartyRepositoryImpl::new();

    let result = repo.find_by_id(&pool, uuid::Uuid::now_v7()).await.unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn update_party() {
    let pool = get_test_pool().await;
    let repo = PartyRepositoryImpl::new();

    let mut party = seed_one(&pool, &repo).await;
    let new_name = format!("Updated_{}", uuid::Uuid::now_v7());

    party.update_display_name(DisplayName::new(&new_name).unwrap());
    repo.update(&pool, &party).await.unwrap();

    let found = repo.find_by_id(&pool, party.id()).await.unwrap().unwrap();
    assert_eq!(found.display_name().value(), new_name);
}

#[tokio::test]
async fn delete_party() {
    let pool = get_test_pool().await;
    let repo = PartyRepositoryImpl::new();

    let party = seed_one(&pool, &repo).await;

    repo.delete(&pool, party.id()).await.unwrap();

    let found = repo.find_by_id(&pool, party.id()).await.unwrap();
    assert!(found.is_none());
}

// ============================================================================
// Query Tests
// ============================================================================

#[tokio::test]
async fn find_seeded_parties() {
    let pool = get_test_pool().await;
    let repo = PartyRepositoryImpl::new();

    let (acme, wayne, stark) = seed_known(&pool, &repo).await;

    for party in [&acme, &wayne, &stark] {
        let found = repo.find_by_id(&pool, party.id()).await.unwrap().unwrap();
        assert_eq!(found.display_name().value(), party.display_name().value());
    }
}

#[tokio::test]
async fn pagination_basic() {
    let pool = get_test_pool().await;
    let repo = PartyRepositoryImpl::new();

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
    let repo = PartyRepositoryImpl::new();

    let (items, _) = repo.find_paginated(&pool, 1, 5).await.unwrap();

    assert!(items.len() <= 5);
}

// ============================================================================
// Error Cases
// ============================================================================

#[tokio::test]
async fn delete_nonexistent_succeeds_silently() {
    let pool = get_test_pool().await;
    let repo = PartyRepositoryImpl::new();

    // Deleting non-existent ID should not error (idempotent)
    let result = repo.delete(&pool, uuid::Uuid::now_v7()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn update_nonexistent_succeeds_silently() {
    let pool = get_test_pool().await;
    let repo = PartyRepositoryImpl::new();

    // Create party but don't persist it
    let party = fake_party();

    // Update should succeed (affected 0 rows is not an error)
    let result = repo.update(&pool, &party).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn pagination_empty_result() {
    let pool = get_test_pool().await;
    let repo = PartyRepositoryImpl::new();

    // Request page far beyond data
    let (items, meta) = repo.find_paginated(&pool, 9999, 10).await.unwrap();

    assert!(items.is_empty());
    assert_eq!(meta.page, 9999);
}
