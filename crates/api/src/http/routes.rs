//! Routing definitions for CyberWiki API.

use axum::{
    routing::{get},
    Router,
};

use crate::adapters::core_bridge::CoreBridge;

pub fn create_router(_bridge: CoreBridge) -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> &'static str {
    "ok"
}
