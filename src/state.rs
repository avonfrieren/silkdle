use std::sync::Mutex;

use crate::compare::CompareResult;

#[derive(Debug)]
pub struct AppState {
    pub guesses: Mutex<Vec<Vec<CompareResult>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            guesses: Mutex::new(vec![]),
        }
    }
}
