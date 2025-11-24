use crate::app_state::AppState;
use application::organization::ListOrganizationsUseCase;
use axum::{Json, extract::Query, extract::State, response::IntoResponse};
use infrastructure::repositories::OrganizationRepositoryImpl;
use serde::Serialize;
use shared::{AppError, PageParams, success_with_pagination};
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    #[schema(example = "Acme Corporation")]
    pub name: String,
    #[schema(example = "contact@acme.com")]
    pub email: Option<String>,
    #[schema(example = "+1-555-0100")]
    pub phone: Option<String>,
    #[schema(example = "https://acme.com")]
    pub website: Option<String>,
    #[schema(example = "Technology")]
    pub industry: Option<String>,
    #[schema(example = true)]
    pub is_active: bool,
    #[schema(example = "2025-01-15T10:30:00+00:00")]
    pub created_at: String,
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
#[utoipa::path(
    get,
    path = "/list",
    params(PageParams),
    responses(
        (status = 200, description = "Successfully retrieved organizations"),
        (status = 400, description = "Invalid pagination parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Organizations"
)]
pub async fn list_organizations(
    Query(params): Query<PageParams>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let params = params.validate(100);

    let (organizations, pagination) =
        ListOrganizationsUseCase::new(OrganizationRepositoryImpl::new())
            .execute(&app_state.pool, params.page, params.page_size)
            .await?;

    let response: Vec<OrganizationResponse> = organizations.into_iter().map(Into::into).collect();

    Ok(Json(success_with_pagination(response, pagination)))
}
