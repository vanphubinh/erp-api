use crate::app_state::AppState;
use crate::dto::{CreatePartyRequest, CreatePartyResponse};
use application::party::{
    CreatePartyUseCase, GetPartyUseCase, ListPartiesUseCase,
};
use axum::{Json, extract::Path, extract::Query, extract::State, response::IntoResponse};
use domain::party::Party;
use infrastructure::repositories::PartyRepositoryImpl;
use shared::{AppError, PageParams, SuccessResponse, created, success, success_with_pagination};
use std::sync::Arc;
use uuid::Uuid;

/// List parties with pagination
#[utoipa::path(
    get,
    path = "/list",
    params(PageParams),
    responses(
        (status = 200, description = "Successfully retrieved parties", body = inline(SuccessResponse<Vec<Party>>)),
        (status = 400, description = "Invalid pagination parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Parties"
)]
pub async fn list_parties(
    Query(params): Query<PageParams>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let params = params.validate(100);

    let (parties, pagination) =
        ListPartiesUseCase::new(PartyRepositoryImpl::new())
            .execute(&app_state.pool, params.page, params.page_size)
            .await?;

    Ok(Json(success_with_pagination(parties, pagination)))
}

/// Create a new party
#[utoipa::path(
    post,
    path = "/create",
    request_body(
        content = CreatePartyRequest,
        description = "Party data to create",
        content_type = "application/json"
    ),
    responses(
        (
            status = 201,
            description = "Party created successfully",
            body = inline(SuccessResponse<CreatePartyResponse>)
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
    tag = "Parties"
)]
pub async fn create_party(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<CreatePartyRequest>,
) -> Result<impl IntoResponse, AppError> {
    let input = application::party::CreatePartyInput {
        party_type: request.party_type,
        display_name: request.display_name,
        legal_name: request.legal_name,
        tin: request.tin,
        registration_number: request.registration_number,
    };

    let party = CreatePartyUseCase::new(PartyRepositoryImpl::new())
        .execute(&app_state.pool, input)
        .await?;

    Ok(created(CreatePartyResponse {
        id: party.id(),
    }))
}

/// Get a single party by ID
#[utoipa::path(
    get,
    path = "/get/{id}",
    params(
        ("id" = Uuid, Path, description = "Party unique identifier")
    ),
    responses(
        (
            status = 200,
            description = "Successfully retrieved party",
            body = inline(SuccessResponse<Party>)
        ),
        (
            status = 404,
            description = "Party not found",
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
    tag = "Parties"
)]
pub async fn get_party(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let party = GetPartyUseCase::new(PartyRepositoryImpl::new())
        .execute(&app_state.pool, id)
        .await?;

    Ok(Json(success(party)))
}
