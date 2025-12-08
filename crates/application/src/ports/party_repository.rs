use async_trait::async_trait;
use domain::party::Party;
use shared::{AppError, PaginationMeta};
use uuid::Uuid;

/// Port (interface) for party persistence
#[async_trait]
pub trait PartyRepository: Send + Sync {
    /// Create a new party (INSERT)
    async fn create<'a, E>(&self, executor: E, party: &Party) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send;

    /// Update existing party
    async fn update<'a, E>(&self, executor: E, party: &Party) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send;

    /// Find party by ID
    async fn find_by_id<'a, E>(
        &self,
        executor: E,
        id: Uuid,
    ) -> Result<Option<Party>, AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send;

    /// Find parties with offset-based pagination
    /// Returns (items, pagination_meta)
    async fn find_paginated<'a, E>(
        &self,
        executor: E,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<Party>, PaginationMeta), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send;

    /// Delete party by ID
    async fn delete<'a, E>(&self, executor: E, id: Uuid) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send;
}
