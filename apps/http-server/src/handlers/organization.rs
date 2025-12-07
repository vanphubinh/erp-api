use crate::app_state::AppState;
use crate::dto::{CreateOrganizationRequest, CreateOrganizationResponse};
use application::organization::{
    CreateOrganizationUseCase, GetOrganizationUseCase, ListOrganizationsUseCase,
};
use axum::{Json, extract::Path, extract::Query, extract::State, response::IntoResponse};
use domain::organization::Organization;
use infrastructure::repositories::OrganizationRepositoryImpl;
use shared::{AppError, PageParams, SuccessResponse, created, success, success_with_pagination};
use std::sync::Arc;
use uuid::Uuid;

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

/// Create a new organization
#[utoipa::path(
    post,
    path = "/create",
    request_body(
        content = CreateOrganizationRequest,
        description = "Organization data to create",
        content_type = "application/json"
    ),
    responses(
        (
            status = 201,
            description = "Organization created successfully",
            body = inline(SuccessResponse<CreateOrganizationResponse>)
        ),
        (
            status = 400,
            description = "Invalid request data - validation failed",
            body = inline(shared::ErrorResponse)
        ),
        (
            status = 422,
            description = "Business rule violation",
            body = inline(shared::ErrorResponse)
        ),
        (
            status = 500,
            description = "Internal server error",
            body = inline(shared::ErrorResponse)
        )
    ),
    tag = "Organizations"
)]
pub async fn create_organization(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<CreateOrganizationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let input = application::organization::CreateOrganizationInput {
        code: request.code,
        name: request.name,
        display_name: request.display_name,
        tax_number: request.tax_number,
        registration_no: request.registration_no,
        phone: request.phone,
        email: request.email,
        website: request.website,
        parent_id: request.parent_id,
    };

    let organization = CreateOrganizationUseCase::new(OrganizationRepositoryImpl::new())
        .execute(&app_state.pool, input)
        .await?;

    Ok(created(CreateOrganizationResponse {
        id: organization.id(),
    }))
}

/// Get a single organization by ID
#[utoipa::path(
    get,
    path = "/get/{id}",
    params(
        ("id" = Uuid, Path, description = "Organization unique identifier")
    ),
    responses(
        (
            status = 200,
            description = "Successfully retrieved organization",
            body = inline(SuccessResponse<Organization>)
        ),
        (
            status = 404,
            description = "Organization not found",
            body = inline(shared::ErrorResponse)
        ),
        (
            status = 400,
            description = "Invalid UUID format",
            body = inline(shared::ErrorResponse)
        ),
        (
            status = 500,
            description = "Internal server error",
            body = inline(shared::ErrorResponse)
        )
    ),
    tag = "Organizations"
)]
pub async fn get_organization(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let organization = GetOrganizationUseCase::new(OrganizationRepositoryImpl::new())
        .execute(&app_state.pool, id)
        .await?;

    Ok(Json(success(organization)))
}
