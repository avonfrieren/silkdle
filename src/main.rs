use axum::{ routing::get, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use askama_axum::Template;

#[derive(Template)] // code automatiquemetn generer permettant a la struct d'etre rendu en html
#[template(path = "homepage.html")] // indique que le template se trouve a la racine du dossier templates lui meme par defaut a la racine
struct HomepageTemplate<'a> {
    test: &'a str,
}

async fn homepage() -> impl axum::response::IntoResponse {
    // instantiation, injection d'une variable et renvoi du template a afficher
    HomepageTemplate { 
        test: "silkdle"
    }
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a>{
    email: &'a str,
}

async fn login() -> impl axum::response::IntoResponse {
    LoginTemplate {
        email : "avon@avon.avon"
    }
}

#[tokio::main]
async fn main() {
    // router
    // dis au routeur que l'url "/static" correspond au dossier statique sur le disque pour qu'il sache ou allez chercher les assets
    let static_files = axum::Router::new().nest_service(
        "/static",
        ServeDir::new("static")
    );

    // instantiation router, declaration des routes, fusion des fichiers statique
    let app = Router::new()
        .route("/", get(homepage))
        .route("/login", get(login))
        .merge(static_files);

    // adresse d'ecoute
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    // lancement du server avec hyper
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}