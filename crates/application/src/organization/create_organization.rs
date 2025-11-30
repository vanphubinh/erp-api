use crate::ports::OrganizationRepository;
use domain::organization::Organization;
use domain::organization::value_objects::{Email, OrganizationName, Phone, Url};
use shared::AppError;

pub struct CreateOrganizationUseCase<R> {
    repository: R,
}

pub struct CreateOrganizationInput {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub website: String,
    pub industry: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country_code: String,
    pub timezone: String,
    pub currency: String,
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
        let organization = Organization::new(name);

        // Apply optional fields through reconstruction
        let organization = Organization::from_storage(
            organization.id(),
            organization.name().clone(),
            email,
            phone,
            website,
            to_option(input.industry),
            to_option(input.address),
            to_option(input.city),
            to_option(input.state),
            to_option(input.postal_code),
            to_option(input.country_code),
            to_option(input.timezone),
            to_option(input.currency),
            true, // is_active
            organization.created_at(),
            organization.updated_at(),
        );

        // Persist to database
        self.repository.create(executor, &organization).await?;

        Ok(organization)
    }
}
