use crate::components::calculation::statistic_calculation::StatAbsolutValue::{HasValue, NoValue};
use crate::domain::{Metric, Score};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum CompareValue {
    HasValue(i32, Option<i32>),
    NoValue,
}

impl CompareValue {
    pub fn display(&self) -> String {
        match self {
            CompareValue::HasValue(set_val, None) => {
                format!("{:+} compared to set average", set_val)
            }
            _ => "-".to_string(),
        }
    }
}

pub fn calculate_compare_value(
    leg_stat_value: &StatAbsolutValue,
    set_stat_value: &StatAbsolutValue,
) -> CompareValue {
    match (leg_stat_value, set_stat_value) {
        (HasValue(leg_value), HasValue(set_value)) => {
            CompareValue::HasValue(*leg_value as i32 - *set_value as i32, None)
        }
        _ => CompareValue::NoValue,
    }
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum StatAbsolutValue {
    HasValue(u16),
    NoValue,
}

impl StatAbsolutValue {
    pub fn display(&self) -> String {
        match self {
            HasValue(val) => format!("{val}"),
            NoValue => "-".to_string(),
        }
    }
    pub fn value_or_zero(&self) -> u16 {
        match self {
            HasValue(val) => *val,
            NoValue => 0,
        }
    }
}

pub fn double_percentage(
    double_succ: StatAbsolutValue,
    double_attempts: StatAbsolutValue,
) -> StatAbsolutValue {
    match (double_succ, double_attempts) {
        (HasValue(double_succ_value), HasValue(double_attempt_value)) => {
            HasValue(((double_succ_value as f32) / (double_attempt_value as f32) * 100.0) as u16)
        }
        _ => NoValue,
    }
}

pub fn parse_stat_absolut_value(val_input: u16) -> StatAbsolutValue {
    match val_input {
        0 => NoValue,
        val => HasValue(val_input),
    }
}

impl Display for StatAbsolutValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            HasValue(val) => format!("{val}"),
            NoValue => "-".to_string(),
        };
        write!(f, "{}", str_val)
    }
}

pub fn leg_live_average(mut scores: Vec<Score>) -> StatAbsolutValue {
    if scores.len() == 0 {
        NoValue
    } else {
        let tail = scores.split_off(1);
        let sum = tail.iter().map(|s| s.thrown).sum::<u16>();
        if tail.len() > 0 {
            HasValue(sum / tail.len() as u16)
        } else {
            NoValue
        }
    }
}

pub fn enhance_set_average(mut scores: Vec<Score>, set_metric: &Metric) -> StatAbsolutValue {
    let Metric {
        sum: set_sum,
        score_amount: set_score_amount,
        ..
    } = set_metric;
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

pub fn enhance_average_value(
    enhancing_stat: Option<&StatAbsolutValue>,
    target: u16,
    divider: u16,
) -> StatAbsolutValue {
    match enhancing_stat {
        None => match divider {
            0 => NoValue,
            _ => parse_stat_absolut_value(target / divider),
        },
        Some(enhancing_value) => {
            parse_stat_absolut_value((enhancing_value.value_or_zero() + target) / (divider + 1))
        }
    }
}

pub fn first_three_average(scores: Vec<Score>) -> StatAbsolutValue {
    let split_check = scores.split_at_checked(1);
    match split_check {
        Some((_, tail)) => {
            let split_val = std::cmp::min(3, tail.len());
            let shortend = tail.split_at_checked(split_val);
            match shortend {
                Some((first3, _)) => {
                    let sum = first3.into_iter().map(|s| s.thrown).sum::<u16>();
                    if first3.len() > 0 {
                        HasValue(sum / first3.len() as u16)
                    } else {
                        NoValue
                    }
                }
                None => NoValue,
            }
        }
        None => NoValue,
    }
}

#[cfg(test)]
mod test {
    use crate::components::calculation::statistic_calculation::{
        first_three_average, leg_live_average, StatAbsolutValue,
    };
    use crate::domain::Score;

    fn helper(thrown: u16, throw_order: u16) -> Score {
        Score {
            leg_id: 1,
            remaining: 501,
            thrown,
            throw_order,
            double_attempt: None,
        }
    }

    #[test]
    fn live_average_ignores_only_init() {
        let input = vec![helper(0, 0)];
        let res = leg_live_average(input);
        assert_eq!(res, StatAbsolutValue::NoValue);
    }

    #[test]
    fn live_average_ignores_init() {
        let input = vec![helper(0, 0), helper(20, 1), helper(30, 2)];
        let res = leg_live_average(input);
        assert_eq!(res, StatAbsolutValue::HasValue(25));
    }

    #[test]
    fn first_three_average_ignores_only_init() {
        let input = vec![helper(0, 0)];
        let res = first_three_average(input);
        assert_eq!(res, StatAbsolutValue::NoValue);
    }

    #[test]
    fn first_three_average_ignores_init() {
        let input = vec![helper(0, 0), helper(20, 1), helper(30, 2)];
        let res = first_three_average(input);
        assert_eq!(res, StatAbsolutValue::HasValue(25));
    }

    #[test]
    fn first_three_average_ignores_following_inputs() {
        let input = vec![
            helper(0, 0),
            helper(20, 1),
            helper(30, 2),
            helper(40, 2),
            helper(10000, 2),
        ];
        let res = first_three_average(input);
        assert_eq!(res, StatAbsolutValue::HasValue(30));
    }
}
