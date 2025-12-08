//! Service layer tests for Party use cases
//!
//! Uses shared test database with #[tokio::test].

use application::party::{
    CreatePartyInput, CreatePartyUseCase, GetPartyUseCase, ListPartiesUseCase,
};
use infrastructure::repositories::PartyRepositoryImpl;
use rstest::fixture;
use shared::AppError;
use sqlx::postgres::PgPoolOptions;

// =============================================================================
// Test Setup
// =============================================================================

async fn get_test_pool() -> sqlx::PgPool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to connect to test database");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

fn unique_name(prefix: &str) -> String {
    format!("{}_{}", prefix, uuid::Uuid::now_v7())
}

// =============================================================================
// Fixtures
// =============================================================================

#[fixture]
fn repo() -> PartyRepositoryImpl {
    PartyRepositoryImpl::new()
}

#[fixture]
fn minimal_input() -> impl Fn(&str) -> CreatePartyInput {
    |name: &str| CreatePartyInput {
        party_type: "company".to_string(),
        display_name: name.to_string(),
        legal_name: String::new(),
        tin: String::new(),
        registration_number: String::new(),
    }
}

#[fixture]
fn full_input() -> CreatePartyInput {
    CreatePartyInput {
        party_type: "company".to_string(),
        display_name: unique_name("AcmeCorp"),
        legal_name: "Acme Corporation Ltd.".to_string(),
        tin: "0123456789".to_string(),
        registration_number: "BRN-12345".to_string(),
    }
}

// =============================================================================
// CreatePartyUseCase Tests
// =============================================================================

#[tokio::test]
async fn create_party_with_minimal_data() {
    let pool = get_test_pool().await;
    let use_case = CreatePartyUseCase::new(repo());

    let result = use_case
        .execute(&pool, minimal_input()(&unique_name("Minimal")))
        .await;

    assert!(result.is_ok());
    let party = result.unwrap();
    assert!(party.display_name().value().starts_with("Minimal_"));
    assert!(party.legal_name().is_none());
}

#[tokio::test]
async fn create_party_with_full_data() {
    let pool = get_test_pool().await;
    let use_case = CreatePartyUseCase::new(repo());

    let result = use_case.execute(&pool, full_input()).await;

    if let Err(ref e) = result {
        eprintln!("Error creating party: {:?}", e);
    }
    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let party = result.unwrap();
    assert!(party.display_name().value().starts_with("AcmeCorp_"));
    assert_eq!(party.legal_name().unwrap().value(), "Acme Corporation Ltd.");
    assert_eq!(party.tin().unwrap().value(), "0123456789");
}

#[tokio::test]
async fn create_party_fails_with_empty_display_name() {
    let pool = get_test_pool().await;
    let use_case = CreatePartyUseCase::new(repo());

    let result = use_case.execute(&pool, minimal_input()("")).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn create_party_fails_with_invalid_party_type() {
    let pool = get_test_pool().await;
    let use_case = CreatePartyUseCase::new(repo());
    let mut input = minimal_input()(&unique_name("InvalidType"));
    input.party_type = "invalid".to_string();

    let result = use_case.execute(&pool, input).await;

    assert!(result.is_err());
}

// =============================================================================
// GetPartyUseCase Tests
// =============================================================================

#[tokio::test]
async fn get_party_returns_created_party() {
    let pool = get_test_pool().await;
    let name = unique_name("FindMe");

    // Create
    let create_use_case = CreatePartyUseCase::new(repo());
    let party = create_use_case
        .execute(&pool, minimal_input()(&name))
        .await
        .unwrap();

    // Get
    let get_use_case = GetPartyUseCase::new(repo());
    let result = get_use_case.execute(&pool, party.id()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().display_name().value(), name);
}

#[tokio::test]
async fn get_party_returns_not_found() {
    let pool = get_test_pool().await;
    let use_case = GetPartyUseCase::new(repo());

    let result = use_case.execute(&pool, uuid::Uuid::now_v7()).await;

    assert!(matches!(result, Err(AppError::NotFound(_))));
}

// =============================================================================
// ListPartiesUseCase Tests
// =============================================================================

#[tokio::test]
async fn list_parties_returns_data() {
    let pool = get_test_pool().await;

    // Create one
    let create_use_case = CreatePartyUseCase::new(repo());
    create_use_case
        .execute(&pool, minimal_input()(&unique_name("ListTest")))
        .await
        .unwrap();

    // List
    let list_use_case = ListPartiesUseCase::new(repo());
    let (parties, pagination) = list_use_case.execute(&pool, 1, 10).await.unwrap();

    assert!(!parties.is_empty());
    assert!(pagination.total >= 1);
}

#[tokio::test]
async fn list_parties_pagination() {
    let pool = get_test_pool().await;
    let list_use_case = ListPartiesUseCase::new(repo());

    let (parties, pagination) = list_use_case.execute(&pool, 1, 5).await.unwrap();

    assert!(parties.len() <= 5);
    assert_eq!(pagination.page, 1);
}

// =============================================================================
// Error Cases
// =============================================================================

#[tokio::test]
async fn create_party_fails_with_display_name_too_long() {
    let pool = get_test_pool().await;
    let use_case = CreatePartyUseCase::new(repo());
    let long_name = "a".repeat(256);

    let result = use_case.execute(&pool, minimal_input()(&long_name)).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn create_party_accepts_empty_optional_fields() {
    let pool = get_test_pool().await;
    let use_case = CreatePartyUseCase::new(repo());

    // All optional fields empty - should succeed
    let result = use_case
        .execute(&pool, minimal_input()(&unique_name("EmptyOptionals")))
        .await;

    assert!(result.is_ok());
    let party = result.unwrap();
    assert!(party.legal_name().is_none());
    assert!(party.tin().is_none());
    assert!(party.registration_number().is_none());
}

#[tokio::test]
async fn create_person_party() {
    let pool = get_test_pool().await;
    let use_case = CreatePartyUseCase::new(repo());
    let mut input = minimal_input()(&unique_name("PersonTest"));
    input.party_type = "person".to_string();

    let result = use_case.execute(&pool, input).await;

    assert!(result.is_ok());
    let party = result.unwrap();
    assert_eq!(party.party_type().as_str(), "person");
}
