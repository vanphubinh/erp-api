use crate::ports::PartyRepository;
use domain::party::value_objects::{DisplayName, LegalName, PartyType, RegistrationNumber, Tin};
use domain::party::Party;
use shared::AppError;

pub struct CreatePartyUseCase<R> {
    repository: R,
}

pub struct CreatePartyInput {
    pub party_type: String,
    pub display_name: String,
    pub legal_name: String,
    pub tin: String,
    pub registration_number: String,
}

impl<R: PartyRepository> CreatePartyUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute<'a, E>(
        &self,
        executor: E,
        input: CreatePartyInput,
    ) -> Result<Party, AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        // Validate and create value objects
        let party_type = PartyType::from_str(&input.party_type)?;
        let display_name = DisplayName::new(input.display_name)?;

        // Convert empty strings to None, and validate if not empty
        let legal_name = if input.legal_name.trim().is_empty() {
            None
        } else {
            Some(LegalName::new(input.legal_name)?)
        };

        let tin = if input.tin.trim().is_empty() {
            None
        } else {
            Some(Tin::new(input.tin)?)
        };

        let registration_number = if input.registration_number.trim().is_empty() {
            None
        } else {
            Some(RegistrationNumber::new(input.registration_number)?)
        };

        // Create party entity
        let base_party = Party::new(party_type, display_name.clone());

        // Apply optional fields through reconstruction
        let party = Party::from_storage(
            base_party.id(),
            party_type,
            display_name,
            legal_name,
            tin,
            registration_number,
            true, // is_active default
            base_party.created_at(),
            base_party.updated_at(),
        );

        // Persist to database
        self.repository.create(executor, &party).await?;

        Ok(party)
    }
}
