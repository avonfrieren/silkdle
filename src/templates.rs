use askama_axum::Template;

use crate::{compare::CompareResult, models::Zone};

#[derive(Template)]
#[template(path = "homepage.html")]
pub struct HomepageTemplate {
    pub labels: Vec<String>,
    pub results_history: Vec<Vec<CompareResult>>,
    pub zones: Vec<Zone>,
}
