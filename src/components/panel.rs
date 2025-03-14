use std::cell::Ref;
use std::num::ParseIntError;
use dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Props, PartialEq, Clone)]
struct CurrentScore {
    remaining: u16,
    thrown: u16
}

#[component]
pub fn Panel() -> Element {
    let mut raw_input = use_signal(|| "".to_string());
    let init_score = CurrentScore { remaining: 501, thrown: 0 };
    let init_count_vector = vec![init_score];
    let mut count = use_signal(|| init_count_vector);
    let is_wrong = use_signal(|| false);
    rsx! {
        "Enter your latest result:",
        div {
            id: "panelDiv",
            input {id: "numberField", type: "number", maxlength:10, min:0, oninput: move |e| raw_input.set(e.value()),
                onkeypress: move |e| {
                    if e.key() == Key::Enter {
                        //tracing::info!("he11o");
                        input_changed(count, is_wrong, raw_input)
                    }
                }
            }
            button {id: "confirmButton",
                onclick: move |_| {
                        input_changed(count, is_wrong, raw_input)
                },
                class: "bg-purple-200 px-4 py-2 rounded-lg border border-black hover:border-indigo-500 active:scale-95 transition-all", "Ok" }
        }
        div {
            id: "displayError",
            if is_wrong() {
                "Please enter a valid number"
            }
        }

        div { id: "numbers",
            table {
            for a in count.iter() {
                tr {
                        td {
                            style:"white-space: pre; text-align: right;",
                            {format!("{:>3}", a.thrown.to_string())}
                        },
                        td {
                            style:"white-space: pre; text-align: right;",
                            {format!("{:>3}", a.remaining.to_string())}
                        },
                }
            }

            }
        }


    }
}

fn input_changed(mut count: Signal<Vec<CurrentScore>>, mut is_wrong: Signal<bool>, input_ref: Signal<String>) {
    let result = input_ref.read().parse();
    match result {
        Ok(val) => {
            if val <= 180 {
                let new_score = get_new_score(&count, val);
                count.write().push(new_score);
                is_wrong.set(false)
            } else {
                is_wrong.set(true)
            }
        }
        Err(_) => { is_wrong.set(true)}
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
    CurrentScore { remaining: new_remaining, thrown: val }
}