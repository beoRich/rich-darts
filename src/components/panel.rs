use dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use dioxus::prelude::*;
use dioxus_elements::style;
use dioxus_logger::tracing;
use std::cell::Ref;
use std::num::ParseIntError;
use crate::components::panel::ScoreMessageMode::{GameFinished, NewScore, ResetLastScore};

#[derive(Props, PartialEq, Clone)]
struct CurrentScore {
    remaining: u16,
    thrown: u16,
}


#[derive(Clone, PartialEq)]
enum ScoreMessageMode {
    NewScore,
    ResetLastScore{ last_score: u16},
    GameFinished
}

impl ScoreMessageMode {
    fn value(&self) -> String {
        match self {
            NewScore => "Enter the new score".to_string(),
            ResetLastScore{last_score} => format!("{} {}", "Correct last entered Score: ".to_string(), last_score.to_string()),
            ScoreMessageMode::GameFinished => "Game finished".to_string(),
        }

    }
}

#[derive(Props, PartialEq, Clone)]
struct ScoreMessage {
    score_message_mode: ScoreMessageMode,
    score_message_label: u16,
}

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

            input {id: "numberField",
                class:"shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline max-w-20",
                type: "number", maxlength:10, min:0, oninput: move |e| raw_input.set(e.value()),
                onkeypress: move |e| {
                    if e.key() == Key::Enter {
                        input_changed(count, is_wrong, raw_input, score_message)
                    }
                }
            }

        }

        div {
            class:"flex items-center justify-between",

            button {id: "confirmButton",
                onclick: move |_| {
                        input_changed(count, is_wrong, raw_input, score_message)
                },
                class:"btn btn-primary" , "Ok" },


            button {id: "resetButton",
                onclick: move |_| {
                        reset_last_score(count, is_wrong, score_message)
                },
                disabled: if count.read().len() < 2 {true},
                class:"btn btn-primary" , "Reset Last Score" },

            button {id: "newGameButton",
                onclick: move |_| {
                        new_game(count, is_wrong)
                },
                class:"btn btn-secondary" , "New Game" },
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

fn new_game(
    mut count: Signal<Vec<CurrentScore>>,
    mut is_wrong: Signal<bool>,
) {
    is_wrong.set(false);
    count.write().clear();
    let init_score = CurrentScore {
        remaining: 501,
        thrown: 0,
    };
    count.push(init_score);
}

fn reset_last_score(
    mut count: Signal<Vec<CurrentScore>>,
    mut is_wrong: Signal<bool>,
    mut score_message: Signal<ScoreMessageMode>
) {
    is_wrong.set(false);
    let generational_ref = count.read();
    let last_score = generational_ref.last();
    match last_score {
        Some(val) => score_message.set(ResetLastScore { last_score: val.thrown}),
        None => {}
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
        ResetLastScore {last_score: _}=>  {
            count.write().pop();
            score_message.set(NewScore)
        },
        NewScore => {},
        ScoreMessageMode::GameFinished => {is_wrong.set(true)}
    }
    let result = input_ref.read().parse();
    match result {
        Ok(val) => {
            if val <= 180 {
                let new_score = get_new_score(&count, val);
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

fn get_new_score(count: &Signal<Vec<CurrentScore>>, val: u16) -> CurrentScore {
    let generational_ref = count.read();
    let last = generational_ref.last().unwrap();
    let last_remaining = last.remaining;
    let new_remaining: u16;
    if val <= last_remaining {
        new_remaining = last_remaining - val;
    } else {
        new_remaining = last_remaining;
    }
    CurrentScore {
        remaining: new_remaining,
        thrown: val,
    }
}
