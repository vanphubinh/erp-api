pub mod fixtures;

use sqlx::postgres::PgPoolOptions;

// Re-export all repository implementations for easy access in tests
pub use infrastructure::repositories::OrganizationRepositoryImpl;

/// Get shared test database pool
pub async fn get_test_pool() -> sqlx::PgPool {
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
