use crate::{backend, Route};
use crate::components::calculations;
use crate::components::enter_panel::EnterPanel;
use crate::components::score_display::ScoreDisplay;
use crate::domain::ErrorMessageMode::CreateNewLeg;
use crate::domain::ScoreMessageMode::{LegFinished, NewShot, UndoLastShot};
use crate::domain::{Score, ErrorMessageMode, ScoreMessageMode, INIT_SCORE};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use std::num::ParseIntError;
use dioxus_logger::tracing::error;

#[component]
pub fn MainComponent(leg: Signal<u16>) -> Element {
    let mut raw_input = use_signal(|| "".to_string());
    let mut count = use_signal(|| vec![]);

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

    use_resource(move || {
        async move {
            let leg_val = leg();
            let leg_exists = backend::leg_exists(leg_val).await.unwrap();
            if !leg_exists {
                new_leg_wrapper(leg_val, leg, count, error_message, score_message).await;
            }
            let init_count_val = backend::list_score(leg()).await;
            if init_count_val.is_ok() && !init_count_val.clone().unwrap().is_empty() {
                count.set(init_count_val.unwrap());
            } else {
                error_message.set(CreateNewLeg);
            };
        }
    });

    rsx! {
        div {
            id: "All",
            class: "container-self",


        div {
          class:"breadcrumbs text-sm",
          ul {
                    li {
                        Link {to: Route::DisplayLegs {}, "Leg"}
                    },
                    li {
                        Link {to: Route::ManualLeg {legval: leg()}, {leg().to_string()}}
                    }
                },
        }

            EnterPanel {count, raw_input, leg, error_message, score_message, allow_score}
            ScoreDisplay {count}
        }
    }
}

pub async fn input_wrapper(
    mut raw_input: Signal<String>,
    leg: Signal<u16>,
    count: Signal<Vec<Score>>,
    mut error_message: Signal<ErrorMessageMode>,
    score_message: Signal<ScoreMessageMode>,
) {
    let (error_message_mode) = input_changed(leg, count, raw_input, score_message).await;
    if error_message_mode == ErrorMessageMode::None {
        document::eval(&"document.getElementById('numberField').value = ' '".to_string());
        raw_input.set(" ".to_string());
    }
    error_message.set(error_message_mode);
    document::eval(&"document.getElementById('numberField').select()".to_string());
}

pub fn undo_wrapper(
    count: Signal<Vec<Score>>,
    error_message: Signal<ErrorMessageMode>,
    score_message: Signal<ScoreMessageMode>,
) {
    let last_score = undo_last_score(count, error_message, score_message);
    document::eval(&format!(
        "document.getElementById('numberField').value = '{last_score}'"
    ));
    document::eval(&"document.getElementById('numberField').select()".to_string());
}

pub async fn new_leg_wrapper(
    leg_val: u16,
    leg: Signal<u16>,
    count: Signal<Vec<Score>>,
    error_message: Signal<ErrorMessageMode>,
    score_message: Signal<ScoreMessageMode>,
) {
    new_leg(leg_val, leg, count, error_message, score_message).await;
    document::eval(&"document.getElementById('numberField').value = ' '".to_string());
    document::eval(&"document.getElementById('numberField').select()".to_string());
}

async fn new_leg(
    leg_val : u16,
    mut leg: Signal<u16>,
    mut count: Signal<Vec<Score>>,
    mut error_message: Signal<ErrorMessageMode>,
    mut score_message: Signal<ScoreMessageMode>,
) {
    error_message.set(ErrorMessageMode::None);
    score_message.set(NewShot);
    count.write().clear();

    leg.set(leg_val);
    backend::save_leg(leg_val)
        .await
        .expect(&format!("Could not save leg {}", leg_val));
    let db_op_res = backend::save_throw(leg_val, INIT_SCORE).await;
    if db_op_res.is_ok() {
        count.set(vec![INIT_SCORE]);
    }
}

fn undo_last_score(
    mut count: Signal<Vec<Score>>,
    mut error_message: Signal<ErrorMessageMode>,
    mut score_message: Signal<ScoreMessageMode>,
) -> u16 {
    error_message.set(ErrorMessageMode::None);
    let generational_ref = count.read();
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
    mut leg: Signal<u16>,
    mut count: Signal<Vec<Score>>,
    input_ref: Signal<String>,
    mut score_message: Signal<ScoreMessageMode>,
) -> ErrorMessageMode {
    let score_message_mode = score_message();
    let result = input_ref.read().parse();
    let leg_val = leg();
    match result {
        Ok(val) => {
            if calculations::valid_thrown(val) {
                let mut last = get_last(&mut count);
                let next_throw_order: u16;
                {
                    match score_message_mode {
                        UndoLastShot { last_score: _ } => {
                            count.write().pop();
                            score_message.set(NewShot);
                            next_throw_order = last.throw_order;
                            let db_op_res =
                                backend::delete_throw_by_order(leg_val, next_throw_order).await;
                            if db_op_res.is_err() {
                                //todo error conversion between db_op_res ServerFnError -> TechnicalError
                                return ErrorMessageMode::TechnicalError;
                            }
                            last = get_snd_last(&mut count);
                        }
                        NewShot => {
                            next_throw_order = last.throw_order + 1;
                        }
                        LegFinished => return (ErrorMessageMode::LegAlreadyFinished),
                    }
                }
                let new_score = calculations::calculate_remaining(last, val, next_throw_order);
                let db_op_res = backend::save_throw(leg_val, new_score.clone()).await;
                if db_op_res.is_ok() {
                    if (&new_score).remaining == 0 {
                        score_message.set(LegFinished)
                    }
                    count.write().push(new_score);
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

fn get_last(count: &mut Signal<Vec<Score>>) -> Score {
    count.read().last().unwrap().to_owned()
}

fn get_snd_last(count: &mut Signal<Vec<Score>>) -> Score {
    let generational_ref = count.read();
    generational_ref
        .get(generational_ref.len() - 1)
        .unwrap()
        .to_owned()
}

fn handle_last() {}
