use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Request to create a new organization
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationRequest {
    /// Organization name (required, 2-255 characters)
    #[schema(example = "Acme Corporation", min_length = 2, max_length = 255)]
    pub name: String,

    /// Primary email address (optional, empty string if not provided)
    #[schema(example = "contact@acme.com", min_length = 0)]
    pub email: String,

    /// Primary phone number (optional, empty string if not provided)
    #[schema(example = "+1-555-0100", min_length = 0)]
    pub phone: String,

    /// Company website URL (optional, must start with http:// or https:// if provided)
    #[schema(example = "https://acme.com", min_length = 0)]
    pub website: String,

    /// Industry type (optional, empty string if not provided)
    #[schema(example = "Technology", min_length = 0)]
    pub industry: String,

    /// Street address (optional, empty string if not provided)
    #[schema(example = "123 Main Street", min_length = 0)]
    pub address: String,

    /// City (optional, empty string if not provided)
    #[schema(example = "San Francisco", min_length = 0)]
    pub city: String,

    /// State or Province (optional, empty string if not provided)
    #[schema(example = "CA", min_length = 0)]
    pub state: String,

    /// Postal or ZIP code (optional, empty string if not provided)
    #[schema(example = "94105", min_length = 0)]
    pub postal_code: String,

    /// ISO 3166-1 alpha-2 country code (optional, e.g., US, GB, TH)
    #[schema(example = "US", min_length = 0)]
    pub country_code: String,

    /// IANA timezone identifier (optional, e.g., America/New_York, Asia/Bangkok)
    #[schema(example = "America/Los_Angeles", min_length = 0)]
    pub timezone: String,

    /// ISO 4217 currency code (optional, e.g., USD, EUR, THB)
    #[schema(example = "USD", min_length = 0)]
    pub currency: String,
}

/// Response after successfully creating an organization
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationResponse {
    /// The ID of the newly created organization
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
}
