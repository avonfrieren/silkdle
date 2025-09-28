use axum::{ routing::get, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use askama_axum::Template;
use std::fmt;

// pages
#[derive(Template)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    test: &'static str,
    results_zone: Vec<CompareResult>,
}

async fn homepage() -> impl axum::response::IntoResponse {
    let z1 = Zone { name: "Moss Grotto", size: 19, acte: 1, bosses: 4, station: true };
    let z2 = Zone { name: "The Marrow", size: 18, acte: 1, bosses: 2, station: true };

    let results = z1.compare(&z2);

    HomepageTemplate {
        test: "silkdle",
        results_zone: results,
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

enum CompareResult {
    Smaller,
    Greater,
    Equal,
    NotCorrect
}

impl fmt::Display for CompareResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompareResult::Smaller => write!(f, "Smaller"),
            CompareResult::Greater => write!(f, "Greater"),
            CompareResult::Equal => write!(f, "Equal"),
            CompareResult::NotCorrect => write!(f, "NotCorrect"),
        }
    }
}

fn compare_values<T: Ord>(a: &T, b: &T) -> CompareResult {
    if a < b {
        CompareResult::Smaller
    } else if a > b {
        CompareResult::Greater
    } else {
        CompareResult::Equal
    }
}

fn compare_str(a: &str, b: &str) -> CompareResult {
    if a == b {
        CompareResult::Equal
    } else {
        CompareResult::NotCorrect
    }
}

fn compare_bool(a: &bool, b: &bool) -> CompareResult {
    if a == b {
        CompareResult::Equal
    } else {
        CompareResult::NotCorrect
    }
}

impl Zone<'_> {
    fn compare(&self, other: &Zone) -> Vec<CompareResult> {
        let mut results = Vec::new();

        results.push(compare_str(&self.name, &other.name));
        results.push(compare_values(&self.size, &other.size));
        results.push(compare_values(&self.acte, &other.acte));
        results.push(compare_values(&self.bosses, &other.bosses));
        results.push(compare_bool(&self.station, &other.station));

        results
    }
}

#[tokio::main]
async fn main() {
    // router
    let static_files = axum::Router::new().nest_service(
        "/static",
        ServeDir::new("static")
    );

    let app = Router::new()
        .route("/", get(homepage))
        .route("/login", get(login))
        .merge(static_files);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}