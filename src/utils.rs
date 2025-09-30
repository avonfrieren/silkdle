use std::fs;

use crate::{models::{Zone, ZonesFile}, compare::CompareResult};

pub fn load_raw() -> toml::Value {
    let zones = std::fs::read_to_string("data/zones.toml").unwrap();
    toml::from_str(&zones).unwrap()
}

pub fn load_zones() -> Vec<Zone> {
    let toml_str = fs::read_to_string("data/zones.toml")
        .expect("failed to read zones.toml");

    let zones_file: ZonesFile = toml::from_str(&toml_str)
        .expect("failed to parse toml");

    zones_file.zones
}

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

pub fn label_results(results: Vec<CompareResult>, labels: &Vec<String>) -> Vec<(String, CompareResult)> {
    labels.iter().cloned().zip(results).collect()
}
