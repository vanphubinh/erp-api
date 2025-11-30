use crate::ports::OrganizationRepository;
use domain::organization::Organization;
use shared::AppError;
use uuid::Uuid;

pub struct GetOrganizationUseCase<R> {
    repository: R,
}

impl<R: OrganizationRepository> GetOrganizationUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute<'a, E>(
        &self,
        executor: E,
        id: Uuid,
    ) -> Result<Organization, AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        self.repository
            .find_by_id(executor, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Organization with ID {} not found", id)))
    }
}

