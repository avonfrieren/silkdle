use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn hello() -> &'static str {
    "Hello from Rust backend!"
}

#[tokio::main]
async fn main() {
    // Créer les routes
    let app = Router::new().route("/", get(hello));

    // Adresse d’écoute
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    // Lancement du serveur (avec hyper)
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
