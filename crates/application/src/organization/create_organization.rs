use crate::ports::OrganizationRepository;
use domain::organization::Organization;
use domain::organization::value_objects::{Email, OrganizationName, Phone, Url};
use serde_json::Value as JsonValue;
use shared::AppError;
use uuid::Uuid;

pub struct CreateOrganizationUseCase<R> {
    repository: R,
}

pub struct CreateOrganizationInput {
    pub code: String,
    pub name: String,
    pub display_name: String,
    pub tax_number: String,
    pub registration_no: String,
    pub phone: String,
    pub email: String,
    pub website: String,
    pub parent_id: Option<Uuid>,
    pub metadata: Option<JsonValue>,
}

impl<R: OrganizationRepository> CreateOrganizationUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute<'a, E>(
        &self,
        executor: E,
        input: CreateOrganizationInput,
    ) -> Result<Organization, AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        // Validate and create value objects
        let name = OrganizationName::new(input.name)?;

        // Convert empty strings to None, and validate if not empty
        let email = if input.email.trim().is_empty() {
            None
        } else {
            Some(Email::new(input.email)?)
        };

        let phone = if input.phone.trim().is_empty() {
            None
        } else {
            Some(Phone::new(input.phone)?)
        };

        let website = if input.website.trim().is_empty() {
            None
        } else {
            Some(Url::new(input.website)?)
        };

        // Helper function to convert empty strings to None
        let to_option = |s: String| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        };

        // Create organization entity
        let org = Organization::new(name);

        // Apply optional fields through reconstruction
        let organization = Organization::from_storage(
            org.id(),
            to_option(input.code),
            org.name().clone(),
            to_option(input.display_name),
            to_option(input.tax_number),
            to_option(input.registration_no),
            phone,
            email,
            website,
            input.parent_id,
            input
                .metadata
                .unwrap_or_else(|| JsonValue::Object(serde_json::Map::new())),
            org.created_at(),
            org.updated_at(),
        );

        // Persist to database
        self.repository.create(executor, &organization).await?;

        Ok(organization)
    }
}
