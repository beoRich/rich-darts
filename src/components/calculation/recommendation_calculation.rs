use crate::components::calculation::recommendation_calculation::IsFinish::Yes;

pub struct RecValue {
    is_finish: IsFinish,
    rec_string: String,
}

impl RecValue {
    pub fn display(&self) -> String {
        match self {
            RecValue {
                is_finish: IsFinish::Yes,
                rec_string,
            } => rec_string.to_string(),
            RecValue {
                is_finish: IsFinish::No { goal },
                rec_string,
            } => format!("{rec_string} -> {goal}"),
        }
    }
}

enum IsFinish {
    Yes,
    No { goal: u16 },
}

pub fn basic(remaining_val: u16) -> RecValue {
    match remaining_val {
        0 => RecValue{
            is_finish: Yes,
            rec_string: "-".to_string()
        },
        remaining_val if remaining_val <= 40 && remaining_val % 2 == 0 => RecValue {
            is_finish: Yes,
            rec_string: format!("D{}", remaining_val / 2),
        },
        remaining_val if remaining_val <= 40 && remaining_val % 2 == 1 => {
            let (single, double) = under_40_two_dart_finish(remaining_val);
            RecValue {
            is_finish: Yes,
            rec_string: format!("S{single} D{double}")}
        },
        remaining_val if remaining_val <= 56 && remaining_val > 40 => {
            let (single, double) = (remaining_val - 32, 16);
            RecValue {
                is_finish: Yes,
                rec_string: format!("S{single} D{double}"),
            }
        },
        remaining_val if remaining_val <= 60 && remaining_val > 56  => {
            let (single, double) = (remaining_val - 40, 20);
            RecValue {
                is_finish: Yes,
                rec_string: format!("S{single} D{double}"),
            }
        },
        61 => RecValue {is_finish: Yes, rec_string: "S25 D18".to_string()},
        62 => RecValue {is_finish: Yes, rec_string: "T10 D16".to_string()},
        63 => RecValue {is_finish: Yes, rec_string: "T13 D12".to_string()},
        64 => RecValue {is_finish: Yes, rec_string: "T16 D8".to_string()},
        65 => RecValue {is_finish: Yes, rec_string: "S25 D20".to_string()},
        66 => RecValue {is_finish: Yes, rec_string: "T10 D18".to_string()},
        67 => RecValue {is_finish: Yes, rec_string: "T9 D20".to_string()},
        68 => RecValue {is_finish: Yes, rec_string: "T20 D4".to_string()},
        69 => RecValue {is_finish: Yes, rec_string: "T11 D18".to_string()},
        70 => RecValue {is_finish: Yes, rec_string: "T18 D8".to_string()},
        71 => RecValue {is_finish: Yes, rec_string: "T13 D16".to_string()},
        72 => RecValue {is_finish: Yes, rec_string: "T20 D6".to_string()},
        73 => RecValue {is_finish: Yes, rec_string: "T19 D8".to_string()},
        74 => RecValue {is_finish: Yes, rec_string: "T14 D16".to_string()},
        75 => RecValue {is_finish: Yes, rec_string: "T17 D12".to_string()},
        76 => RecValue {is_finish: Yes, rec_string: "T16 D14".to_string()},
        77 => RecValue {is_finish: Yes, rec_string: "T19 D10".to_string()},
        78 => RecValue {is_finish: Yes, rec_string: "T18 D12".to_string()},
        79 => RecValue {is_finish: Yes, rec_string: "T19 D11".to_string()},
        80 => RecValue {is_finish: Yes, rec_string: "T20 D10".to_string()},
        81 => RecValue {is_finish: Yes, rec_string: "T19 D12".to_string()},
        82 => RecValue {is_finish: Yes, rec_string: "T14 D20".to_string()},
        83 => RecValue {is_finish: Yes, rec_string: "T17 D16".to_string()},
        84 => RecValue {is_finish: Yes, rec_string: "T20 D12".to_string()},
        85 => RecValue {is_finish: Yes, rec_string: "T15 D20".to_string()},
        86 => RecValue {is_finish: Yes, rec_string: "T18 D16".to_string()},
        87 => RecValue {is_finish: Yes, rec_string: "T17 D18".to_string()},
        88 => RecValue {is_finish: Yes, rec_string: "T20 D14".to_string()},
        89 => RecValue {is_finish: Yes, rec_string: "T19 D16".to_string()},
        90 => RecValue {is_finish: Yes, rec_string: "T20 D15".to_string()},
        remaining_val if remaining_val > 180 => RecValue {
            is_finish: IsFinish::No {
                goal: remaining_val - 180,
            },
            rec_string: "D20 D20 D20".to_string(),
        },
        _ => RecValue {
            is_finish: IsFinish::No {
                goal: 32,
            },
            rec_string: format!("{}", remaining_val - 32),
        },
    }
}

fn under_40_two_dart_finish(remaining_val: u16) -> (u16, u16) {
    match  remaining_val {
        39|37|35|33 => (remaining_val - 32, 16),
        remaining_val if remaining_val < 32 && remaining_val > 16 => (remaining_val - 16, 8),
        remaining_val if remaining_val < 16 && remaining_val > 8 => (remaining_val - 8, 4),
        remaining_val if remaining_val < 8 && remaining_val > 4 => (remaining_val - 4, 2),
        3 => (1, 1),
        _ => panic!("Unreachable state {remaining_val}")
    }
}