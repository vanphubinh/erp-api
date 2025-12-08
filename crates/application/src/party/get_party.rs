use crate::ports::PartyRepository;
use domain::party::Party;
use shared::AppError;
use uuid::Uuid;

pub struct GetPartyUseCase<R> {
    repository: R,
}

impl<R: PartyRepository> GetPartyUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute<'a, E>(
        &self,
        executor: E,
        id: Uuid,
    ) -> Result<Party, AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        self.repository
            .find_by_id(executor, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Party with ID {} not found", id)))
    }
}
