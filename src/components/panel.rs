use dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use dioxus::prelude::*;
use dioxus_elements::style;
use dioxus_logger::tracing;
use std::cell::Ref;
use std::num::ParseIntError;

#[derive(Props, PartialEq, Clone)]
struct CurrentScore {
    remaining: u16,
    thrown: u16,
}

#[component]
pub fn Panel() -> Element {
    let mut raw_input = use_signal(|| "".to_string());
    let init_score = CurrentScore {
        remaining: 501,
        thrown: 0,
    };
    let init_count_vector = vec![init_score];
    let mut count = use_signal(|| init_count_vector);
    let is_wrong = use_signal(|| false);
    rsx! {
      div {
            class:"bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4",
        div {
            class:"mb-4",
                label {
                    class:"block text-gray-700 text-sm font-bold mb-2",
                    for: "numberField",
                    "Enter the score"
                    }

            input {id: "numberField",
                class:"shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"a,
                type: "number", maxlength:10, min:0, oninput: move |e| raw_input.set(e.value()),
                onkeypress: move |e| {
                    if e.key() == Key::Enter {
                        //tracing::info!("he11o");
                        input_changed(count, is_wrong, raw_input)
                    }
                }
            }

        }

        div {
            class:"flex items-center justify-between",

            button {id: "confirmButton",
                onclick: move |_| {
                        input_changed(count, is_wrong, raw_input)
                },
                class:"btn btn-primary" , "Ok" },
        }

    }
        div {
            id: "displayError",
            if is_wrong() {
                p {
                class: "text-xs text-red-700",
                "Please enter a valid number" }
            }
        }
        div { id: "numbers",
            //class:"relative overflow-x-auto",
            table {
                class: "w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400",
                thead {
                    class: "text-xs text-gray-700 uppercase bg-gray-100 dark:bg-gray-700 dark:text-gray-400",
                    tr {
                        th {
                            scope:"col",
                            style:"white-space: pre; text-align: center;",
                            class:"text-blue-600 px-6 py-3",
                            "Thrown"
                        },
                        th {
                            scope:"col",
                            style:"white-space: pre; text-align: center;",
                            class:"px-6 py-3",
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

fn input_changed(
    mut count: Signal<Vec<CurrentScore>>,
    mut is_wrong: Signal<bool>,
    input_ref: Signal<String>,
) {
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
