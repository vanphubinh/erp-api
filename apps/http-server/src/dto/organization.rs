use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Request to create a new organization
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationRequest {
    /// Unique organization code (optional, e.g., ORG-001)
    #[schema(example = "ORG-001", min_length = 0, max_length = 255, required = true)]
    #[serde(default)]
    pub code: String,

    /// Organization legal name (required, 2-255 characters)
    #[schema(
        example = "Acme Corporation",
        min_length = 2,
        max_length = 255,
        required = true
    )]
    pub name: String,

    /// Display/trading name (optional)
    #[schema(example = "Acme Corp", min_length = 0, required = true)]
    #[serde(default)]
    pub display_name: String,

    /// Tax identification number (optional, MST for Vietnam)
    #[schema(
        example = "0123456789",
        min_length = 0,
        max_length = 255,
        required = true
    )]
    #[serde(default)]
    pub tax_number: String,

    /// Business registration number (optional)
    #[schema(
        example = "BRN-12345",
        min_length = 0,
        max_length = 255,
        required = true
    )]
    #[serde(default)]
    pub registration_no: String,

    /// Primary phone number (optional)
    #[schema(
        example = "+1-555-0100",
        min_length = 0,
        max_length = 20,
        required = true
    )]
    #[serde(default)]
    pub phone: String,

    /// Primary email address (optional)
    #[schema(
        example = "contact@acme.com",
        min_length = 0,
        max_length = 255,
        required = true
    )]
    #[serde(default)]
    pub email: String,

    /// Company website URL (optional, must start with http:// or https://)
    #[schema(
        example = "https://acme.com",
        min_length = 0,
        max_length = 255,
        format = "uri",
        required = true
    )]
    #[serde(default)]
    pub website: String,

    /// Parent organization ID for hierarchy (optional)
    #[schema(
        example = "550e8400-e29b-41d4-a716-446655440001",
        format = "uuid",
        nullable = true,
        required = true
    )]
    pub parent_id: Option<Uuid>,
}

/// Response after successfully creating an organization
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationResponse {
    /// The ID of the newly created organization
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
}
