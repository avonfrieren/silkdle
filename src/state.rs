use std::sync::Mutex;
use std::collections::HashMap;

use crate::models::Zone;
use crate::{compare::CompareResult, utils::get_today_zone};

#[derive(Debug)]
pub struct AppState {
    pub guesses: Mutex<HashMap<String, Vec<Vec<CompareResult>>>>,
    pub daily_zone: Zone
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            guesses: Mutex::new(HashMap::new()),
            daily_zone: get_today_zone(),
        }
    }
}
