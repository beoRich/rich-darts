use dioxus::prelude::*;
use dioxus::core_macro::Props;

#[derive(Props, PartialEq, Clone, Debug)]
pub struct CurrentScore {
    pub remaining: u16,
    pub thrown: u16,
}

#[derive(Props, PartialEq, Clone)]
pub struct ScoreMessage {
    pub score_message_mode: ScoreMessageMode,
    pub score_message_label: u16,
}

#[derive(Clone, PartialEq)]
pub enum ScoreMessageMode {
    NewShot,
    UndoLastShot { last_score: u16},
    GameFinished
}

impl ScoreMessageMode {
    pub fn value(&self) -> String {
        match self {
            ScoreMessageMode::NewShot => "Enter Shot".to_string(),
            ScoreMessageMode::UndoLastShot {last_score} => format!("{} {}", "Correct last Shot: ".to_string(), last_score.to_string()),
            ScoreMessageMode::GameFinished => "Leg finished".to_string(),
        }

    }
}