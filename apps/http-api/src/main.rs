use axum::Router;
use http_api::config;
use tower_http::{
    cors::CorsLayer,
    trace::{self, TraceLayer},
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: config::Config = config::Config::new()?;

    init_tracing();

    info!("Starting VPB ERP Backend...");

    let app = Router::new().with_state(CorsLayer::permissive()).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
            .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
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
                .unwrap_or_else(|_| "http_api=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
