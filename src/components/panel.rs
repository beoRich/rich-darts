use dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use dioxus::prelude::*;
use dioxus_elements::style;
use dioxus_logger::tracing;
use std::cell::Ref;
use std::num::ParseIntError;
use dioxus::events::Key::New;
use crate::components::calculations;
use crate::components::domain::{CurrentScore, ScoreMessageMode};
use crate::components::domain::ScoreMessageMode::{GameFinished, NewScore, UndoLastScore};


#[component]
pub fn Panel() -> Element {
    let mut raw_input = use_signal(|| "".to_string());
    let init_current_score = CurrentScore {
        remaining: 501,
        thrown: 0,
    };
    let init_count_vector = vec![init_current_score];
    let mut count = use_signal(|| init_count_vector);
    let mut score_message = use_signal(|| NewScore);
    let is_wrong = use_signal(|| false);

    rsx! {
      div {
            class:"bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4",
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
                    onkeypress: move |e| {
                        if e.key() == Key::Enter && {score_message}.read().to_owned() != GameFinished {
                            input_changed(count, is_wrong, raw_input, score_message);
                            document::eval(&"document.getElementById('numberField').value = '0'".to_string());
                            document::eval(&"document.getElementById('numberField').select()".to_string());
                        }
                    }
                }
            }

        }

        div {
            class:"grid grid-cols-10 gap-4",

                div {
                    class:"col-span-1 grid ",
                    button {id: "confirmButton",
                        onclick: move |_| {
                                input_changed(count, is_wrong, raw_input, score_message);
                                document::eval(&"document.getElementById('numberField').value = '0'".to_string());
                                document::eval(&"document.getElementById('numberField').select()".to_string());
                        },
                        disabled: if {score_message}.read().to_owned() == GameFinished {true},
                        class:"btn btn-primary" , "Ok" },
                }

                div {
                    class:"col-span-1 grid ",
                    button {id: "undoButton",
                        onclick: move |_| {
                                let last_score = undo_last_score(count, is_wrong, score_message);
                                    document::eval(&format!(
                                    "document.getElementById('numberField').value = '{last_score}'"
                                 ));
                                document::eval(&"document.getElementById('numberField').select()".to_string());
                        },
                        disabled: if count.read().len() < 2 {true},
                        class:"btn btn-primary" , "Undo" },
                }

                div {
                    class:"col-span-8 grid grid-cols-subgrid gap-4",
                    div {
                        class:"col-start-8",
                        button {id: "newGameButton",
                            onclick: move |_| {
                                    new_game(count, is_wrong, score_message)
                            },
                            class:"btn btn-secondary" , "New Game" },
                    }
                }
        }


    }
        div {
            id: "displayError",
            if is_wrong() {
                p {
                class: "text-xs text-color-error",
                "Please enter a valid number" }
            }
        }

      div {
            class:"bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4",
            div { id: "numbers",
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
                        for a in count.iter() {
                            tr {
                                    class:"bg-white border-b dark:bg-white-800 dark:border-gray-700 border-gray-200",
                                    td {
                                        class:"px-6 py-4",
                                        style:"white-space: pre; text-align: center;",
                                        {format!("{:>3}", a.thrown.to_string())}
                                    },
                                    td {
                                        class:"px-6 py-4",
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

fn new_game(
    mut count: Signal<Vec<CurrentScore>>,
    mut is_wrong: Signal<bool>,
    mut score_message: Signal<ScoreMessageMode>
) {
    is_wrong.set(false);
    score_message.set(NewScore);
    count.write().clear();
    let init_score = CurrentScore {
        remaining: 501,
        thrown: 0,
    };
    count.push(init_score);
}

fn undo_last_score(
    mut count: Signal<Vec<CurrentScore>>,
    mut is_wrong: Signal<bool>,
    mut score_message: Signal<ScoreMessageMode>
) -> u16 {
    is_wrong.set(false);
    let generational_ref = count.read();
    let last_score = generational_ref.last();
    match last_score {
        Some(val) => {
            let last_thrown = val.thrown;
            score_message.set(UndoLastScore { last_score: last_thrown });
            last_thrown
        },
        None => {0}
    }
}

fn input_changed(
    mut count: Signal<Vec<CurrentScore>>,
    mut is_wrong: Signal<bool>,
    input_ref: Signal<String>,
    mut score_message: Signal<ScoreMessageMode>
) {
    let score_message_mode = score_message();
    match score_message_mode {
        UndoLastScore {last_score: _}=>  {
            count.write().pop();
            score_message.set(NewScore)
        },
        NewScore => {},
        GameFinished => {is_wrong.set(true)}
    }
    let result = input_ref.read().parse();
    match result {
        Ok(val) => {
            if val <= 180 {
                let new_score = calculations::calculate_remaining(&count.read().to_owned(), val);
                count.write().push(new_score.clone());
                if new_score.remaining == 0 {
                    score_message.set(GameFinished)
                }
                is_wrong.set(false)
            } else {
                is_wrong.set(true)
            }
        }
        Err(_) => is_wrong.set(true),
    }
}

