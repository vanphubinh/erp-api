use std::env;
use std::fs;
use std::sync::Arc;

use http_server::app_state;
use http_server::config;
use sea_orm::Database;
use tower_http::trace;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(OpenApi)]
    #[openapi(info(
        title = "Van Phu Binh API",
        version = "1.0.0",
        description = "API for managing Van Phu Binh Internal System"
    ))]
    pub struct ApiDoc;

    let config: config::Config = config::Config::new()?;

    init_tracing();

    info!("Starting VPB ERP Backend...");

    let openapi = ApiDoc::openapi();

    let conn = Database::connect(config.db_url)
        .await
        .expect("Database connection failed");

    let app_state = Arc::new(app_state::AppState { connection: conn });

    let (app, openapi_doc) = OpenApiRouter::with_openapi(openapi.clone())
        .with_state(app_state)
        .split_for_parts();

    let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
    if env == "development" || env == "dev" {
        generate_openapi_json(&openapi_doc)?;
    }

    let app = app
        .merge(Scalar::with_url("/docs", openapi_doc.clone()))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(
                    trace::DefaultOnResponse::new()
                        .level(tracing::Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                ),
        );

    let listener = tokio::net::TcpListener::bind(config.addr).await?;
    info!("Listening on http://{}", config.addr);

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
    let openapi_json = serde_json::to_string_pretty(api)?;

    // Use current directory or workspace root
    let output_path = if fs::metadata("./apps/http-server").is_ok() {
        "./apps/http-server/openapi.json"
    } else {
        "./openapi.json"
    };

    fs::write(output_path, &openapi_json)?;
    info!("✅ Generated OpenAPI spec: {}", output_path);
    info!("📄 {} bytes written", openapi_json.len());
    Ok(())
}
