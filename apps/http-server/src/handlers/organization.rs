use crate::app_state::AppState;
use application::organization::ListOrganizationsUseCase;
use axum::{Json, extract::Query, extract::State, response::IntoResponse};
use domain::organization::Organization;
use infrastructure::repositories::OrganizationRepositoryImpl;
use shared::{AppError, PageParams, SuccessResponse, success_with_pagination};
use std::sync::Arc;

/// List organizations with pagination
#[utoipa::path(
    get,
    path = "/list",
    params(PageParams),
    responses(
        (status = 200, description = "Successfully retrieved organizations", body = inline(SuccessResponse<Vec<Organization>>)),
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

    Ok(Json(success_with_pagination(organizations, pagination)))
}
