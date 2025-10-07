use std::sync::Arc;

use axum::{extract::{Form, State}, response::IntoResponse};
use serde::Deserialize;

use crate::{
    state::AppState,
    templates::HomepageTemplate,
    utils::load_labels_and_zones,
};

pub async fn homepage() -> impl IntoResponse {
    let (labels, zones) = load_labels_and_zones();
    HomepageTemplate {
        labels,
        results_history: vec![],
        zones,
    }
}

#[derive(Deserialize)]
pub struct ZoneChoice {
    pub zone_name: String,
}

pub async fn choose_zone(
    State(state): State<Arc<AppState>>,
    Form(input): Form<ZoneChoice>
) -> impl IntoResponse {
    let (labels, all_zones) = load_labels_and_zones();
    let chosen = all_zones.iter()
        .find(|z| z.name == input.zone_name)
        .expect("the chosen zone does not exist");

    let results = chosen.compare(&state.daily_zone);

    // Stocker l'historique comme des lignes de CompareResult (sans labels)
    let results_history = {
        let mut guesses = state.guesses.lock().unwrap();
        guesses.push(results);
        guesses.clone()
    };

    HomepageTemplate {
        labels,
        results_history,
        zones: all_zones,
    }
}
