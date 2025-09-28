use axum::{ routing::get, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use askama_axum::Template;

// pages
#[derive(Template)]
#[template(path = "homepage.html")]
struct HomepageTemplate<'a> {
    test: &'a str,
}

async fn homepage() -> impl axum::response::IntoResponse {
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

// core logic
#[derive(Debug)]
struct Zone<'a> {
    name: &'a str, // name on the map
    size: u8, // numbers of rooms (for now)
    acte: u8, // 1, 2 or 3: is it accessible normally ? (the intended way)
    bosses: u8, // numbers of bosses, not elite mobs
    station: bool, // is there a station in the zone
}

impl Zone<'_> {
    fn compare(&self, other: &Zone) {
        if self.name == other.name {
            println!("correct : name");
        } else {
            println!("not correct : name")
        }

        if self.size <= other.size {
            println!("smaller : size");
        } else if self.size >= other.size {
            println!("greater : size")
        } else {
            println!("correct : size");
        }

        if self.acte < other.acte {
            println!("smaller : acte");
        } else if self.acte > other.acte {
            println!("greater : acte")
        } else {
            println!("correct : acte");
        }

        if self.bosses <= other.bosses {
            println!("smaller : bosses");
        } else if self.bosses >= other.bosses {
            println!("greater : bosses")
        } else {
            println!("correct : bosses");
        }
        
        if self.station == other.station {
            println!("correct : station");
        } else {
            println!("not correct : station")
        }
    }
}

#[tokio::main]
async fn main() {
    // test
    let z1 = Zone { name: "Moss Grotto", size: 19, acte: 1, bosses: 4, station: true };
    let z2 = Zone { name: "The Marrow", size: 18, acte: 1, bosses: 2, station: true };

    z1.compare(&z2);

    // router
    // let static_files = axum::Router::new().nest_service(
    //     "/static",
    //     ServeDir::new("static")
    // );

    // let app = Router::new()
    //     .route("/", get(homepage))
    //     .route("/login", get(login))
    //     .merge(static_files);

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("🚀 Server running at http://{}", addr);

    // axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
    //     .await
    //     .unwrap();
}