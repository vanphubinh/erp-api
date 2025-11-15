use crate::app_state::AppState;
use application::organization::ListOrganizationsUseCase;
use axum::{Json, extract::Query, extract::State, response::IntoResponse};
use infrastructure::repositories::OrganizationRepositoryImpl;
use serde::Serialize;
use shared::{AppError, PageParams, SuccessResponse, success_with_pagination};
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationResponse {
    /// Organization unique identifier
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,

    /// Organization name
    #[schema(example = "Acme Corporation")]
    pub name: String,

    /// Primary email address
    #[schema(example = "contact@acme.com")]
    pub email: Option<String>,

    /// Phone number
    #[schema(example = "+1-555-0100")]
    pub phone: Option<String>,

    /// Website URL
    #[schema(example = "https://acme.com")]
    pub website: Option<String>,

    /// Industry type
    #[schema(example = "Technology")]
    pub industry: Option<String>,

    /// Whether organization is active
    #[schema(example = true)]
    pub is_active: bool,

    /// Creation timestamp (RFC 3339 / ISO 8601)
    #[schema(example = "2025-01-15T10:30:00+00:00")]
    pub created_at: String,

    /// Last update timestamp (RFC 3339 / ISO 8601)
    #[schema(example = "2025-01-15T15:45:00+00:00")]
    pub updated_at: String,
}

impl From<domain::organization::Organization> for OrganizationResponse {
    fn from(org: domain::organization::Organization) -> Self {
        Self {
            id: org.id().to_string(),
            name: org.name().value().to_string(),
            email: org.email().map(|e| e.to_string()),
            phone: org.phone().map(|p| p.to_string()),
            website: org.website().map(|w| w.to_string()),
            industry: org.industry().map(|s| s.to_string()),
            is_active: org.is_active(),
            created_at: org.created_at().to_rfc3339(),
            updated_at: org.updated_at().to_rfc3339(),
        }
    }
}

/// List organizations with pagination
///
/// Returns a paginated list of organizations with metadata including:
/// - Total count of organizations
/// - Current page number
/// - Page size
/// - Total pages
/// - Has next/previous page flags
#[utoipa::path(
    get,
    path = "/list",
    params(PageParams),
    responses(
        (status = 200, description = "Successfully retrieved organizations with pagination", 
         body = inline(SuccessResponse<Vec<OrganizationResponse>>),
        ),
        (status = 400, description = "Bad request - invalid pagination parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Organizations"
)]
pub async fn list_organizations(
    Query(params): Query<PageParams>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    // Validate pagination params
    let params = params.validate(100);

    // Initialize repository and use case
    let repository = OrganizationRepositoryImpl::new();
    let use_case = ListOrganizationsUseCase::new(repository);

    // Execute use case
    let (organizations, pagination) = use_case
        .execute(&app_state.connection, params.page, params.page_size)
        .await?;

    // Map domain models to response DTOs using From trait
    let response: Vec<OrganizationResponse> = organizations
        .into_iter()
        .map(OrganizationResponse::from)
        .collect();

    // Return with pagination metadata
    Ok(Json(success_with_pagination(response, pagination)))
}
