use crate::backend;
use crate::components::calculations;
use crate::domain::ErrorMessageMode::TechnicalError;
use crate::domain::LegStatus::Ongoing;
use crate::domain::ScoreMessageMode::{NewShot, UndoLastShot};
use crate::domain::{
    ErrorMessageMode, IdOrder, Leg, LegStatus, Score, ScoreMessageMode, Set, SetStatus, INIT_SCORE,
};
use dioxus::prelude::*;
#[component]
pub fn NumberFieldError(
    scores: Signal<Vec<Score>>,
    mut raw_input: Signal<String>,
    set_signal: Signal<Set>,
    leg_signal: Signal<Leg>,
    mut error_message: Signal<ErrorMessageMode>,
    score_message: Signal<ScoreMessageMode>,
    allow_score: Signal<bool>,
) -> Element {
    rsx! {
        div {
            id: "NumberFieldError",
            class: "mb-4",
            label {
                class: "block text-gray-700 text-xl text-primary font-bold mb-2",
                r#for: "numberField",
                {score_message.read().display()}
            }
            div {
                class: "grid grid-cols-12 gap-4",
                margin: "auto",
                input {
                    id: "numberField",
                    autofocus: true,
                    class: "text-2xl shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline",
                    r#type: "number",
                    maxlength: 10,
                    min: 0,
                    oninput: move |e| raw_input.set(e.value()),
                    onfocusin: move |_| {
                        document::eval(&"document.getElementById('numberField').select()".to_string());
                    },
                    onkeypress: move |e| async move {
                        let key = e.key();
                        if key == Key::Enter && allow_score() {
                            input_wrapper(
                                    raw_input,
                                    set_signal,
                                    leg_signal,
                                    scores,
                                    error_message,
                                    score_message,
                                )
                                .await;
                        } else if key == Key::Home {
                            undo_wrapper(scores, error_message, score_message);
                        }
                    },
                
                }
                div {
                    id: "displayError",
                    if error_message.read().value().is_some() {
                        p {
                            class: "text-l text-error",
                            {error_message.read().value()}
                        }
                    }
                }
            
            }
        
        }
    }
}
#[component]
pub fn Buttons(
    scores: Signal<Vec<Score>>,
    mut raw_input: Signal<String>,
    set_signal: Signal<Set>,
    leg_signal: Signal<Leg>,
    mut error_message: Signal<ErrorMessageMode>,
    score_message: Signal<ScoreMessageMode>,
    allow_score: Signal<bool>,
) -> Element {
    rsx! {
        div {
            id: "ButtonsDiv",
            class: "grid grid-cols-12 gap-4",
            div {
                class: "col-span-1 grid ",
                button {
                    id: "confirmButton",
                    onclick: move |_| async move {
                        input_wrapper(
                                raw_input,
                                set_signal,
                                leg_signal,
                                scores,
                                error_message,
                                score_message,
                            )
                            .await;
                    },
                    disabled: if !allow_score() { true },
                    class: "btn btn-soft btn-primary",
                    "Ok"
                }
            }
            div {
                class: "col-span-1 grid ",
                button {
                    id: "undoButton",
                    onclick: move |_| {
                        undo_wrapper(scores, error_message, score_message);
                    },
                    disabled: if scores.read().len() < 2 { true },
                    class: "btn btn-soft btn-secondary",
                    "Undo"
                }
            }
            div {
                class: "col-span-9 grid grid-cols-subgrid gap-4",
                div {
                    class: "col-start-9",
                    button {
                        id: "newLegButton",
                        onclick: move |_| async move {
                            new_leg(set_signal().id, leg_signal, scores, error_message, score_message).await;
                        },
                        title: "Cancel current leg (if unfinished) and start a new one",
                        class: "btn btn-soft btn-primary",
                        disabled: if !score_message().allow_new_leg() { true },
                        "New Leg"
                    }
                
                }
            }
            div {
                class: "col-span-1 grid grid-cols-subgrid gap-4",
                div {
                    class: "col-start-11",
                    button {
                        id: "cancelLegButton",
                        onclick: move |_| async move {
                            cancel_leg(leg_signal().id, error_message, score_message).await;
                        },
                        title: "Cancel current leg",
                        class: "btn btn-soft btn-secondary",
                        "Cancel"
                    }
                
                }
            }
        }
    }
}
async fn input_wrapper(
    mut raw_input: Signal<String>,
    set_signal: Signal<Set>,
    leg_signal: Signal<Leg>,
    score: Signal<Vec<Score>>,
    mut error_message: Signal<ErrorMessageMode>,
    score_message: Signal<ScoreMessageMode>,
) {
    let (error_message_mode) =
        input_changed(leg_signal, score, raw_input, score_message, &set_signal()).await;
    if error_message_mode == ErrorMessageMode::None {
        document::eval(&"document.getElementById('numberField').value = ' '".to_string());
        raw_input.set(" ".to_string());
    }
    error_message.set(error_message_mode);
    document::eval(&"document.getElementById('numberField').select()".to_string());
}
fn undo_wrapper(
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
async fn new_leg(
    set_val: u16,
    mut leg_signal: Signal<Leg>,
    mut score: Signal<Vec<Score>>,
    mut error_message: Signal<ErrorMessageMode>,
    mut score_message: Signal<ScoreMessageMode>,
) {
    error_message.set(ErrorMessageMode::None);
    score_message.set(NewShot);
    let last_score_val = score().last().map(|score| score.remaining);
    score.write().clear();
    let start_score = leg_signal().start_score;
    match last_score_val {
        Some(val) if val > 0 => {
            let _ =
                backend::api::dart_leg::update_leg_status(leg_signal().id, LegStatus::Cancelled)
                    .await;
        }
        _ => {}
    }
    let new_leg_res = backend::api::dart_leg::new_leg_init_score(set_val, start_score).await;
    match new_leg_res {
        Ok(new_leg) => {
            leg_signal.set(new_leg);
            score.set(vec![INIT_SCORE]);
            document::eval(&"document.getElementById('numberField').value = ' '".to_string());
            document::eval(&"document.getElementById('numberField').select()".to_string());
        }
        _ => error_message.set(TechnicalError),
    }
}
async fn cancel_leg(
    leg_id: u16,
    mut error_message: Signal<ErrorMessageMode>,
    mut score_message: Signal<ScoreMessageMode>,
) {
    let cancel_leg_res =
        backend::api::dart_leg::update_leg_status(leg_id, LegStatus::Cancelled).await;
    match cancel_leg_res {
        Ok(_) => {
            error_message.set(ErrorMessageMode::None);
            score_message.set(ScoreMessageMode::LegCancelled);
            document::eval(&"document.getElementById('numberField').value = ' '".to_string());
            document::eval(&"document.getElementById('numberField').select()".to_string());
        }
        _ => error_message.set(TechnicalError),
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
    mut leg_signal: Signal<Leg>,
    mut score: Signal<Vec<Score>>,
    input_ref: Signal<String>,
    mut score_message: Signal<ScoreMessageMode>,
    current_set: &Set,
) -> ErrorMessageMode {
    let score_message_mode = score_message();
    let result = input_ref.read().parse();
    let leg_val = leg_signal();
    match result {
        Ok(val) => {
            if calculations::valid_thrown(val) {
                {
                    if let Ok((last, next_throw_order)) = handle_score_message_mode(
                        &mut score,
                        &mut score_message,
                        score_message_mode,
                        &leg_val,
                        &current_set,
                    )
                    .await
                    {
                        let new_score =
                            calculations::calculate_remaining(last, val, next_throw_order);
                        match handle_new_score(
                            &mut score,
                            &mut score_message,
                            leg_val,
                            &new_score,
                            current_set,
                        )
                        .await
                        {
                            Ok(_) => ErrorMessageMode::None,
                            Err(value) => TechnicalError,
                        }
                    } else {
                        TechnicalError
                    }
                }
            } else {
                ErrorMessageMode::NotADartsNumber
            }
        }
        Err(_) => ErrorMessageMode::NotANumber,
    }
}
async fn handle_score_message_mode(
    mut score: &mut Signal<Vec<Score>>,
    score_message: &mut Signal<ScoreMessageMode>,
    score_message_mode: ScoreMessageMode,
    leg_val: &Leg,
    current_set: &Set,
) -> Result<(Score, u16), ServerFnError> {
    let last = get_last(&mut score);
    match score_message_mode {
        UndoLastShot { last_score: _ } => {
            score.write().pop();
            score_message.set(NewShot);
            let next_throw_order = last.throw_order;
            let _ = backend::api::dart_score::delete_score_by_order(leg_val.id, next_throw_order)
                .await?;
            let _ = backend::api::dart_set::update_set_status(current_set.id, SetStatus::Ongoing)
                .await?;
            let _ = backend::api::dart_leg::update_leg_status(leg_val.id, Ongoing).await?;
            let last = get_snd_last(&mut score);
            Ok((last, next_throw_order))
        }
        NewShot => {
            let next_throw_order = last.throw_order + 1;
            Ok((last, next_throw_order))
        }
        _ => Err(ServerFnError::new("help")),
    }
}
async fn handle_new_score(
    score: &mut Signal<Vec<Score>>,
    score_message: &mut Signal<ScoreMessageMode>,
    leg_val: Leg,
    new_score: &Score,
    current_set: &Set,
) -> Result<(), ServerFnError> {
    backend::api::dart_score::save_score(leg_val.id, new_score.clone()).await?;
    if (&new_score).remaining == 0 {
        backend::api::dart_leg::update_leg_status(leg_val.id, LegStatus::Finished).await?;
        score_message.set(ScoreMessageMode::LegFinished);
        if leg_val.leg_order == current_set.leg_amount {
            backend::api::dart_set::update_set_status(current_set.id, SetStatus::Finished).await?;
            score_message.set(ScoreMessageMode::SetFinished);
        }
    }
    score.write().push(new_score.clone());
    Ok(())
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
