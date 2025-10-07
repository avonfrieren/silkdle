use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use tower_http::services::ServeDir;
use tower_cookies::CookieManagerLayer;

mod state;
mod models;
mod compare;
mod handlers;
mod templates;
mod utils;

use crate::state::AppState;
use crate::handlers::{homepage, choose_zone};

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let static_files = Router::new().nest_service("/static", ServeDir::new("static"));

    let app = Router::new()
        .route("/", get(homepage))
        .route("/choose_zone", post(choose_zone))
        .with_state(state.clone())
        .merge(static_files)
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
