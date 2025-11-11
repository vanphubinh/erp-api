pub mod error;
pub mod pagination;
pub mod response;

// Re-export commonly used types
pub use error::{AppError, DomainError, ValidationError};
pub use pagination::{PageParams, PaginationMeta};
pub use response::{ErrorResponse, FieldError, Meta, SuccessResponse};

// Re-export helper functions for convenience
pub use response::{accepted, created, no_content, success, success_with_pagination};
