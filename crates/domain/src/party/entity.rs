use super::value_objects::{DisplayName, LegalName, PartyType, RegistrationNumber, Tin};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Party aggregate root - unified entity for companies and persons
#[derive(Debug, Clone, PartialEq, ToSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    id: Uuid,

    #[schema(example = "company")]
    party_type: PartyType,

    #[schema(example = "Acme Corp")]
    display_name: DisplayName,

    #[schema(example = "Acme Corporation Ltd.")]
    legal_name: Option<LegalName>,

    #[schema(example = "0123456789")]
    tin: Option<Tin>,

    #[schema(example = "BRN-12345")]
    registration_number: Option<RegistrationNumber>,

    #[schema(example = true)]
    is_active: bool,

    #[schema(example = "2025-01-15T10:30:00Z")]
    created_at: DateTime<Utc>,

    #[schema(example = "2025-01-15T15:45:00Z")]
    updated_at: DateTime<Utc>,
}

impl Party {
    /// Create a new party with minimal required fields
    pub fn new(party_type: PartyType, display_name: DisplayName) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            party_type,
            display_name,
            legal_name: None,
            tin: None,
            registration_number: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Reconstitute from storage (used by repository)
    #[allow(clippy::too_many_arguments)]
    pub fn from_storage(
        id: Uuid,
        party_type: PartyType,
        display_name: DisplayName,
        legal_name: Option<LegalName>,
        tin: Option<Tin>,
        registration_number: Option<RegistrationNumber>,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            party_type,
            display_name,
            legal_name,
            tin,
            registration_number,
            is_active,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn party_type(&self) -> PartyType {
        self.party_type
    }

    pub fn display_name(&self) -> &DisplayName {
        &self.display_name
    }

    pub fn legal_name(&self) -> Option<&LegalName> {
        self.legal_name.as_ref()
    }

    pub fn tin(&self) -> Option<&Tin> {
        self.tin.as_ref()
    }

    pub fn registration_number(&self) -> Option<&RegistrationNumber> {
        self.registration_number.as_ref()
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // Business logic methods
    pub fn update_display_name(&mut self, display_name: DisplayName) {
        self.display_name = display_name;
        self.updated_at = Utc::now();
    }

    pub fn update_legal_name(&mut self, legal_name: Option<LegalName>) {
        self.legal_name = legal_name;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_party(name: &str) -> Party {
        Party::new(PartyType::Company, DisplayName::new(name).unwrap())
    }

    #[test]
    fn new_party_has_defaults() {
        let party = create_party("Test Corp");

        assert_eq!(party.display_name().value(), "Test Corp");
        assert_eq!(party.party_type(), PartyType::Company);
        assert!(party.is_active());
        assert!(party.legal_name().is_none());
        assert!(party.tin().is_none());
        assert!(party.registration_number().is_none());
    }

    #[test]
    fn new_party_has_uuid_v7() {
        let party = create_party("Test Corp");
        // UUID v7 starts with timestamp, so it should be non-zero
        assert!(!party.id().is_nil());
    }

    #[test]
    fn new_party_has_timestamps() {
        let before = Utc::now();
        let party = create_party("Test Corp");
        let after = Utc::now();

        assert!(party.created_at() >= before);
        assert!(party.created_at() <= after);
        assert_eq!(party.created_at(), party.updated_at());
    }

    #[test]
    fn update_display_name_changes_name_and_timestamp() {
        let mut party = create_party("Old Name");
        let before_update = party.updated_at();

        std::thread::sleep(std::time::Duration::from_millis(1));
        party.update_display_name(DisplayName::new("New Name").unwrap());

        assert_eq!(party.display_name().value(), "New Name");
        assert!(party.updated_at() > before_update);
    }

    #[test]
    fn deactivate_sets_is_active_false() {
        let mut party = create_party("Test Corp");
        assert!(party.is_active());

        party.deactivate();

        assert!(!party.is_active());
    }

    #[test]
    fn activate_sets_is_active_true() {
        let mut party = create_party("Test Corp");
        party.deactivate();
        assert!(!party.is_active());

        party.activate();

        assert!(party.is_active());
    }

    #[test]
    fn can_create_person_party() {
        let party = Party::new(PartyType::Person, DisplayName::new("John Doe").unwrap());

        assert_eq!(party.party_type(), PartyType::Person);
        assert_eq!(party.display_name().value(), "John Doe");
    }
}
