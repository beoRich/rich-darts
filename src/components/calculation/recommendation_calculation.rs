use dioxus::core_macro::Props;
use serde::{Deserialize, Serialize};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum RecValue {
    IsFinish(FinishRecValue),
    NoFinish(NonFinishRecValue)

}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct NonFinishRecValue {
    pub rec: String,
    pub goal: u16
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct FinishRecValue {
    pub primary_rec: String,
    pub secondary_rec: Option<String>,
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
        0 => FinishRecValue {
            primary_rec: "-".to_string(),
            secondary_rec: None,
        },
        remaining_val if remaining_val <= 40 && remaining_val % 2 == 0 => FinishRecValue {
            primary_rec: format!("D{}", remaining_val / 2),
            secondary_rec: None,
        },
        remaining_val if remaining_val <= 40 && remaining_val % 2 == 1 => {
            let ((single, double), secondary_ref) = under_40_two_dart_finish(remaining_val);
            FinishRecValue {
                primary_rec: format!("S{single} D{double}"),
                secondary_rec: secondary_ref.map(|(single, double)| format!("S{single} D{double}")),
            }
        },
        remaining_val if remaining_val <= 56 && remaining_val > 40 => {
            let (single, double) = (remaining_val - 32, 16);
            let (single_snd, double_snd) = (remaining_val - 40, 20);
            FinishRecValue {
                primary_rec: format!("S{single} D{double}"),
                secondary_rec: Some(format!("S{single_snd} D{double_snd}"))
            }
        },
        remaining_val if remaining_val <= 60 && remaining_val > 56  => {
            let (single, double) = (remaining_val - 40, 20);
            FinishRecValue {
                primary_rec: format!("S{single} D{double}"),
                secondary_rec: None
            }
        },
        61 => FinishRecValue {primary_rec: "S25 D18".to_string(), secondary_rec: Some("T15 D8".to_string())},
        62 => FinishRecValue {primary_rec: "T10 D16".to_string(), secondary_rec: Some("T14 D10".to_string())},
        63 => FinishRecValue {primary_rec: "T13 D12".to_string(), secondary_rec: None},
        64 => FinishRecValue {primary_rec: "T16 D8".to_string(), secondary_rec: Some("T8 D20".to_string())},
        65 => FinishRecValue {primary_rec: "S25 D20".to_string(), secondary_rec: Some("T11 D16".to_string())},
        66 => FinishRecValue {primary_rec: "T10 D18".to_string(), secondary_rec: Some("T18 D6".to_string())},
        67 => FinishRecValue {primary_rec: "T9 D20".to_string(), secondary_rec: None},
        68 => FinishRecValue {primary_rec: "T20 D4".to_string(), secondary_rec: Some("T16 D10".to_string())},
        69 => FinishRecValue {primary_rec: "T11 D18".to_string(), secondary_rec: Some("T15 D12".to_string())},
        70 => FinishRecValue {primary_rec: "T18 D8".to_string(), secondary_rec: Some("T10 D20".to_string())},
        71 => FinishRecValue {primary_rec: "T13 D16".to_string(), secondary_rec: Some("T17 D10".to_string())},
        72 => FinishRecValue {primary_rec: "T20 D6".to_string(), secondary_rec: Some("T12 D18".to_string())},
        73 => FinishRecValue {primary_rec: "T19 D8".to_string(), secondary_rec: None},
        74 => FinishRecValue {primary_rec: "T14 D16".to_string(), secondary_rec: Some("T16 D13".to_string())},
        75 => FinishRecValue {primary_rec: "T17 D12".to_string(), secondary_rec: Some("S25 S50".to_string())},
        76 => FinishRecValue {primary_rec: "T16 D14".to_string(), secondary_rec: None},
        77 => FinishRecValue {primary_rec: "T19 D10".to_string(), secondary_rec: None},
        78 => FinishRecValue {primary_rec: "T18 D12".to_string(), secondary_rec: None},
        79 => FinishRecValue {primary_rec: "T19 D11".to_string(), secondary_rec: Some("T13 D20".to_string())},
        80 => FinishRecValue {primary_rec: "T20 D10".to_string(), secondary_rec: Some("D20 D20".to_string())},
        81 => FinishRecValue {primary_rec: "T19 D12".to_string(), secondary_rec: Some("T15 D18".to_string())},
        82 => FinishRecValue {primary_rec: "T14 D20".to_string(), secondary_rec: Some("S50 D16".to_string())},
        83 => FinishRecValue {primary_rec: "T17 D16".to_string(), secondary_rec: Some("S50 S18 D20".to_string())},
        84 => FinishRecValue {primary_rec: "T20 D12".to_string(), secondary_rec: Some("T16 D18".to_string())},
        85 => FinishRecValue {primary_rec: "T15 D20".to_string(), secondary_rec: Some("T19 D14".to_string())},
        86 => FinishRecValue {primary_rec: "T18 D16".to_string(), secondary_rec: None},
        87 => FinishRecValue {primary_rec: "T17 D18".to_string(), secondary_rec: None},
        88 => FinishRecValue {primary_rec: "T20 D14".to_string(), secondary_rec: None},
        89 => FinishRecValue {primary_rec: "T19 D16".to_string(), secondary_rec: None},
        90 => FinishRecValue {primary_rec: "T20 D15".to_string(), secondary_rec: Some("S50 D20".to_string())},
        _ => panic!("Todo: Unreachable state {remaining_val}")
    }
}

fn under_40_two_dart_finish(remaining_val: u16) -> ((u16, u16), Option<(u16, u16)>) {
    match  remaining_val {
        3 => ((1, 1), None),
        39|37|35|33 => ((remaining_val - 32, 16), Some((remaining_val - 20, 10))),
        remaining_val if remaining_val < 8 && remaining_val > 4 => ((remaining_val - 4, 2), None),
        remaining_val if remaining_val < 16 && remaining_val < 10 => ((remaining_val - 8, 4), None),
        remaining_val if remaining_val < 16 && remaining_val > 10 => ((remaining_val - 8, 4), Some((remaining_val - 10, 5))),
        remaining_val if remaining_val < 32 && remaining_val < 20 => ((remaining_val - 16, 8), Some((remaining_val - 10, 5))),
        remaining_val if remaining_val < 32 && remaining_val > 20 => ((remaining_val - 16, 8), Some((remaining_val - 20, 10))),
        _ => panic!("Todo: Unreachable state {remaining_val}")
    }
}