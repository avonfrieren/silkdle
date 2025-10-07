use std::fs;
use rand_chacha::ChaCha8Rng;
use chrono_tz::Europe::Paris;

use rand::{
    SeedableRng,
    seq::IndexedRandom
};

use chrono::{
    TimeZone,
    Utc,
    NaiveDate,
};

use crate::{models::{Zone, ZonesFile}, compare::CompareResult};

// load zones from data file as a toml tree object like json
pub fn load_raw() -> toml::Value {
    let zones = std::fs::read_to_string("data/zones.toml").unwrap();
    toml::from_str(&zones).unwrap()
}

// load zones from data file as a vector of zones
pub fn load_zones() -> Vec<Zone> {
    let toml_str = fs::read_to_string("data/zones.toml")
        .expect("failed to read zones.toml");

    let zones_file: ZonesFile = toml::from_str(&toml_str)
        .expect("failed to parse toml");

    zones_file.zones
}

// load labels (attributes of a zone) and zones from data file
pub fn load_labels_and_zones() -> (Vec<String>, Vec<Zone>) {
    let raw = load_raw();

    let labels = raw["labels"]
        .as_array().unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect::<Vec<_>>();

    let zones_vec = load_zones();

    (labels, zones_vec)
}

// label the results of a comparison between two zones for displaying purpose
pub fn label_results(results: Vec<CompareResult>, labels: &Vec<String>) -> Vec<(String, CompareResult)> {
    labels.iter().cloned().zip(results).collect()
}

// constructing a random pick between existing zone for the day
fn get_daily_zone(zones: &Vec<Zone>, today: NaiveDate) -> Zone {
    let day_id = today.signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).num_days();
    let seed = day_id as u64;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    zones.choose(&mut rng).unwrap().clone()
}

// get a zone for each day
pub fn get_today_zone() -> Zone {
    let today = Paris.from_utc_datetime(&Utc::now().naive_utc()).date_naive();
    let zones = &load_zones();
    get_daily_zone(zones, today)
}