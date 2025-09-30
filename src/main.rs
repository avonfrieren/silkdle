use std::{
    fs,
    fmt,
    sync::Arc,
    sync::Mutex
};

use axum::{
    extract::{Form, State},
    routing::get,
    Router
};

use tower_http::services::ServeDir;
use askama_axum::Template;
use std::net::SocketAddr;
use axum::routing::post;
use serde::Deserialize;

// pages
#[derive(Template)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    results_history: Vec<Vec<(String, CompareResult)>>,
    zones: Vec<Zone>,
}

async fn homepage() -> impl axum::response::IntoResponse {
    HomepageTemplate {
        results_history: vec![],
        zones: load_zones(),
    }
}

#[derive(Deserialize)]
struct ZoneChoice {
    zone_name: String,
}

// load zones like a generic tree
fn load_raw() -> toml::Value {
    let zones = std::fs::read_to_string("data/zones.toml").unwrap();
    toml::from_str(&zones).unwrap()
}

// load zones as a vector of every zones
fn load_zones() -> Vec<Zone> {
    let toml_str = fs::read_to_string("data/zones.toml")
        .expect("failed to read zones.toml");

    let zones_file: ZonesFile = toml::from_str(&toml_str)
        .expect("failed to parse toml");

    zones_file.zones
}

fn load_labels_and_zones() -> (Vec<String>, Vec<Zone>) {
    let raw = load_raw();

    // récupérer l'ordre des labels
    let labels = raw["labels"]
        .as_array().unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect::<Vec<_>>();

    let zones_vec = load_zones();

    (labels, zones_vec)
}

fn label_results(results: Vec<CompareResult>, labels: &Vec<String>) -> Vec<(String, CompareResult)> {
    labels.iter().cloned().zip(results).collect()
}

async fn choose_zone(State(state): State<Arc<AppState>>, Form(input): Form<ZoneChoice>) -> impl axum::response::IntoResponse {
    // target hardcoded for now (need to be chosen every day randomly)
    let target = Zone { name: "Moss Grotto".to_string(), size: 18, acte: 1, bosses: 4, station: true };
    
    // loading every labels and zones from the zones.toml file
    let (labels, all_zones) = load_labels_and_zones();
    // getting the zone struct with input zone name
    let chosen = all_zones.iter()
        .find(|z| z.name == input.zone_name)
        .expect("the chosen zone does not exist");

    // actual comparison (core logic)
    let results = chosen.compare(&target);

    let results_history: Vec<Vec<(String, CompareResult)>> = {
    let mut guesses = state.guesses.lock().unwrap();
    guesses.push(results);
    guesses.iter()
        .map(|g| label_results(g.clone(), &labels))
        .collect()
    };

    HomepageTemplate {
        results_history,
        zones: all_zones,
    }
}

// core logic
#[derive(Debug, Deserialize)]
struct Zone {
    name: String, // name on the map
    size: u8, // numbers of rooms (for now)
    acte: u8, // 1, 2 or 3: is it accessible normally ? (the intended way)
    bosses: u8, // numbers of bosses
    station: bool, // is there a station in the zone
}

#[derive(Debug, Deserialize)]
struct ZonesFile {
    zones: Vec<Zone>,
}

#[derive(Debug, Clone)]
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

impl Zone {
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

#[derive(Debug)]
struct AppState {
    guesses: Mutex<Vec<Vec<CompareResult>>>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        guesses: Mutex::new(vec![]),
    });
    
    // router
    let static_files = axum::Router::new().nest_service(
        "/static",
        ServeDir::new("static")
    );

    let app = Router::new()
        .route("/", get(homepage))
        .route("/choose_zone", post(choose_zone))
        .with_state(state.clone())
        .merge(static_files);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}