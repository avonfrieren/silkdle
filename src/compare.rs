use std::fmt;

use crate::models::Zone;

#[derive(Debug, Clone)]
pub enum CompareResult {
    Smaller,
    Greater,
    Equal,
    NotCorrect,
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

pub fn compare_values<T: Ord>(a: &T, b: &T) -> CompareResult {
    if a < b {
        CompareResult::Smaller
    } else if a > b {
        CompareResult::Greater
    } else {
        CompareResult::Equal
    }
}

pub fn compare_str(a: &str, b: &str) -> CompareResult {
    if a == b {
        CompareResult::Equal
    } else {
        CompareResult::NotCorrect
    }
}

pub fn compare_bool(a: &bool, b: &bool) -> CompareResult {
    if a == b {
        CompareResult::Equal
    } else {
        CompareResult::NotCorrect
    }
}

impl Zone {
    pub fn compare(&self, other: &Zone) -> Vec<CompareResult> {
        vec![
            compare_str(&self.name, &other.name),
            compare_values(&self.size, &other.size),
            compare_values(&self.acte, &other.acte),
            compare_values(&self.bosses, &other.bosses),
            compare_bool(&self.station, &other.station),
        ]
    }
}
