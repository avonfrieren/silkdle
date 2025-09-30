use std::sync::Arc;

use axum::{extract::{Form, State}, response::IntoResponse};
use serde::Deserialize;

use crate::{
    state::AppState,
    templates::HomepageTemplate,
    utils::{load_labels_and_zones, label_results, load_zones},
    models::Zone,
};

pub async fn homepage() -> impl IntoResponse {
    HomepageTemplate {
        results_history: vec![],
        zones: load_zones(),
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
    // target hardcoded for now (need to be chosen every day randomly)
    let target = Zone { 
        name: "Moss Grotto".to_string(), 
        size: 18, 
        acte: 1, 
        bosses: 4, 
        station: true 
    };

    let (labels, all_zones) = load_labels_and_zones();
    let chosen = all_zones.iter()
        .find(|z| z.name == input.zone_name)
        .expect("the chosen zone does not exist");

    let results = chosen.compare(&target);

    let results_history = {
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
