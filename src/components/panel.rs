use crate::components::calculations;
use crate::domain::ScoreMessageMode::{GameFinished, NewShot, UndoLastShot};
use crate::domain::{CurrentScore, ErrorMessageMode, ScoreMessageMode, INIT_SCORE};
use dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use dioxus::events::Key::New;
use dioxus::prelude::*;
use dioxus_elements::style;
use dioxus_logger::tracing;
use std::cell::Ref;
use std::num::ParseIntError;
use crate::backend;

#[component]
pub fn Panel() -> Element {
    let mut raw_input = use_signal(|| "".to_string());
    let init_count_vector = vec![INIT_SCORE];
    let mut count = use_signal(|| init_count_vector);
    let mut score_message = use_signal(|| NewShot);
    let mut error_message = use_signal(|| ErrorMessageMode::None);


    rsx! {
        div {
      id: "All",
            class: "container-self",

      div {
        id:"TopHalf",
        class:"bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 overflow-x-scroll",
        div {
            class:"mb-4",
                label {
                    class:"block text-gray-700 text-sm font-bold mb-2",
                    for: "numberField",
                    {score_message.read().value()}
                    }

            div {
                class:"grid grid-cols-10 gap-4",
                input {id: "numberField",
                        autofocus: true,
                    class:"shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline",
                    type: "number", maxlength:10, min:0, oninput: move |e| raw_input.set(e.value()),
                    onfocusin: move |_| {
                            document::eval(&"document.getElementById('numberField').select()".to_string());
                        },
                    onkeypress: move |e| async move {
                            let key = e.key();
                            if key == Key::Enter && {score_message}.read().to_owned() != GameFinished {
                                let option = input_wrapper(raw_input, count, error_message, score_message);
                                if option.is_some() {
                                    _ = backend::save_throw(1, option.unwrap()).await
                                }
                            } else if key == Key::Home  {
                                undo_wrapper(count, error_message, score_message);
                            };
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
        div {
            id: "ButtonsDiv",
            class:"grid grid-cols-10 gap-4",

                div {
                    class:"col-span-1 grid ",
                    button {id: "confirmButton",
                        onclick: move |_| async move {
                                let option = input_wrapper(raw_input, count, error_message, score_message);
                                if option.is_some() {
                                    _ = backend::save_throw(1, option.unwrap()).await
                                }

                        },
                        disabled: if {score_message}.read().to_owned() == GameFinished {true},
                        class:"btn btn-soft btn-primary" , "Ok" },
                }

                div {
                    class:"col-span-1 grid ",
                    button {id: "undoButton",
                        onclick: move |_| {
                                undo_wrapper(count, error_message, score_message);
                        },
                        disabled: if count.read().len() < 2 {true},
                        class:"btn btn-soft btn-secondary" , "Undo" },
                }

                div {
                    class:"col-span-8 grid grid-cols-subgrid gap-4",
                    div {
                        class:"col-start-8",
                        button {id: "newLegButton",
                            onclick: move |_| async move {
                                    new_leg(count, error_message, score_message);
                                    document::eval(&"document.getElementById('numberField').value = ' '".to_string());
                                    document::eval(&"document.getElementById('numberField').select()".to_string());
                                    let  _ = backend::save_throw(1, INIT_SCORE.clone()).await;
                            },
                            class:"btn btn-soft btn-info" , "New Leg" },
                    }
                }
        }


    }

      div {
            id:"BottomHalf",
            class:"bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 overflow-y-auto",
            div { id: "numbers",
                    class: "table-container",
                table {
                    thead {
                        class: "text-xs uppercase bg-neutral-content",
                        tr {
                            th {
                                scope:"col",
                                style:"white-space: pre; text-align: center;",
                                class:"text-primary px-6 py-3",
                                "Thrown"
                            },
                            th {
                                scope:"col",
                                style:"white-space: pre; text-align: center;",
                                class:"text-secondary px-6 py-3",
                                "Remaining"
                            }
                        }
                    }
                    tbody {
                        id: "numbers-body",
                        for (i, a) in count().into_iter().rev().enumerate() {
                            tr {
                                    td {
                                        class: if i == 0 {"px-6 py-4 bg-accent text-accent-content"},
                                        class: if i % 2 == 0 && i!=0 {"px-6 py-4 bg-base-200 text-base-content"},
                                        class: if i % 2 == 1 {"px-6 py-4 bg-base-300 text-base-content"},
                                        style:"white-space: pre; text-align: center;",
                                        {format!("{:>3}", a.thrown.to_string())}
                                    },
                                    td {
                                        class: if i == 0 {"px-6 py-4 bg-accent text-accent-content"},
                                        class: if i % 2 == 0 && i!=0 {"px-6 py-4 bg-base-200 text-base-content"},
                                        class: if i % 2 == 1 {"px-6 py-4 bg-base-300 text-base-content"},
                                        style:"white-space: pre; text-align: center;",
                                        {format!("{:>3}", a.remaining.to_string())}
                                    },
                            }
                        }
                    }

                }
            }
      }
    }
    }
}

fn input_wrapper(
    mut raw_input: Signal<String>,
    mut count: Signal<Vec<CurrentScore>>,
    mut error_message: Signal<ErrorMessageMode>,
    mut score_message: Signal<ScoreMessageMode>,
) -> Option<CurrentScore> {
    let (error_message_mode, score_message_maybe) = input_changed(count, raw_input, score_message);
    if error_message_mode == ErrorMessageMode::None {
        document::eval(&"document.getElementById('numberField').value = ' '".to_string());
        raw_input.set(" ".to_string());
    }
    error_message.set(error_message_mode);
    document::eval(&"document.getElementById('numberField').select()".to_string());
    score_message_maybe
}

fn undo_wrapper(
    mut count: Signal<Vec<CurrentScore>>,
    mut error_message: Signal<ErrorMessageMode>,
    mut score_message: Signal<ScoreMessageMode>,
) {
    let last_score = undo_last_score(count, error_message, score_message);
    document::eval(&format!(
        "document.getElementById('numberField').value = '{last_score}'"
    ));
    document::eval(&"document.getElementById('numberField').select()".to_string());
}

fn new_leg(
    mut count: Signal<Vec<CurrentScore>>,
    mut error_message: Signal<ErrorMessageMode>,
    mut score_message: Signal<ScoreMessageMode>,
) {
    error_message.set(ErrorMessageMode::None);
    score_message.set(NewShot);
    count.write().clear();
    count.push(INIT_SCORE);
}

fn undo_last_score(
    mut count: Signal<Vec<CurrentScore>>,
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

fn input_changed(
    mut count: Signal<Vec<CurrentScore>>,
    input_ref: Signal<String>,
    mut score_message: Signal<ScoreMessageMode>,
) -> (ErrorMessageMode, Option<CurrentScore>) {
    let score_message_mode = score_message();
    let result = input_ref.read().parse();
    match result {
        Ok(val) => {
            if calculations::valid_thrown(val) {
                let last = count.read().last().unwrap().to_owned();
                let next_throw_order: u16;
                {
                    match score_message_mode {
                        UndoLastShot { last_score: _ } => {
                            count.write().pop();
                            score_message.set(NewShot);
                            next_throw_order = last.throw_order;
                            //todo set all leg entries that are equal to next_throw_order deleted = true
                        }
                        NewShot => {
                            next_throw_order = last.throw_order + 1;
                        }
                        GameFinished => return (ErrorMessageMode::LegAlreadyFinished, None),
                    }
                }
                let new_score = calculations::calculate_remaining(last, val, next_throw_order);
                count.write().push(new_score.clone());
                let _ = async {
                    backend::save_throw(1, new_score.clone()).await.expect("TODO: panic message");
                };
                if new_score.remaining == 0 {
                    score_message.set(GameFinished)
                }
                (ErrorMessageMode::None, Some(new_score))
            } else {
                (ErrorMessageMode::NotADartsNumber, None)
            }
        }
        Err(_) => (ErrorMessageMode::NotANumber, None),
    }
}
fn handle_last() {

}