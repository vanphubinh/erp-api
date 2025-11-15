use super::value_objects::{Email, OrganizationName, Phone, Url};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Organization aggregate root - pure domain model
#[derive(Debug, Clone, PartialEq)]
pub struct Organization {
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
