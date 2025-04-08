use std::fmt::{Display, Formatter};
use crate::components::calculation::recommendation_calculation::ScoreType::{D, S, T};
use dioxus::core_macro::Props;
use dioxus::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum RecValue {
    IsFinish(FinishRecValue),
    NoFinish(NonFinishRecValue),
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum ScoreType {
    T(u16),
    D(u16),
    S(u16),
}

impl Display for ScoreType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            T(val) => format!("T{val}"),
            D(val) => format!("D{val}"),
            S(val) => format!("S{val}"),
        };
        write!(f, "{}", str_val)
    }
}


impl ScoreType {
    pub(crate) fn display(&self) -> String {
        match self {
            T(val) => format!("T{val}"),
            D(val) => format!("D{val}"),
            S(val) => format!("S{val}"),
        }
    }
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct NonFinishRecValue {
    pub rec: u16,
    pub goal: u16,
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct FinishRecValue {
    pub primary_rec: Option<Vec<ScoreType>>,
    pub secondary_rec: Option<Vec<ScoreType>>,
}

pub fn display_score_types(input_vec: &Option<Vec<ScoreType>>) -> String {
    match input_vec {
        Some(primary_rec_val) => primary_rec_val
            .into_iter()
            .map(|val| val.display())
            .join(" "),
        None => "-".to_string(),
    }
}

enum IsFinish {
    Yes,
    No { goal: u16 },
}

pub fn determine_rec(remaining_val: u16) -> RecValue {
    match remaining_val {
        remaining_val if remaining_val < 91 => {
            RecValue::IsFinish(determine_finish_rec(remaining_val))
        }
        remaining_val if remaining_val > 180 => {
            let val = NonFinishRecValue {
                goal: remaining_val - 180,
                rec: 180,
            };
            RecValue::NoFinish(val)
        }
        _ => {
            let val = NonFinishRecValue {
                goal: 32,
                rec: remaining_val - 32,
            };
            RecValue::NoFinish(val)
        }
    }
}

fn determine_finish_rec(remaining_val: u16) -> FinishRecValue {
    match remaining_val {
        0 | 1 => FinishRecValue {
            primary_rec: None,
            secondary_rec: None,
        },
        remaining_val if remaining_val <= 40 && remaining_val % 2 == 0 => FinishRecValue {
            primary_rec: Some(vec![D(remaining_val / 2)]),
            secondary_rec: None,
        },
        remaining_val if remaining_val <= 40 && remaining_val % 2 == 1 => {
            let ((single, double), secondary_ref) = under_40_two_dart_finish(remaining_val);
            FinishRecValue {
                primary_rec: Some(vec![S(single), D(double)]),
                secondary_rec: secondary_ref.map(|(single, double)| vec![S(single), D(double)]),
            }
        }
        remaining_val if remaining_val <= 52 && remaining_val > 40 => {
            let (single, double) = (remaining_val - 32, 16);
            let (single_snd, double_snd) = (remaining_val - 40, 20);
            FinishRecValue {
                primary_rec: Some(vec![S(single), D(double)]),
                secondary_rec: Some(vec![S(single_snd), D(double_snd)]),
            }
        }
        remaining_val if remaining_val <= 60 && remaining_val > 52 => {
            let (single, double) = (remaining_val - 40, 20);
            FinishRecValue {
                primary_rec: Some(vec![S(single), D(double)]),
                secondary_rec: None,
            }
        }
        61 => FinishRecValue {
            primary_rec: Some(vec![S(25), D(18)]),
            secondary_rec: Some(vec![T(15), D(8)]),
        },
        62 => FinishRecValue {
            primary_rec: Some(vec![T(10), D(16)]),
            secondary_rec: Some(vec![T(14), D(10)]),
        },
        63 => FinishRecValue {
            primary_rec: Some(vec![T(13), D(12)]),
            secondary_rec: None,
        },
        64 => FinishRecValue {
            primary_rec: Some(vec![T(16), D(8)]),
            secondary_rec: Some(vec![T(8), D(20)]),
        },
        65 => FinishRecValue {
            primary_rec: Some(vec![S(25), D(20)]),
            secondary_rec: Some(vec![T(11), D(16)]),
        },
        66 => FinishRecValue {
            primary_rec: Some(vec![T(10), D(18)]),
            secondary_rec: Some(vec![T(18), D(6)]),
        },
        67 => FinishRecValue {
            primary_rec: Some(vec![T(9), D(20)]),
            secondary_rec: None,
        },
        68 => FinishRecValue {
            primary_rec: Some(vec![T(20), D(4)]),
            secondary_rec: Some(vec![T(16), D(10)]),
        },
        69 => FinishRecValue {
            primary_rec: Some(vec![T(11), D(18)]),
            secondary_rec: Some(vec![T(15), D(12)]),
        },
        70 => FinishRecValue {
            primary_rec: Some(vec![T(18), D(8)]),
            secondary_rec: Some(vec![T(10), D(20)]),
        },
        71 => FinishRecValue {
            primary_rec: Some(vec![T(13), D(16)]),
            secondary_rec: Some(vec![T(17), D(10)]),
        },
        72 => FinishRecValue {
            primary_rec: Some(vec![T(20), D(6)]),
            secondary_rec: Some(vec![T(12), D(18)]),
        },
        73 => FinishRecValue {
            primary_rec: Some(vec![T(19), D(8)]),
            secondary_rec: None,
        },
        74 => FinishRecValue {
            primary_rec: Some(vec![T(14), D(16)]),
            secondary_rec: Some(vec![T(16), D(13)]),
        },
        75 => FinishRecValue {
            primary_rec: Some(vec![T(17), D(12)]),
            secondary_rec: Some(vec![S(25), S(50)]),
        },
        76 => FinishRecValue {
            primary_rec: Some(vec![T(16), D(14)]),
            secondary_rec: None,
        },
        77 => FinishRecValue {
            primary_rec: Some(vec![T(19), D(10)]),
            secondary_rec: None,
        },
        78 => FinishRecValue {
            primary_rec: Some(vec![T(18), D(12)]),
            secondary_rec: None,
        },
        79 => FinishRecValue {
            primary_rec: Some(vec![T(19), D(11)]),
            secondary_rec: Some(vec![T(13), D(20)]),
        },
        80 => FinishRecValue {
            primary_rec: Some(vec![T(20), D(10)]),
            secondary_rec: Some(vec![D(20), D(20)]),
        },
        81 => FinishRecValue {
            primary_rec: Some(vec![T(19), D(12)]),
            secondary_rec: Some(vec![T(15), D(18)]),
        },
        82 => FinishRecValue {
            primary_rec: Some(vec![T(14), D(20)]),
            secondary_rec: Some(vec![S(50), D(16)]),
        },
        83 => FinishRecValue {
            primary_rec: Some(vec![T(17), D(16)]),
            secondary_rec: Some(vec![S(50), S(17), D(8)]),
        },
        84 => FinishRecValue {
            primary_rec: Some(vec![T(20), D(12)]),
            secondary_rec: Some(vec![T(16), D(18)]),
        },
        85 => FinishRecValue {
            primary_rec: Some(vec![T(15), D(20)]),
            secondary_rec: Some(vec![T(19), D(14)]),
        },
        86 => FinishRecValue {
            primary_rec: Some(vec![T(18), D(16)]),
            secondary_rec: None,
        },
        87 => FinishRecValue {
            primary_rec: Some(vec![T(17), D(18)]),
            secondary_rec: None,
        },
        88 => FinishRecValue {
            primary_rec: Some(vec![T(20), D(14)]),
            secondary_rec: None,
        },
        89 => FinishRecValue {
            primary_rec: Some(vec![T(19), D(16)]),
            secondary_rec: None,
        },
        90 => FinishRecValue {
            primary_rec: Some(vec![T(20), D(15)]),
            secondary_rec: Some(vec![S(50), D(20)]),
        },
        91 => FinishRecValue {
            primary_rec: Some(vec![T(17), D(20)]),
            secondary_rec: Some(vec![S(50), T(16), D(9)]),
        },
        92 => FinishRecValue {
            primary_rec: Some(vec![T(20), D(16)]),
            secondary_rec: Some(vec![S(50), T(17), D(8)]),
        },
        93 => FinishRecValue {
            primary_rec: Some(vec![T(19), D(18)]),
            secondary_rec: Some(vec![S(50), T(18), D(7)]),
        },
        94 => FinishRecValue {
            primary_rec: Some(vec![T(18), D(20)]),
            secondary_rec: Some(vec![S(25), T(19), D(6)]),
        },
        95 => FinishRecValue {
            primary_rec: Some(vec![T(19), D(19)]),
            secondary_rec: Some(vec![S(19), D(19), D(19)]),
        },
        96 => FinishRecValue {
            primary_rec: Some(vec![T(20), D(18)]),
            secondary_rec: None,
        },
        97 => FinishRecValue {
            primary_rec: Some(vec![T(19), D(20)]),
            secondary_rec: None,
        },
        98 => FinishRecValue {
            primary_rec: Some(vec![T(20), D(19)]),
            secondary_rec: None,
        },
        99 => FinishRecValue {
            primary_rec: Some(vec![T(19), S(10),  D(16)]),
            secondary_rec: Some(vec![T(17), S(16),  D(16)]),
        },
        100 => FinishRecValue {
            primary_rec: Some(vec![T(20), D(20)]),
            secondary_rec: Some(vec![S(20), D(20),  D(20)]),
        },


        _ => panic!("Todo: Unreachable state {remaining_val}"),
    }
}

fn under_40_two_dart_finish(remaining_val: u16) -> ((u16, u16), Option<(u16, u16)>) {
    match remaining_val {
        3 => ((1, 1), None),
        39 | 37 | 35 | 33 => ((remaining_val - 32, 16), Some((remaining_val - 20, 10))),
        remaining_val if remaining_val > 4 && remaining_val < 8 => ((remaining_val - 4, 2), None),
        remaining_val if remaining_val > 8 && remaining_val < 12 => ((remaining_val - 8, 4), None),
        remaining_val if remaining_val > 12 && remaining_val < 16 => {
            ((remaining_val - 8, 4), Some((remaining_val - 10, 5)))
        }
        remaining_val if remaining_val > 16 && remaining_val < 22 => {
            ((remaining_val - 16, 8), Some((remaining_val - 10, 5)))
        }
        remaining_val if remaining_val > 22 && remaining_val < 32 => {
            ((remaining_val - 16, 8), Some((remaining_val - 10, 5)))
        }
        _ => panic!("Todo: Unreachable state {remaining_val}"),
    }
}

#[cfg(test)]
mod test {
    use std::any::type_name;
    use crate::components::calculation::recommendation_calculation::{determine_rec, FinishRecValue, RecValue, ScoreType};

    #[test]
    fn no_panic_due_unreachable_state() {
        (0..501).into_iter().for_each(|input_val| {
            determine_rec(input_val as u16);
        })
    }

    fn sum_parser(input_vector: Vec<ScoreType>) -> u16 {
        input_vector.into_iter().map(score_parser).sum()
    }

    fn score_parser(input_score: ScoreType) -> u16 {
        match input_score {
            ScoreType::T(val) => 3 * val,
            ScoreType::D(val) => 2 * val,
            ScoreType::S(val) => 1 * val,
        }
    }

    #[test]
    fn expect_remaining() {
        let result: Vec<(u16, RecValue)> = (0..90)
            .into_iter()
            .map(|input_val| (input_val as u16, determine_rec(input_val as u16)))
            .collect();
        result.into_iter().for_each(|(input_val, rec_value) | {
            match rec_value {
                RecValue::IsFinish(is_finish_value) => {
                    let FinishRecValue{primary_rec, secondary_rec} = is_finish_value;
                    check_finish_sum_expect_remaining(input_val, primary_rec);
                    check_finish_sum_expect_remaining(input_val, secondary_rec);
                }
                RecValue::NoFinish(_) => {
                    //todo
                }
            }
        });
    }

    fn check_finish_sum_expect_remaining(input_val: u16, rec_vector_maybe: Option<Vec<ScoreType>>) {
        match rec_vector_maybe {
            Some(rec_vector) => {
                let sum_val = sum_parser(rec_vector);
                assert_eq!(
                    sum_val, input_val,
                    "Sum Recommendation for {input_val } failed: Should {input_val}, Is {sum_val}"
                )
            }
            None => {}
        }
    }

    #[test]
    fn finish_ends_with_double() {
        let result: Vec<(u16, FinishRecValue)> = (0..90)
            .into_iter()
            .map(|input_val| (input_val as u16, determine_rec(input_val as u16)))
            .filter_map(|(input_val, rec)| {
                match rec {
                    RecValue::IsFinish(is_finish_value) => Some((input_val, is_finish_value)),
                    _ => None
                }
            })
            .collect();
        result.into_iter().for_each(|(input_val, rec_value) | {
            let FinishRecValue{primary_rec, secondary_rec} = rec_value;
            check_finish_ends_with_double(input_val, primary_rec);
            check_finish_ends_with_double(input_val, secondary_rec);
        });
    }

    fn check_finish_ends_with_double(input_val: u16, rec_vector_maybe: Option<Vec<ScoreType>>) {
        match rec_vector_maybe {
            Some(rec_vector) => {
                let last_val_maybe = rec_vector.last();
                assert!(last_val_maybe.is_some());
                let last_val = last_val_maybe.unwrap();
                match last_val {
                    ScoreType::D(_) => {
                        // to be expected
                    },
                    ScoreType::S(50) => {
                        // to be expected
                    }
                    _ => panic!("{}", format!("lastval of {input_val} is a finish but has {last_val} instead of a Double"))

                }
            }
            None => {}
        }
    }
}
