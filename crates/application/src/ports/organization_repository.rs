use async_trait::async_trait;
use domain::organization::Organization;
use sea_orm::ConnectionTrait;
use shared::{AppError, PaginationMeta};
use uuid::Uuid;

/// Port (interface) for organization persistence
#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    /// Save a new organization
    async fn save<C>(&self, conn: &C, organization: &Organization) -> Result<(), AppError>
    where
        C: ConnectionTrait;

    /// Update existing organization
    async fn update<C>(&self, conn: &C, organization: &Organization) -> Result<(), AppError>
    where
        C: ConnectionTrait;

    /// Find organization by ID
    async fn find_by_id<C>(&self, conn: &C, id: Uuid) -> Result<Option<Organization>, AppError>
    where
        C: ConnectionTrait;

    /// Find organizations with offset-based pagination
    /// Returns (items, pagination_meta)
    async fn find_paginated<C>(
        &self,
        conn: &C,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<Organization>, PaginationMeta), AppError>
    where
        C: ConnectionTrait;

    /// Delete organization by ID
    async fn delete<C>(&self, conn: &C, id: Uuid) -> Result<(), AppError>
    where
        C: ConnectionTrait;
}
