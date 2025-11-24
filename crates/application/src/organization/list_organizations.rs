use crate::ports::OrganizationRepository;
use domain::organization::Organization;
use shared::{AppError, PaginationMeta};

pub struct ListOrganizationsUseCase<R> {
    repository: R,
}

impl<R: OrganizationRepository> ListOrganizationsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute<'a, E>(
        &self,
        executor: E,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<Organization>, PaginationMeta), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        self.repository
            .find_paginated(executor, page, page_size)
            .await
    }
}
