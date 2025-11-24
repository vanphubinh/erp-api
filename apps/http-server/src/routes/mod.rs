pub mod organization;

use crate::app_state::AppState;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

/// Create all API routes with OpenAPI documentation
/// Hybrid REST verbs + RPC-style action paths
pub fn api_routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().nest("/api/organizations", organization::routes())
    // Add more resources here
    // .nest("/api/contacts", contact::routes())
    // .nest("/api/invoices", invoice::routes())
    // .nest("/api/products", product::routes())
}
