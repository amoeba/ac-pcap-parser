use axum::{
    extract::Query,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use tracing::info;

mod discord;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
        )
        .init();

    // Check for Discord OAuth token
    let discord_token = std::env::var("DISCORD_OAUTH_TOKEN").ok();
    if discord_token.is_none() {
        tracing::warn!("DISCORD_OAUTH_TOKEN not set - Discord API features disabled");
    }

    let app = create_router();

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind listener");

    info!("Server running on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

fn create_router() -> Router {
    let dist_path = std::path::PathBuf::from("dist");

    Router::new()
        .route("/api/health", get(health))
        .route("/api/discord", get(discord_pull))
        .nest_service("/", ServeDir::new(&dist_path))
}

/// Health check endpoint
async fn health() -> &'static str {
    "OK"
}

#[derive(Deserialize)]
struct DiscordQuery {
    channel: String,
    msg: String,
}

#[derive(Serialize)]
struct DiscordError {
    error: String,
}

/// Fetch PCAP from Discord message attachment
async fn discord_pull(
    Query(params): Query<DiscordQuery>,
) -> Result<Vec<u8>, (StatusCode, Json<DiscordError>)> {
    // Validate channel and message IDs
    if params.channel.is_empty() || params.msg.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(DiscordError {
                error: "Missing channel or msg parameters".to_string(),
            }),
        ));
    }

    // Check if token is available
    let _token = std::env::var("DISCORD_OAUTH_TOKEN").map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(DiscordError {
                error: "Discord OAuth token not configured".to_string(),
            }),
        )
    })?;

    // TODO: Implement Discord API call
    // 1. Validate snowflake IDs
    // 2. Fetch message details from Discord API
    // 3. Extract PCAP attachment
    // 4. Return binary PCAP data

    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(DiscordError {
            error: "Discord message pull not yet implemented".to_string(),
        }),
    ))
}
