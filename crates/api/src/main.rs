//! CyberWiki API service entry point.
//!
//! Boots an HTTP server for controlled, documentation-focused access
//! to the CyberWiki knowledge corpus.

mod adapters;
mod http;

use std::net::SocketAddr;

use axum::Router;
use http::routes::create_router;

use crate::adapters::core_bridge::CoreBridge;

#[tokio::main]
async fn main() {
    let bridge = CoreBridge::new();
    let app: Router = create_router(bridge);

    let addr: SocketAddr = "0.0.0.0:8080".parse().expect("valid socket address");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server run");
}
