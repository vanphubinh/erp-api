use std::{env, fs, sync::Arc};

use http_server::{app_state::AppState, config::Config, routes};
use sqlx::postgres::PgPoolOptions;
use tower_http::{LatencyUnit, cors::CorsLayer, trace::TraceLayer};
use tracing::{Level, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

#[derive(OpenApi)]
#[openapi(info(
    title = "Van Phu Binh API",
    version = "1.0.0",
    description = "API for managing Van Phu Binh Internal System"
))]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let config = Config::new()?;
    info!("Starting VPB ERP Backend...");

    // Initialize database pool with migrations
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await?;

    sqlx::migrate!("../../migrations").run(&pool).await?;
    info!("✅ Database migrations completed");

    let app_state = Arc::new(AppState { pool });

    // Build application with routes and OpenAPI docs
    let (app, openapi) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(routes::api_routes())
        .with_state(app_state)
        .split_for_parts();

    // Generate OpenAPI JSON in development
    if matches!(
        env::var("RUST_ENV").as_deref(),
        Ok("development" | "dev") | Err(_)
    ) {
        generate_openapi_json(&openapi)?;
    }

    // Configure middleware
    let app = app
        .merge(Scalar::with_url("/docs", openapi))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .on_request(tower_http::trace::DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    tower_http::trace::DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        );

    let listener = tokio::net::TcpListener::bind(config.addr).await?;
    info!("🚀 Listening on http://{}", config.addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c");
    info!("Received shutdown signal");
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "http_server=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn generate_openapi_json(api: &utoipa::openapi::OpenApi) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = if fs::metadata("./apps/http-server").is_ok() {
        "./apps/http-server/openapi.json"
    } else {
        "./openapi.json"
    };

    let openapi_json = serde_json::to_string_pretty(api)?;
    fs::write(output_path, &openapi_json)?;

    info!("✅ Generated OpenAPI spec: {}", output_path);
    info!("📄 {} bytes written", openapi_json.len());
    Ok(())
}
