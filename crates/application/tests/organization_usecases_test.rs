//! Service layer tests for Organization use cases
//!
//! Uses shared test database with #[tokio::test].

use application::organization::{
    CreateOrganizationInput, CreateOrganizationUseCase, GetOrganizationUseCase,
    ListOrganizationsUseCase,
};
use infrastructure::repositories::OrganizationRepositoryImpl;
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
fn repo() -> OrganizationRepositoryImpl {
    OrganizationRepositoryImpl::new()
}

#[fixture]
fn minimal_input() -> impl Fn(&str) -> CreateOrganizationInput {
    |name: &str| CreateOrganizationInput {
        code: String::new(),
        name: name.to_string(),
        display_name: String::new(),
        tax_number: String::new(),
        registration_no: String::new(),
        phone: String::new(),
        email: String::new(),
        website: String::new(),
        parent_id: None,
    }
}

#[fixture]
fn full_input() -> CreateOrganizationInput {
    CreateOrganizationInput {
        code: unique_name("ORG"),
        name: unique_name("AcmeCorp"),
        display_name: "Acme Corporation".to_string(),
        tax_number: "0123456789".to_string(),
        registration_no: "BRN-12345".to_string(),
        phone: "+1-555-0100".to_string(),
        email: "contact@acme.com".to_string(),
        website: "https://acme.com".to_string(),
        parent_id: None,
    }
}

// =============================================================================
// CreateOrganizationUseCase Tests
// =============================================================================

#[tokio::test]
async fn create_organization_with_minimal_data() {
    let pool = get_test_pool().await;
    let use_case = CreateOrganizationUseCase::new(repo());

    let result = use_case
        .execute(&pool, minimal_input()(&unique_name("Minimal")))
        .await;

    assert!(result.is_ok());
    let org = result.unwrap();
    assert!(org.name().value().starts_with("Minimal_"));
    assert!(org.email().is_none());
}

#[tokio::test]
async fn create_organization_with_full_data() {
    let pool = get_test_pool().await;
    let use_case = CreateOrganizationUseCase::new(repo());

    let result = use_case.execute(&pool, full_input()).await;

    if let Err(ref e) = result {
        eprintln!("Error creating organization: {:?}", e);
    }
    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let org = result.unwrap();
    assert!(org.name().value().starts_with("AcmeCorp_"));
    assert_eq!(&**org.email().unwrap(), "contact@acme.com");
    assert!(org.code().unwrap().starts_with("ORG_"));
    assert_eq!(org.tax_number(), Some("0123456789"));
}

#[tokio::test]
async fn create_organization_fails_with_empty_name() {
    let pool = get_test_pool().await;
    let use_case = CreateOrganizationUseCase::new(repo());

    let result = use_case.execute(&pool, minimal_input()("")).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn create_organization_fails_with_invalid_email() {
    let pool = get_test_pool().await;
    let use_case = CreateOrganizationUseCase::new(repo());
    let mut input = minimal_input()(&unique_name("InvalidEmail"));
    input.email = "invalid-email".to_string();

    let result = use_case.execute(&pool, input).await;

    assert!(result.is_err());
}

// =============================================================================
// GetOrganizationUseCase Tests
// =============================================================================

#[tokio::test]
async fn get_organization_returns_created_org() {
    let pool = get_test_pool().await;
    let name = unique_name("FindMe");

    // Create
    let create_use_case = CreateOrganizationUseCase::new(repo());
    let org = create_use_case
        .execute(&pool, minimal_input()(&name))
        .await
        .unwrap();

    // Get
    let get_use_case = GetOrganizationUseCase::new(repo());
    let result = get_use_case.execute(&pool, org.id()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().name().value(), name);
}

#[tokio::test]
async fn get_organization_returns_not_found() {
    let pool = get_test_pool().await;
    let use_case = GetOrganizationUseCase::new(repo());

    let result = use_case.execute(&pool, uuid::Uuid::now_v7()).await;

    assert!(matches!(result, Err(AppError::NotFound(_))));
}

// =============================================================================
// ListOrganizationsUseCase Tests
// =============================================================================

#[tokio::test]
async fn list_organizations_returns_data() {
    let pool = get_test_pool().await;

    // Create one
    let create_use_case = CreateOrganizationUseCase::new(repo());
    create_use_case
        .execute(&pool, minimal_input()(&unique_name("ListTest")))
        .await
        .unwrap();

    // List
    let list_use_case = ListOrganizationsUseCase::new(repo());
    let (orgs, pagination) = list_use_case.execute(&pool, 1, 10).await.unwrap();

    assert!(!orgs.is_empty());
    assert!(pagination.total >= 1);
}

#[tokio::test]
async fn list_organizations_pagination() {
    let pool = get_test_pool().await;
    let list_use_case = ListOrganizationsUseCase::new(repo());

    let (orgs, pagination) = list_use_case.execute(&pool, 1, 5).await.unwrap();

    assert!(orgs.len() <= 5);
    assert_eq!(pagination.page, 1);
}

// =============================================================================
// Error Cases
// =============================================================================

#[tokio::test]
async fn create_organization_fails_with_invalid_website() {
    let pool = get_test_pool().await;
    let use_case = CreateOrganizationUseCase::new(repo());
    let mut input = minimal_input()(&unique_name("InvalidWebsite"));
    input.website = "not-a-url".to_string();

    let result = use_case.execute(&pool, input).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn create_organization_fails_with_name_too_long() {
    let pool = get_test_pool().await;
    let use_case = CreateOrganizationUseCase::new(repo());
    let long_name = "a".repeat(256);

    let result = use_case.execute(&pool, minimal_input()(&long_name)).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn create_organization_accepts_empty_optional_fields() {
    let pool = get_test_pool().await;
    let use_case = CreateOrganizationUseCase::new(repo());

    // All optional fields empty - should succeed
    let result = use_case
        .execute(&pool, minimal_input()(&unique_name("EmptyOptionals")))
        .await;

    assert!(result.is_ok());
    let org = result.unwrap();
    assert!(org.email().is_none());
    assert!(org.phone().is_none());
    assert!(org.website().is_none());
}
