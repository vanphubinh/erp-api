use super::value_objects::{Email, OrganizationName, Phone, Url};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Organization aggregate root - pure domain model
#[derive(Debug, Clone, PartialEq, ToSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    id: Uuid,

    #[schema(example = "Acme Corporation")]
    name: OrganizationName,

    #[schema(example = "contact@acme.com")]
    email: Option<Email>,

    #[schema(example = "+1-555-0100")]
    phone: Option<Phone>,

    #[schema(example = "https://acme.com")]
    website: Option<Url>,

    #[schema(example = "Technology")]
    industry: Option<String>,

    #[schema(example = "123 Main Street")]
    address: Option<String>,

    #[schema(example = "San Francisco")]
    city: Option<String>,

    #[schema(example = "CA")]
    state: Option<String>,

    #[schema(example = "94105")]
    postal_code: Option<String>,

    #[schema(example = "US")]
    country_code: Option<String>,

    #[schema(example = "America/Los_Angeles")]
    timezone: Option<String>,

    #[schema(example = "USD")]
    currency: Option<String>,

    #[schema(example = true)]
    is_active: bool,

    #[schema(example = "2025-01-15T10:30:00Z")]
    created_at: DateTime<Utc>,

    #[schema(example = "2025-01-15T15:45:00Z")]
    updated_at: DateTime<Utc>,
}

impl Organization {
    /// Create a new organization (minimal fields) - stores in UTC
    pub fn new(name: OrganizationName) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            name,
            email: None,
            phone: None,
            website: None,
            industry: None,
            address: None,
            city: None,
            state: None,
            postal_code: None,
            country_code: None,
            timezone: None,
            currency: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Reconstitute from storage (used by repository)
    #[allow(clippy::too_many_arguments)]
    pub fn from_storage(
        id: Uuid,
        name: OrganizationName,
        email: Option<Email>,
        phone: Option<Phone>,
        website: Option<Url>,
        industry: Option<String>,
        address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        postal_code: Option<String>,
        country_code: Option<String>,
        timezone: Option<String>,
        currency: Option<String>,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            email,
            phone,
            website,
            industry,
            address,
            city,
            state,
            postal_code,
            country_code,
            timezone,
            currency,
            is_active,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &OrganizationName {
        &self.name
    }

    pub fn email(&self) -> Option<&Email> {
        self.email.as_ref()
    }

    pub fn phone(&self) -> Option<&Phone> {
        self.phone.as_ref()
    }

    pub fn website(&self) -> Option<&Url> {
        self.website.as_ref()
    }

    pub fn industry(&self) -> Option<&str> {
        self.industry.as_deref()
    }

    pub fn address(&self) -> Option<&str> {
        self.address.as_deref()
    }

    pub fn city(&self) -> Option<&str> {
        self.city.as_deref()
    }

    pub fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }

    pub fn postal_code(&self) -> Option<&str> {
        self.postal_code.as_deref()
    }

    pub fn country_code(&self) -> Option<&str> {
        self.country_code.as_deref()
    }

    pub fn timezone(&self) -> Option<&str> {
        self.timezone.as_deref()
    }

    pub fn currency(&self) -> Option<&str> {
        self.currency.as_deref()
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
    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn update_name(&mut self, name: OrganizationName) {
        self.name = name;
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_org(name: &str) -> Organization {
        Organization::new(OrganizationName::new(name).unwrap())
    }

    #[test]
    fn new_organization_has_defaults() {
        let org = create_org("Test Corp");

        assert_eq!(org.name().value(), "Test Corp");
        assert!(org.is_active());
        assert!(org.email().is_none());
        assert!(org.phone().is_none());
        assert!(org.website().is_none());
    }

    #[test]
    fn new_organization_has_uuid_v7() {
        let org = create_org("Test Corp");
        // UUID v7 starts with timestamp, so it should be non-zero
        assert!(!org.id().is_nil());
    }

    #[test]
    fn new_organization_has_timestamps() {
        let before = Utc::now();
        let org = create_org("Test Corp");
        let after = Utc::now();

        assert!(org.created_at() >= before);
        assert!(org.created_at() <= after);
        assert_eq!(org.created_at(), org.updated_at());
    }

    #[test]
    fn activate_sets_active_and_updates_timestamp() {
        let mut org = create_org("Test Corp");
        org.deactivate();
        let before_update = org.updated_at();

        std::thread::sleep(std::time::Duration::from_millis(1));
        org.activate();

        assert!(org.is_active());
        assert!(org.updated_at() > before_update);
    }

    #[test]
    fn deactivate_sets_inactive_and_updates_timestamp() {
        let mut org = create_org("Test Corp");
        let before_update = org.updated_at();

        std::thread::sleep(std::time::Duration::from_millis(1));
        org.deactivate();

        assert!(!org.is_active());
        assert!(org.updated_at() > before_update);
    }

    #[test]
    fn update_name_changes_name_and_timestamp() {
        let mut org = create_org("Old Name");
        let before_update = org.updated_at();

        std::thread::sleep(std::time::Duration::from_millis(1));
        org.update_name(OrganizationName::new("New Name").unwrap());

        assert_eq!(org.name().value(), "New Name");
        assert!(org.updated_at() > before_update);
    }
}
