use askama_axum::Template;

use crate::{compare::CompareResult, models::Zone};

#[derive(Template)]
#[template(path = "homepage.html")]
pub struct HomepageTemplate {
    pub results_history: Vec<Vec<(String, CompareResult)>>,
    pub zones: Vec<Zone>,
}
