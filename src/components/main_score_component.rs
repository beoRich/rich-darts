use crate::components::calculations;
use crate::components::enter_panel::EnterPanel;
use crate::components::score_display::ScoreDisplay;
use crate::domain::ErrorMessageMode::{CreateNewLeg, TechnicalError};
use crate::domain::ScoreMessageMode::{LegFinished, NewShot, UndoLastShot};
use crate::domain::{ErrorMessageMode, Leg, Score, ScoreMessageMode, INIT_SCORE};
use crate::{backend, Route};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_logger::tracing::error;
use crate::backend::leg_exists;

#[component]
pub fn MainScoreComponent(set_signal: Signal<u16>, leg_signal: Signal<u16>) -> Element {
    let mut raw_input = use_signal(|| "".to_string());
    let mut scores = use_signal(|| vec![]);

    let mut score_message = use_signal(|| NewShot);
    let mut error_message = use_signal(|| ErrorMessageMode::None);

    let mut allow_score = use_signal(|| true);

    use_memo(move || {
        let allow: bool = { score_message }.read().to_owned() != LegFinished && { error_message }
            .read()
            .to_owned()
            != CreateNewLeg;
        allow_score.set(allow)
    });

    use_resource(move || async move {
        let init_score_val = backend::list_score(leg_signal()).await;
        match init_score_val {
            Ok(val) if !val.is_empty() => scores.set(val),
            _ => error_message.set(CreateNewLeg),
        };
    });

    rsx! {
        div {
            id: "All",
            class: "container-self",


        div {
          class:"breadcrumbs text-sm",
          ul {
                    li {
                        "todo"
                        //Link {to: Route::DisplayLegs {}, class:"text-xl", "Leg"}
                    },
                    li {
                        "todo"
                        //Link {to: Route::ManualLeg {legval: leg_signal()}, class:"text-xl", {leg_signal().to_string()}}
                    }
                },
        }

            EnterPanel {scores, raw_input, set_signal, leg_signal, error_message, score_message, allow_score}
            ScoreDisplay {scores}
        }
    }
}

pub async fn input_wrapper(
    mut raw_input: Signal<String>,
    leg_signal: Signal<u16>,
    score: Signal<Vec<Score>>,
    mut error_message: Signal<ErrorMessageMode>,
    score_message: Signal<ScoreMessageMode>,
) {
    let (error_message_mode) = input_changed(leg_signal, score, raw_input, score_message).await;
    if error_message_mode == ErrorMessageMode::None {
        document::eval(&"document.getElementById('numberField').value = ' '".to_string());
        raw_input.set(" ".to_string());
    }
    error_message.set(error_message_mode);
    document::eval(&"document.getElementById('numberField').select()".to_string());
}

pub fn undo_wrapper(
    score: Signal<Vec<Score>>,
    error_message: Signal<ErrorMessageMode>,
    score_message: Signal<ScoreMessageMode>,
) {
    let last_score = undo_last_score(score, error_message, score_message);
    document::eval(&format!(
        "document.getElementById('numberField').value = '{last_score}'"
    ));
    document::eval(&"document.getElementById('numberField').select()".to_string());
}


pub async fn new_leg_wrapper(
    set_val: u16,
    leg_signal: Signal<u16>,
    score: Signal<Vec<Score>>,
    error_message: Signal<ErrorMessageMode>,
    score_message: Signal<ScoreMessageMode>,
) {
    new_leg(set_val, leg_signal, score, error_message, score_message).await;
    document::eval(&"document.getElementById('numberField').value = ' '".to_string());
    document::eval(&"document.getElementById('numberField').select()".to_string());
}

async fn new_leg(
    set_val: u16,
    mut leg_signal: Signal<u16>,
    mut score: Signal<Vec<Score>>,
    mut error_message: Signal<ErrorMessageMode>,
    mut score_message: Signal<ScoreMessageMode>,
) {
    error_message.set(ErrorMessageMode::None);
    score_message.set(NewShot);
    score.write().clear();

    let new_leg = backend::new_leg_init_score(set_val as i32).await;

    if new_leg.is_err() {
        error_message.set(TechnicalError);
    } else {
        leg_signal.set(new_leg.unwrap().id);
        score.set(vec![INIT_SCORE]);
    }
}

fn undo_last_score(
    mut score: Signal<Vec<Score>>,
    mut error_message: Signal<ErrorMessageMode>,
    mut score_message: Signal<ScoreMessageMode>,
) -> u16 {
    error_message.set(ErrorMessageMode::None);
    let generational_ref = score.read();
    let last_score = generational_ref.last();
    match last_score {
        Some(val) => {
            let last_thrown = val.thrown;
            score_message.set(UndoLastShot {
                last_score: last_thrown,
            });
            last_thrown
        }
        None => 0,
    }
}

async fn input_changed(
    mut leg_signal: Signal<u16>,
    mut score: Signal<Vec<Score>>,
    input_ref: Signal<String>,
    mut score_message: Signal<ScoreMessageMode>,
) -> ErrorMessageMode {
    let score_message_mode = score_message();
    let result = input_ref.read().parse();
    let leg_val = leg_signal();
    match result {
        Ok(val) => {
            if calculations::valid_thrown(val) {
                let mut last = get_last(&mut score);
                let next_throw_order: u16;
                {
                    match score_message_mode {
                        UndoLastShot { last_score: _ } => {
                            score.write().pop();
                            score_message.set(NewShot);
                            next_throw_order = last.throw_order;
                            let db_op_res =
                                backend::delete_score_by_order(leg_val, next_throw_order).await;
                            if db_op_res.is_err() {
                                //todo error conversion between db_op_res ServerFnError -> TechnicalError
                                return ErrorMessageMode::TechnicalError;
                            }
                            last = get_snd_last(&mut score);
                        }
                        NewShot => {
                            next_throw_order = last.throw_order + 1;
                        }
                        LegFinished => return (ErrorMessageMode::LegAlreadyFinished),
                    }
                }
                let new_score = calculations::calculate_remaining(last, val, next_throw_order);
                let db_op_res = backend::save_score(leg_val, new_score.clone()).await;
                if db_op_res.is_ok() {
                    if (&new_score).remaining == 0 {
                        score_message.set(LegFinished)
                    }
                    score.write().push(new_score);
                    return ErrorMessageMode::None;
                }
                //todo error conversion between db_op_res ServerFnError -> TechnicalError
                ErrorMessageMode::TechnicalError
            } else {
                ErrorMessageMode::NotADartsNumber
            }
        }
        Err(_) => ErrorMessageMode::NotANumber,
    }
}

fn get_last(score: &mut Signal<Vec<Score>>) -> Score {
    score.read().last().unwrap().to_owned()
}

fn get_snd_last(score: &mut Signal<Vec<Score>>) -> Score {
    let generational_ref = score.read();
    generational_ref
        .get(generational_ref.len() - 1)
        .unwrap()
        .to_owned()
}

fn handle_last() {}
