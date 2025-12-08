use crate::ports::PartyRepository;
use domain::party::Party;
use shared::{AppError, PaginationMeta};

pub struct ListPartiesUseCase<R> {
    repository: R,
}

impl<R: PartyRepository> ListPartiesUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute<'a, E>(
        &self,
        executor: E,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<Party>, PaginationMeta), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        self.repository
            .find_paginated(executor, page, page_size)
            .await
    }
}
