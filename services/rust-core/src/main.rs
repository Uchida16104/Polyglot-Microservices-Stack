mod handlers;
mod models;
mod middleware;
mod db;
mod queue;
mod error;

use axum::{
    routing::{get, post},
    Router,
    middleware as axum_middleware,
};
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    request_id::MakeRequestUuid,
    request_id::PropagateRequestIdLayer,
    request_id::SetRequestIdLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;
use sqlx::PgPool;
use redis::Client as RedisClient;

use handlers::compile::{submit_compile_job, get_compile_status};
use handlers::execute::submit_execute_job;
use middleware::auth::internal_auth;

pub struct AppState {
    pub db: PgPool,
    pub redis: RedisClient,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_core=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let redis_url = std::env::var("REDIS_URL")
        .expect("REDIS_URL must be set");

    let db = PgPool::connect(&database_url).await?;
    let redis = RedisClient::open(redis_url)?;

    sqlx::migrate!("./migrations").run(&db).await?;

    let state = Arc::new(AppState { db, redis: redis.clone() });

    let worker_state = state.clone();
    tokio::spawn(async move {
        queue::worker::run_compile_worker(worker_state).await;
    });

    let x_request_id = axum::http::HeaderName::from_static("x-request-id");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/compile", post(submit_compile_job))
        .route("/compile/:job_id/status", get(get_compile_status))
        .route("/execute", post(submit_execute_job))
        .layer(axum_middleware::from_fn(internal_auth))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http())
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(SetRequestIdLayer::new(x_request_id, MakeRequestUuid))
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8001".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("rust-core listening on {addr}");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "status": "ok", "service": "rust-core" }))
}
