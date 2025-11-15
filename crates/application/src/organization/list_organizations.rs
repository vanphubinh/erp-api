use crate::ports::OrganizationRepository;
use domain::organization::Organization;
use sea_orm::ConnectionTrait;
use shared::{AppError, PaginationMeta};

/// Use case: List organizations with pagination
pub struct ListOrganizationsUseCase<R: OrganizationRepository> {
    repository: R,
}

impl<R: OrganizationRepository> ListOrganizationsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Get paginated list of organizations
    /// Returns (items, pagination_meta)
    pub async fn execute<C>(
        &self,
        conn: &C,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<Organization>, PaginationMeta), AppError>
    where
        C: ConnectionTrait,
    {
        self.repository.find_paginated(conn, page, page_size).await
    }
}
