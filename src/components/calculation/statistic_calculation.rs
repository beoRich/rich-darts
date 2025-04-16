use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use tracing::debug;
use web_sys::js_sys::Math;
use crate::components::calculation::statistic_calculation::AverageValue::{HasValue, NoValue};
use crate::domain::{Metric, Score};

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

pub fn leg_live_average(mut scores: Vec<Score>) -> AverageValue {
    if scores.len() == 0 {
        NoValue
    }
    else {
        let tail = scores.split_off(1);
        let sum = tail.iter().map(|s| s.thrown).sum::<u16>();
        if tail.len() > 0 {
            HasValue(sum / tail.len() as u16)
        } else {
            NoValue
        }
    }
}

pub fn enhance_set_average(mut scores: Vec<Score>, set_metric: Metric) -> AverageValue {
    let Metric{sum: set_sum, score_amount: set_score_amount} = set_metric;
    let mut leg_sum = 0;
    let mut leg_score_amount = 0;
    if scores.len() > 0 {
        let tail = scores.split_off(1);
        leg_sum = tail.iter().map(|s| s.thrown).sum::<u16>();
        leg_score_amount = tail.len() as u16;
    }
    let common_score_amount = set_score_amount + leg_score_amount;
    if common_score_amount == 0 {
        NoValue
    } else {
        HasValue((set_sum + leg_sum) / common_score_amount)
    }
}

pub fn first_three_average(scores: Vec<Score>) -> AverageValue {
    let split_check =  scores.split_at_checked(1);
    match split_check {
        Some((_, tail)) => {
            let split_val = std::cmp::min(3, tail.len());
            let shortend = tail.split_at_checked(split_val);
            match shortend {
                Some((first3,_)) =>  {
                    let sum = first3.into_iter().map(|s| s.thrown).sum::<u16>();
                    if first3.len() > 0 {
                        HasValue(sum/first3.len() as u16)
                    } else {
                        NoValue
                    }
                }
                None => NoValue
            }
        }
        None => NoValue
    }
}

#[cfg(test)]
mod test {
    use crate::components::calculation::statistic_calculation::{first_three_average, leg_live_average, AverageValue};
    use crate::domain::Score;

    fn helper(thrown: u16, throw_order: u16) -> Score {
        Score{leg_id: 1, remaining: 501, thrown, throw_order, double_attempt: None}
    }

    #[test]
    fn live_average_ignores_only_init() {
        let input =  vec![helper(0, 0)];
        let res = leg_live_average(input);
        assert_eq!(res, AverageValue::NoValue);
    }

    #[test]
    fn live_average_ignores_init() {
        let input =  vec![helper(0, 0), helper(20, 1), helper(30, 2)];
        let res = leg_live_average(input);
        assert_eq!(res, AverageValue::HasValue(25));
    }

    #[test]
    fn first_three_average_ignores_only_init() {
        let input =  vec![helper(0, 0)];
        let res = first_three_average(input);
        assert_eq!(res, AverageValue::NoValue);
    }

    #[test]
    fn first_three_average_ignores_init() {
        let input =  vec![helper(0, 0), helper(20, 1), helper(30, 2)];
        let res = first_three_average(input);
        assert_eq!(res, AverageValue::HasValue(25));
    }

    #[test]
    fn first_three_average_ignores_following_inputs() {
        let input =  vec![helper(0, 0),
                          helper(20, 1),
                          helper(30, 2),
                          helper(40, 2),
                          helper(10000, 2)
        ];
        let res = first_three_average(input);
        assert_eq!(res, AverageValue::HasValue(30));
    }
}

