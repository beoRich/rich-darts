use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::components::calculation::statistic_calculation::AverageValue::{HasValue, NoValue};
use crate::domain::Score;

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum AverageValue {
    HasValue(u16),
    NoValue
}


impl AverageValue {
    pub fn display(&self) -> String {
        match self {
            HasValue(val) => format!("{val}"),
            NoValue => "-".to_string()
        }
    }
}


impl Display for AverageValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            HasValue(val) => format!("{val}"),
            NoValue => "-".to_string()

        };
        write!(f, "{}", str_val)
    }
}

pub fn live_average(scores: Vec<Score>) -> AverageValue {
    let split_check =  scores.split_at_checked(1);
    match split_check {
        Some((_, tail)) => {
            let sum = tail.into_iter().map(|s| s.thrown).sum::<u16>();
            if tail.len() > 0 {
                HasValue(sum/tail.len() as u16)
            } else {
                NoValue
            }
            }

        None => NoValue
    }

}