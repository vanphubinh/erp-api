use async_trait::async_trait;
use domain::organization::Organization;
use shared::{AppError, PaginationMeta};
use uuid::Uuid;

/// Port (interface) for organization persistence
#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    /// Create a new organization (INSERT)
    async fn create<'a, E>(&self, executor: E, organization: &Organization) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send;

    /// Update existing organization
    async fn update<'a, E>(&self, executor: E, organization: &Organization) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send;

    /// Find organization by ID
    async fn find_by_id<'a, E>(
        &self,
        executor: E,
        id: Uuid,
    ) -> Result<Option<Organization>, AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send;

    /// Find organizations with offset-based pagination
    /// Returns (items, pagination_meta)
    async fn find_paginated<'a, E>(
        &self,
        executor: E,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<Organization>, PaginationMeta), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send;

    /// Delete organization by ID
    async fn delete<'a, E>(&self, executor: E, id: Uuid) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send;
}
