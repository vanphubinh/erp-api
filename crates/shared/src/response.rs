use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

use crate::pagination::PaginationMeta;

/// Success response structure
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse<T> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

/// Error response following RFC 7807 (Problem Details)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    /// URI reference that identifies the problem type
    #[serde(rename = "type")]
    #[schema(example = "https://api.example.com/errors/not_found")]
    pub error_type: String,

    /// Short, human-readable summary
    #[schema(example = "Not Found")]
    pub title: String,

    /// HTTP status code
    #[schema(example = 404)]
    pub status: u16,

    /// Human-readable explanation
    #[schema(example = "The requested resource was not found")]
    pub detail: String,

    /// URI reference that identifies the specific occurrence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// Validation errors for fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FieldError>>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FieldError {
    #[schema(example = "email")]
    pub field: String,
    #[schema(example = "Invalid email format")]
    pub message: String,
}

/// Meta information (can include pagination, timestamps, etc.)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMeta>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "2025-10-02T10:30:00Z")]
    pub timestamp: Option<String>,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data, meta: None }
    }

    pub fn with_meta(mut self, meta: Meta) -> Self {
        self.meta = Some(meta);
        self
    }

    pub fn with_pagination(mut self, pagination: PaginationMeta) -> Self {
        let meta = self.meta.get_or_insert_with(|| Meta {
            pagination: None,
            timestamp: None,
        });
        meta.pagination = Some(pagination);
        self
    }
}

impl<T: Serialize> IntoResponse for SuccessResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

/// Helper functions to create responses
pub fn success<T: Serialize>(data: T) -> SuccessResponse<T> {
    SuccessResponse::new(data)
}

pub fn success_with_pagination<T: Serialize>(
    data: T,
    pagination: PaginationMeta,
) -> SuccessResponse<T> {
    SuccessResponse::new(data).with_pagination(pagination)
}

pub fn created<T: Serialize>(data: T) -> impl IntoResponse {
    (StatusCode::CREATED, Json(SuccessResponse::new(data)))
}

pub fn accepted<T: Serialize>(data: T) -> impl IntoResponse {
    (StatusCode::ACCEPTED, Json(SuccessResponse::new(data)))
}

pub fn no_content() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}
