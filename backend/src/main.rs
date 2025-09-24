use axum::Router;
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // router
    let app = Router::new().nest_service(
        "/",
        ServeDir::new("../static").append_index_html_on_directories(true),
    );

    // adresse d'ecoute
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    // lancement du server avec hyper
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
