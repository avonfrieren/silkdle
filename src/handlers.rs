use std::sync::Arc;

use axum::{extract::{Form, State}, response::IntoResponse};
use serde::Deserialize;
use tower_cookies::{Cookies, Cookie};
use tower_cookies::cookie::SameSite;
use uuid::Uuid;

use crate::{
    state::AppState,
    templates::HomepageTemplate,
    utils::load_labels_and_zones,
};

fn get_or_set_uid(cookies: &Cookies) -> String {
    if let Some(c) = cookies.get("uid") {
        c.value().to_string()
    } else {
        let uid = Uuid::new_v4().to_string();
        let mut cookie = Cookie::new("uid", uid.clone());
        cookie.set_path("/");
        cookie.set_http_only(true);
        cookie.set_same_site(SameSite::Lax);
        cookies.add(cookie);
        uid
    }
}

pub async fn homepage(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
) -> impl IntoResponse {
    let uid = get_or_set_uid(&cookies);
    let (labels, zones) = load_labels_and_zones();

    let results_history = {
        let mut map = state.guesses.lock().unwrap();
        map.entry(uid).or_default().clone()
    };

    HomepageTemplate {
        labels,
        results_history,
        zones,
    }
}

#[derive(Deserialize)]
pub struct ZoneChoice {
    pub zone_name: String,
}

pub async fn choose_zone(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Form(input): Form<ZoneChoice>
) -> impl IntoResponse {
    let uid = get_or_set_uid(&cookies);

    let (labels, all_zones) = load_labels_and_zones();
    let chosen = all_zones.iter()
        .find(|z| z.name == input.zone_name)
        .expect("the chosen zone does not exist");

    let results = chosen.compare(&state.daily_zone);

    let results_history = {
        let mut map = state.guesses.lock().unwrap();
        let entry = map.entry(uid).or_default();
        entry.push(results);
        entry.clone()
    };

    HomepageTemplate {
        labels,
        results_history,
        zones: all_zones,
    }
}
