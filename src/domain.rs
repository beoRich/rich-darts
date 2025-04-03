use dioxus::core_macro::Props;
use dioxus::prelude::server_fn::serde::Deserialize;
use dioxus::prelude::*;
use serde::Serialize;

pub const INIT_SCORE: Score = Score {
    remaining: 501,
    thrown: 0,
    throw_order: 0,
};

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize, Copy)]
pub struct IdOrder {
    pub id: u16,
    pub order: u16,
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize, Copy)]
pub struct IdOrderParent {
    pub id: u16,
    pub order: u16,
    pub parent_id: u16,
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Score {
    pub remaining: u16,
    pub thrown: u16,
    pub throw_order: u16,
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Leg {
    pub id: u16,
    pub leg_order: u16,
    pub start_score: u16,
    pub status: String,
    pub last_score: Option<u16>,
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Set {
    pub id: u16,
    pub set_order: u16,
    pub status: String,
    pub best_of: u16,
    pub leg_amount: u16,
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Match {
    pub id: u16,
    pub status: String,
}

#[derive(Props, PartialEq, Clone)]
pub struct ScoreMessage {
    pub score_message_mode: ScoreMessageMode,
    pub score_message_label: u16,
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize, Debug)]
pub enum LegStatus {
    Ongoing,
    Finished,
    Cancelled,
    Future,
}

impl LegStatus {
    pub fn display(&self) -> String {
        match self {
            LegStatus::Finished => "Leg finished".to_string(),
            LegStatus::Ongoing => "Ongoing".to_string(),
            LegStatus::Cancelled => "Leg cancelled".to_string(),
            LegStatus::Future => "Not started yet".to_string(),
        }
    }

    pub fn count_towards_leg_amount(&self) -> bool {
        match self {
            LegStatus::Cancelled => false,
            _ => true,
        }
    }
}

pub fn parse_leg_status(status_str: String) -> LegStatus {
    match status_str {
        s if s == LegStatus::Ongoing.display() => LegStatus::Ongoing,
        s if s == LegStatus::Cancelled.display() => LegStatus::Cancelled,
        s if s == LegStatus::Future.display() => LegStatus::Future,
        s if s == LegStatus::Finished.display() => LegStatus::Finished,
        _ => panic!("Unknown leg status {:?}", status_str),
    }
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize, Debug)]
pub enum SetStatus {
    Ongoing,
    Finished,
    Cancelled,
}

impl SetStatus {
    pub fn value(&self) -> String {
        match self {
            SetStatus::Finished => "Set finished".to_string(),
            SetStatus::Ongoing => "Ongoing".to_string(),
            SetStatus::Cancelled => "Set cancelled".to_string(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ScoreMessageMode {
    NewShot,
    UndoLastShot { last_score: u16 },
    LegFinished,
    LegCancelled,
    SetFinished,
}

impl ScoreMessageMode {
    pub fn display(&self) -> String {
        match self {
            ScoreMessageMode::NewShot => "Enter Shot".to_string(),
            ScoreMessageMode::UndoLastShot { last_score } => format!(
                "{} {}",
                "Correct last Shot: ".to_string(),
                last_score.to_string()
            ),
            ScoreMessageMode::LegFinished => "Leg finished".to_string(),
            ScoreMessageMode::LegCancelled => "Leg cancelled".to_string(),
            ScoreMessageMode::SetFinished => "Set finished".to_string(),
        }
    }

    pub fn allow_score(&self) -> bool {
        match self {
            ScoreMessageMode::NewShot => true,
            ScoreMessageMode::UndoLastShot { last_score: _ } => true,
            _ => false,
        }
    }

    pub fn allow_new_leg(&self) -> bool {
        match self {
            ScoreMessageMode::LegFinished => true,
            ScoreMessageMode::LegCancelled => true,
            _ => false,
        }
    }
}
pub fn parse_score_message(status_str: String) -> ScoreMessageMode {
    match status_str {
        s if s == ScoreMessageMode::LegFinished.display() => ScoreMessageMode::LegFinished,
        s if s == ScoreMessageMode::LegCancelled.display() => ScoreMessageMode::LegCancelled,
        s if s == ScoreMessageMode::SetFinished.display() => ScoreMessageMode::SetFinished,
        _ => ScoreMessageMode::NewShot,
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
            ErrorMessageMode::NotADartsNumber => Some("Invalid Darts number".to_string()),
            ErrorMessageMode::NotANumber => Some("Not a number".to_string()),
            ErrorMessageMode::LegAlreadyFinished => Some("Leg already finished".to_string()),
            ErrorMessageMode::TechnicalError => Some("Technical error".to_string()),
            ErrorMessageMode::CreateNewLeg => Some("Create a new  leg".to_string()),
        }
    }

    pub fn allow_score(&self) -> bool {
        match self {
            ErrorMessageMode::None => true,
            ErrorMessageMode::NotADartsNumber => true,
            ErrorMessageMode::NotANumber => true,
            _ => false,
        }
    }
}
