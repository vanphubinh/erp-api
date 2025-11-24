use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::response::{ErrorResponse, FieldError};

/// Error codes for programmatic error handling by clients
pub mod error_codes {
    pub const DOMAIN_ERROR: &str = "domain_error";
    pub const INVALID_VALUE: &str = "invalid_value";
    pub const BUSINESS_RULE_VIOLATION: &str = "business_rule_violation";
    pub const ENTITY_NOT_FOUND: &str = "entity_not_found";
    pub const DUPLICATE_ENTITY: &str = "duplicate_entity";
    pub const DATABASE_ERROR: &str = "database_error";
    pub const NOT_FOUND: &str = "not_found";
    pub const VALIDATION_ERROR: &str = "validation_error";
    pub const UNAUTHORIZED: &str = "unauthorized";
    pub const FORBIDDEN: &str = "forbidden";
    pub const INTERNAL_ERROR: &str = "internal_error";
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error")]
    Validation(ValidationError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),

    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("Duplicate entity: {0}")]
    DuplicateEntity(String),
}

/// Structured validation error with field-level details
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub fields: Vec<FieldError>,
}

impl ValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            fields: Vec::new(),
        }
    }

    pub fn with_field(mut self, field: impl Into<String>, message: impl Into<String>) -> Self {
        self.fields.push(FieldError {
            field: field.into(),
            message: message.into(),
        });
        self
    }

    pub fn add_field(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.fields.push(FieldError {
            field: field.into(),
            message: message.into(),
        });
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}

impl AppError {
    /// Helper to create ErrorResponse with common fields
    fn create_error_response(
        error_code: &str,
        title: impl Into<String>,
        status: StatusCode,
        detail: impl Into<String>,
    ) -> ErrorResponse {
        ErrorResponse {
            error_type: format!("urn:error:{}", error_code),
            title: title.into(),
            status: status.as_u16(),
            detail: detail.into(),
            instance: None,
            errors: None,
        }
    }

    /// Convert AppError to ErrorResponse with proper RFC 7807 structure
    fn to_error_response(&self) -> ErrorResponse {
        match self {
            AppError::Domain(domain_err) => match domain_err {
                DomainError::InvalidValue(msg) => Self::create_error_response(
                    error_codes::INVALID_VALUE,
                    "Invalid Value",
                    StatusCode::BAD_REQUEST,
                    msg,
                ),
                DomainError::BusinessRuleViolation(msg) => Self::create_error_response(
                    error_codes::BUSINESS_RULE_VIOLATION,
                    "Business Rule Violation",
                    StatusCode::UNPROCESSABLE_ENTITY,
                    msg,
                ),
                DomainError::EntityNotFound(msg) => Self::create_error_response(
                    error_codes::ENTITY_NOT_FOUND,
                    "Entity Not Found",
                    StatusCode::NOT_FOUND,
                    msg,
                ),
                DomainError::DuplicateEntity(msg) => Self::create_error_response(
                    error_codes::DUPLICATE_ENTITY,
                    "Duplicate Entity",
                    StatusCode::CONFLICT,
                    msg,
                ),
            },
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                Self::create_error_response(
                    error_codes::DATABASE_ERROR,
                    "Database Error",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred. Please try again later.",
                )
            }
            AppError::NotFound(msg) => Self::create_error_response(
                error_codes::NOT_FOUND,
                "Not Found",
                StatusCode::NOT_FOUND,
                msg,
            ),
            AppError::Validation(validation_err) => {
                let mut response = Self::create_error_response(
                    error_codes::VALIDATION_ERROR,
                    "Validation Error",
                    StatusCode::BAD_REQUEST,
                    &validation_err.message,
                );
                if !validation_err.fields.is_empty() {
                    response.errors = Some(validation_err.fields.clone());
                }
                response
            }
            AppError::Unauthorized => Self::create_error_response(
                error_codes::UNAUTHORIZED,
                "Unauthorized",
                StatusCode::UNAUTHORIZED,
                "Authentication is required to access this resource.",
            ),
            AppError::Forbidden(msg) => Self::create_error_response(
                error_codes::FORBIDDEN,
                "Forbidden",
                StatusCode::FORBIDDEN,
                msg,
            ),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                Self::create_error_response(
                    error_codes::INTERNAL_ERROR,
                    "Internal Server Error",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unexpected error occurred. Please try again later.",
                )
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.to_error_response().into_response()
    }
}
