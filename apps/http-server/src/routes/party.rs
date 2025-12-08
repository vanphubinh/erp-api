use crate::app_state::AppState;
use crate::handlers::party;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

/// Hybrid REST verbs + RPC-style action paths
/// Uses proper HTTP verbs (GET, POST, PUT, DELETE) with action-based paths
///
/// GET    /api/parties/list          - List all parties
/// GET    /api/parties/get/:id       - Get party by ID  
/// POST   /api/parties/create        - Create new party
/// PUT    /api/parties/update/:id    - Update party
/// DELETE /api/parties/delete/:id    - Delete party
/// PUT    /api/parties/activate/:id  - Activate party
/// PUT    /api/parties/deactivate/:id - Deactivate party
pub fn routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(party::list_parties))
        .routes(routes!(party::get_party))
        .routes(routes!(party::create_party))
    // .routes(routes!(party::update_party))
    // .routes(routes!(party::delete_party))
    // .routes(routes!(party::activate_party))
    // .routes(routes!(party::deactivate_party))
}
