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

    #[schema(example = "ORG-001")]
    code: Option<String>,

    #[schema(example = "Acme Corporation")]
    name: OrganizationName,

    #[schema(example = "Acme Corp")]
    display_name: Option<String>,

    #[schema(example = "0123456789")]
    tax_number: Option<String>,

    #[schema(example = "BRN-12345")]
    registration_no: Option<String>,

    #[schema(example = "+1-555-0100")]
    phone: Option<Phone>,

    #[schema(example = "contact@acme.com")]
    email: Option<Email>,

    #[schema(example = "https://acme.com")]
    website: Option<Url>,

    #[schema(example = "550e8400-e29b-41d4-a716-446655440001")]
    parent_id: Option<Uuid>,

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
            code: None,
            name,
            display_name: None,
            tax_number: None,
            registration_no: None,
            phone: None,
            email: None,
            website: None,
            parent_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Reconstitute from storage (used by repository)
    #[allow(clippy::too_many_arguments)]
    pub fn from_storage(
        id: Uuid,
        code: Option<String>,
        name: OrganizationName,
        display_name: Option<String>,
        tax_number: Option<String>,
        registration_no: Option<String>,
        phone: Option<Phone>,
        email: Option<Email>,
        website: Option<Url>,
        parent_id: Option<Uuid>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            code,
            name,
            display_name,
            tax_number,
            registration_no,
            phone,
            email,
            website,
            parent_id,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    pub fn name(&self) -> &OrganizationName {
        &self.name
    }

    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    pub fn tax_number(&self) -> Option<&str> {
        self.tax_number.as_deref()
    }

    pub fn registration_no(&self) -> Option<&str> {
        self.registration_no.as_deref()
    }

    pub fn phone(&self) -> Option<&Phone> {
        self.phone.as_ref()
    }

    pub fn email(&self) -> Option<&Email> {
        self.email.as_ref()
    }

    pub fn website(&self) -> Option<&Url> {
        self.website.as_ref()
    }

    pub fn parent_id(&self) -> Option<Uuid> {
        self.parent_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // Business logic methods
    pub fn update_name(&mut self, name: OrganizationName) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    pub fn set_parent(&mut self, parent_id: Option<Uuid>) {
        self.parent_id = parent_id;
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
        assert!(org.code().is_none());
        assert!(org.display_name().is_none());
        assert!(org.email().is_none());
        assert!(org.phone().is_none());
        assert!(org.website().is_none());
        assert!(org.parent_id().is_none());
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
    fn update_name_changes_name_and_timestamp() {
        let mut org = create_org("Old Name");
        let before_update = org.updated_at();

        std::thread::sleep(std::time::Duration::from_millis(1));
        org.update_name(OrganizationName::new("New Name").unwrap());

        assert_eq!(org.name().value(), "New Name");
        assert!(org.updated_at() > before_update);
    }

    #[test]
    fn set_parent_updates_parent_id() {
        let mut org = create_org("Child Corp");
        let parent_id = Uuid::now_v7();

        org.set_parent(Some(parent_id));

        assert_eq!(org.parent_id(), Some(parent_id));
    }
}
