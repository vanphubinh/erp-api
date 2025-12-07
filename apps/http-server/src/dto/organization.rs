use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use utoipa::ToSchema;
use uuid::Uuid;

/// Request to create a new organization
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationRequest {
    /// Unique organization code (optional, e.g., ORG-001)
    #[schema(example = "ORG-001")]
    #[serde(default)]
    pub code: String,

    /// Organization legal name (required, 2-255 characters)
    #[schema(example = "Acme Corporation", min_length = 2, max_length = 255)]
    pub name: String,

    /// Display/trading name (optional)
    #[schema(example = "Acme Corp")]
    #[serde(default)]
    pub display_name: String,

    /// Tax identification number (optional, MST for Vietnam)
    #[schema(example = "0123456789")]
    #[serde(default)]
    pub tax_number: String,

    /// Business registration number (optional)
    #[schema(example = "BRN-12345")]
    #[serde(default)]
    pub registration_no: String,

    /// Primary phone number (optional)
    #[schema(example = "+1-555-0100")]
    #[serde(default)]
    pub phone: String,

    /// Primary email address (optional)
    #[schema(example = "contact@acme.com")]
    #[serde(default)]
    pub email: String,

    /// Company website URL (optional, must start with http:// or https://)
    #[schema(example = "https://acme.com")]
    #[serde(default)]
    pub website: String,

    /// Parent organization ID for hierarchy (optional)
    #[schema(example = "550e8400-e29b-41d4-a716-446655440001")]
    pub parent_id: Option<Uuid>,

    /// Flexible metadata (optional JSON object)
    #[schema(example = json!({"industry": "Technology", "size": "Large"}))]
    pub metadata: Option<JsonValue>,
}

/// Response after successfully creating an organization
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationResponse {
    /// The ID of the newly created organization
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
}
