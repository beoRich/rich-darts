use dioxus::core_macro::Props;
use serde::{Deserialize, Serialize};
use dioxus::prelude::*;
use crate::components::calculation::recommendation_calculation::ScoreType::{D, S, T};

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum RecValue {
    IsFinish(FinishRecValue),
    NoFinish(NonFinishRecValue)

}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum ScoreType {
    T(u16), D(u16), S(u16)
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct NonFinishRecValue {
    pub rec: String,
    pub goal: u16
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct FinishRecValue {
    pub primary_rec: Option<Vec<ScoreType>>,
    pub secondary_rec: Option<Vec<ScoreType>>,
}



enum IsFinish {
    Yes,
    No { goal: u16 },
}


pub fn determine_rec(remaining_val: u16) -> RecValue {
    match remaining_val {
        remaining_val if remaining_val < 91 => { RecValue::IsFinish(determine_finish_rec(remaining_val))}
        remaining_val if remaining_val > 180 => {
        let val = NonFinishRecValue{
            goal: remaining_val - 180,
            rec: "D20 D20 D20".to_string(),
        };
        RecValue::NoFinish(val)
        },
        _ => {
            let val = NonFinishRecValue{
                goal: 32,
                rec: format!("{}", remaining_val - 32),
            };
            RecValue::NoFinish(val)
        },
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
        },
        remaining_val if remaining_val <= 52 && remaining_val > 40 => {
            let (single, double) = (remaining_val - 32, 16);
            let (single_snd, double_snd) = (remaining_val - 40, 20);
            FinishRecValue {
                primary_rec: Some(vec![S(single), D(double)]),
                secondary_rec: Some(vec![S(single_snd), D(double_snd)])
            }
        },
        remaining_val if remaining_val <= 60 && remaining_val > 52  => {
            let (single, double) = (remaining_val - 40, 20);
            FinishRecValue {
                primary_rec: Some(vec![S(single), D(double)]),
                secondary_rec: None
            }
        },
        61 => FinishRecValue {primary_rec: Some(vec![S(25), D(18)]), secondary_rec: Some(vec![T(15), D(8)])},
        62 => FinishRecValue {primary_rec: Some(vec![T(10), D(16)]), secondary_rec: Some(vec![T(14), D(10)])},
        63 => FinishRecValue {primary_rec:Some(vec![T(13), D(12)]), secondary_rec: None},
        64 => FinishRecValue {primary_rec:Some(vec![T(16), D(8)]), secondary_rec: Some(vec![T(8), D(20)])},
        65 => FinishRecValue {primary_rec:Some(vec![S(25), D(20)]), secondary_rec: Some(vec![T(11), D(16)])},
        66 => FinishRecValue {primary_rec:Some(vec![T(10), D(18)]), secondary_rec: Some(vec![T(18), D(6)])},
        67 => FinishRecValue {primary_rec:Some(vec![T(9), D(20)]), secondary_rec: None},
        68 => FinishRecValue {primary_rec:Some(vec![T(20), D(4)]), secondary_rec: Some(vec![T(16), D(10)])},
        69 => FinishRecValue {primary_rec:Some(vec![T(11),D(18)]), secondary_rec: Some(vec![T(15), D(12)])},
        70 => FinishRecValue {primary_rec:Some(vec![T(18),D(8)]), secondary_rec: Some(vec![T(10), D(20)])},
        71 => FinishRecValue {primary_rec:Some(vec![T(13),D(16)]), secondary_rec: Some(vec![T(17), D(10)])},
        72 => FinishRecValue {primary_rec:Some(vec![T(20),D(6)]), secondary_rec: Some(vec![T(12), D(18)])},
        73 => FinishRecValue {primary_rec:Some(vec![T(19),D(8)]), secondary_rec: None},
        74 => FinishRecValue {primary_rec:Some(vec![T(14),D(16)]), secondary_rec: Some(vec![T(16), D(13)])},
        75 => FinishRecValue {primary_rec:Some(vec![T(17),D(12)]), secondary_rec: Some(vec![S(25), S(50)])},
        76 => FinishRecValue {primary_rec:Some(vec![T(16),D(14)]), secondary_rec: None},
        77 => FinishRecValue {primary_rec:Some(vec![T(19),D(10)]), secondary_rec: None},
        78 => FinishRecValue {primary_rec:Some(vec![T(18),D(12)]), secondary_rec: None},
        79 => FinishRecValue {primary_rec:Some(vec![T(19),D(11)]), secondary_rec: Some(vec![T(13), D(20)])},
        80 => FinishRecValue {primary_rec:Some(vec![T(20),D(10)]), secondary_rec: Some(vec![D(20), D(20)])},
        81 => FinishRecValue {primary_rec:Some(vec![T(19),D(12)]), secondary_rec: Some(vec![T(15), D(18)])},
        82 => FinishRecValue {primary_rec:Some(vec![T(14),D(20)]), secondary_rec: Some(vec![S(50), D(16)])},
        83 => FinishRecValue {primary_rec:Some(vec![T(17),D(16)]), secondary_rec: Some(vec![S(50), S(18), D(20)])},
        84 => FinishRecValue {primary_rec:Some(vec![T(20),D(12)]), secondary_rec: Some(vec![T(16), D(18)])},
        85 => FinishRecValue {primary_rec:Some(vec![T(15),D(20)]), secondary_rec: Some(vec![T(19), D(14)])},
        86 => FinishRecValue {primary_rec:Some(vec![T(18),D(16)]), secondary_rec: None},
        87 => FinishRecValue {primary_rec:Some(vec![T(17),D(18)]), secondary_rec: None},
        88 => FinishRecValue {primary_rec:Some(vec![T(20),D(14)]), secondary_rec: None},
        89 => FinishRecValue {primary_rec:Some(vec![T(19),D(16)]), secondary_rec: None},
        90 => FinishRecValue {primary_rec:Some(vec![T(20),D(15)]), secondary_rec: Some(vec![S(50), D(20)])},
        _ => panic!("Todo: Unreachable state {remaining_val}")
    }
}

fn under_40_two_dart_finish(remaining_val: u16) -> ((u16, u16), Option<(u16, u16)>) {
    match  remaining_val {
        3 => ((1, 1), None),
        39|37|35|33 => ((remaining_val - 32, 16), Some((remaining_val - 20, 10))),
        remaining_val if remaining_val > 4 && remaining_val < 8 => ((remaining_val - 4, 2), None),
        remaining_val if remaining_val > 8 && remaining_val < 12 => ((remaining_val - 8, 4), None),
        remaining_val if remaining_val > 12 && remaining_val < 16 => ((remaining_val - 8, 4), Some((remaining_val - 10, 5))),
        remaining_val if remaining_val > 16 && remaining_val < 22 => ((remaining_val - 16, 8), Some((remaining_val - 10, 5))),
        remaining_val if remaining_val > 22 && remaining_val < 32 => ((remaining_val - 16, 8), Some((remaining_val - 10, 5))),
        _ => panic!("Todo: Unreachable state {remaining_val}")
    }
}


#[cfg(test)]
mod test {
    use crate::components::calculation::recommendation_calculation::{determine_rec, RecValue};

    #[test]
    fn no_panic_due_unreachable_state() {
        (0..501).into_iter().for_each(|input_val| {determine_rec(input_val as u16);})
    }

    #[test]
    fn expect_remaining() {
        let result: Vec<RecValue> = (0..90).into_iter().map(|input_val| determine_rec(input_val as u16)).collect();
        for rec_value in result {
            match rec_value {
                RecValue::IsFinish(is_finish) => {
                    is_finish.primary_rec
                }
            }
        }

    }

}

