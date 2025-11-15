use crate::app_state::AppState;
use crate::handlers::organization;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

/// Hybrid REST verbs + RPC-style action paths
/// Uses proper HTTP verbs (GET, POST, PUT, DELETE) with action-based paths
///
/// GET    /api/organizations/list          - List all organizations
/// GET    /api/organizations/get/:id       - Get organization by ID  
/// POST   /api/organizations/create        - Create new organization
/// PUT    /api/organizations/update/:id    - Update organization
/// DELETE /api/organizations/delete/:id    - Delete organization
/// PUT    /api/organizations/activate/:id  - Activate organization
/// PUT    /api/organizations/deactivate/:id - Deactivate organization
pub fn routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(organization::list_organizations))
    // .routes(routes!(organization::get_organization))
    // .routes(routes!(organization::create_organization))
    // .routes(routes!(organization::update_organization))
    // .routes(routes!(organization::delete_organization))
    // .routes(routes!(organization::activate_organization))
    // .routes(routes!(organization::deactivate_organization))
}
