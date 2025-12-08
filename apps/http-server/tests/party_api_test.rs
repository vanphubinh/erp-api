//! API integration tests for Party endpoints
//!
//! Uses a shared test database with #[tokio::test].

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_server::{app_state::AppState, routes::api_routes};
use rstest::fixture;
use serde_json::{Value, json};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use tower::ServiceExt;
use utoipa_axum::router::OpenApiRouter;

// =============================================================================
// Test Setup
// =============================================================================

async fn get_test_pool() -> PgPool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to connect to test database");

    // Run migrations
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

fn app(pool: PgPool) -> Router {
    let state = Arc::new(AppState { pool });
    let (router, _) = OpenApiRouter::new()
        .merge(api_routes())
        .with_state(state)
        .split_for_parts();
    router
}

// =============================================================================
// Helper Functions
// =============================================================================

async fn post_json(app: &Router, path: &str, body: &Value) -> (StatusCode, Value) {
    let req = Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&bytes).unwrap_or(json!({}));
    (status, json)
}

async fn get_json(app: &Router, path: &str) -> (StatusCode, Value) {
    let req = Request::builder()
        .method("GET")
        .uri(path)
        .body(Body::empty())
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&bytes).unwrap_or(json!({}));
    (status, json)
}

/// Generate unique name to avoid test conflicts in shared DB
fn unique_name(prefix: &str) -> String {
    format!("{}_{}", prefix, uuid::Uuid::now_v7())
}

// =============================================================================
// Fixtures
// =============================================================================

#[fixture]
fn minimal_party() -> impl Fn(&str) -> Value {
    |name: &str| {
        json!({
            "partyType": "company",
            "displayName": name
        })
    }
}

#[fixture]
fn full_party() -> Value {
    json!({
        "partyType": "company",
        "displayName": unique_name("FullDataCorp"),
        "legalName": "Full Data Corporation Ltd.",
        "tin": "0123456789",
        "registrationNumber": "BRN-12345"
    })
}

// =============================================================================
// POST /api/parties/create
// =============================================================================

#[tokio::test]
async fn create_party_success() {
    let pool = get_test_pool().await;
    let app = app(pool);

    let (status, body) = post_json(
        &app,
        "/api/parties/create",
        &minimal_party()(&unique_name("CreateTest")),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn create_party_with_full_data() {
    let pool = get_test_pool().await;
    let app = app(pool);

    let (status, body) = post_json(&app, "/api/parties/create", &full_party()).await;

    if status != StatusCode::CREATED {
        eprintln!("Error response: {:?}", body);
    }
    assert_eq!(status, StatusCode::CREATED);
}

#[tokio::test]
async fn create_party_fails_with_empty_display_name() {
    let pool = get_test_pool().await;
    let app = app(pool);

    let (status, _) = post_json(&app, "/api/parties/create", &minimal_party()("")).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_party_fails_with_invalid_party_type() {
    let pool = get_test_pool().await;
    let app = app(pool);

    let payload = json!({
        "partyType": "invalid",
        "displayName": unique_name("InvalidType")
    });

    let (status, _) = post_json(&app, "/api/parties/create", &payload).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// =============================================================================
// GET /api/parties/get/:id
// =============================================================================

#[tokio::test]
async fn get_party_success() {
    let pool = get_test_pool().await;
    let app = app(pool);

    // Create first
    let name = unique_name("GetTest");
    let (_, create_body) =
        post_json(&app, "/api/parties/create", &minimal_party()(&name)).await;
    let id = create_body["data"]["id"].as_str().unwrap();

    // Get
    let (status, body) = get_json(&app, &format!("/api/parties/get/{}", id)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["displayName"], name);
}

#[tokio::test]
async fn get_party_not_found() {
    let pool = get_test_pool().await;
    let app = app(pool);

    let (status, _) = get_json(
        &app,
        &format!("/api/parties/get/{}", uuid::Uuid::now_v7()),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

// =============================================================================
// GET /api/parties/list
// =============================================================================

#[tokio::test]
async fn list_parties_returns_data() {
    let pool = get_test_pool().await;
    let app = app(pool);

    // Create one
    post_json(
        &app,
        "/api/parties/create",
        &minimal_party()(&unique_name("ListTest")),
    )
    .await;

    let (status, body) = get_json(&app, "/api/parties/list").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
    // Pagination is under meta.pagination
    assert!(body["meta"]["pagination"].is_object());
}

#[tokio::test]
async fn list_parties_pagination() {
    let pool = get_test_pool().await;
    let app = app(pool);

    let (status, body) = get_json(&app, "/api/parties/list?page=1&page-size=5").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
    assert!(body["data"].as_array().unwrap().len() <= 5);
    assert!(body["meta"]["pagination"]["page"].is_number());
}

// =============================================================================
// Error Cases
// =============================================================================

#[tokio::test]
async fn create_party_fails_with_display_name_too_long() {
    let pool = get_test_pool().await;
    let app = app(pool);

    let long_name = "a".repeat(256);
    let (status, _) = post_json(&app, "/api/parties/create", &minimal_party()(&long_name)).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn get_party_fails_with_invalid_uuid() {
    let pool = get_test_pool().await;
    let app = app(pool);

    let (status, _) = get_json(&app, "/api/parties/get/not-a-uuid").await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_person_party() {
    let pool = get_test_pool().await;
    let app = app(pool);

    let payload = json!({
        "partyType": "person",
        "displayName": unique_name("PersonTest")
    });

    let (status, body) = post_json(&app, "/api/parties/create", &payload).await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(body["data"]["id"].is_string());
}
