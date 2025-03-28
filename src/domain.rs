use dioxus::core_macro::Props;
use dioxus::prelude::*;
use dioxus::prelude::server_fn::serde::Deserialize;
use serde::Serialize;

pub const INIT_SCORE: Score = Score {
    remaining: 501,
    thrown: 0,
    throw_order: 0,
};

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Score {
    pub remaining: u16,
    pub thrown: u16,
    pub throw_order: u16
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Leg {
    pub id: u16,
    pub status: String,
}

#[derive(Props, PartialEq, Clone)]
pub struct ScoreMessage {
    pub score_message_mode: ScoreMessageMode,
    pub score_message_label: u16,
}

#[derive(Clone, PartialEq)]
pub enum ScoreMessageMode {
    NewShot,
    UndoLastShot { last_score: u16 },
    LegFinished,
}

impl ScoreMessageMode {
    pub fn value(&self) -> String {
        match self {
            ScoreMessageMode::NewShot => "Enter Shot".to_string(),
            ScoreMessageMode::UndoLastShot { last_score } => format!(
                "{} {}",
                "Correct last Shot: ".to_string(),
                last_score.to_string()
            ),
            ScoreMessageMode::LegFinished => "Leg finished".to_string(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ErrorMessageMode {
    None,
    NotANumber,
    NotADartsNumber,
    LegAlreadyFinished,
    CreateNewLeg,
    TechnicalError,
}

impl ErrorMessageMode {
    pub fn value(&self) -> Option<String> {
        match self {
            ErrorMessageMode::None => None,
            ErrorMessageMode::NotADartsNumber => Some("Not a valid Darts number".to_string()),
            ErrorMessageMode::NotANumber => Some("Not a number".to_string()),
            ErrorMessageMode::LegAlreadyFinished => Some("Leg already finished".to_string()),
            ErrorMessageMode::TechnicalError => Some("Technical error".to_string()),
            ErrorMessageMode::CreateNewLeg => Some("Create a new  leg".to_string())
        }
    }
}
