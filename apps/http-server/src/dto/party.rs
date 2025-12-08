use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Request to create a new party
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePartyRequest {
    /// Party type: 'company' or 'person' (required)
    #[schema(example = "company", required = true)]
    pub party_type: String,

    /// Display/trading name (required, 2-255 characters)
    #[schema(
        example = "Acme Corporation",
        min_length = 2,
        max_length = 255,
        required = true
    )]
    pub display_name: String,

    /// Legal/registered name (optional)
    #[schema(example = "Acme Corporation Ltd.", min_length = 0, max_length = 255)]
    #[serde(default)]
    pub legal_name: String,

    /// Tax identification number (optional, MST for Vietnam)
    #[schema(example = "0123456789", min_length = 0, max_length = 50)]
    #[serde(default)]
    pub tin: String,

    /// Business registration number (optional)
    #[schema(example = "BRN-12345", min_length = 0, max_length = 100)]
    #[serde(default)]
    pub registration_number: String,
}

/// Response after successfully creating a party
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePartyResponse {
    /// The ID of the newly created party
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
}
