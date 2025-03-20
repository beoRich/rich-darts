use dioxus::prelude::*;
use crate::components::main_component::{input_wrapper, new_leg_wrapper, undo_wrapper};
use crate::domain::{CurrentScore, ErrorMessageMode, ScoreMessageMode};

#[component]
pub fn EnterPanel(count: Signal<Vec<CurrentScore>>,
                  mut raw_input: Signal<String>,
                  leg: Signal<u16>,
                  mut error_message: Signal<ErrorMessageMode>,
                  score_message: Signal<ScoreMessageMode>,
                  allow_score: Signal<bool>

) -> Element {
   rsx! {


      div {
        id:"EnterPanel",
        class:"bg-base-100 shadow-md rounded px-8 pt-6 pb-8 mb-4 overflow-x-scroll",
        NunberFieldError {count, raw_input, leg, error_message, score_message, allow_score}
        div {
            id: "ButtonsDiv",
            class:"grid grid-cols-10 gap-4",

                div {
                    class:"col-span-1 grid ",
                    button {id: "confirmButton",
                        onclick: move |_| async move {
                                input_wrapper(raw_input, leg, count, error_message, score_message).await;
                        },
                        disabled: if !allow_score() {true},
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
                                    new_leg_wrapper(leg, count, error_message, score_message).await;
                            },
                            class:"btn btn-soft btn-info" , "New Leg" },
                    }
                }
        }


    }
   }
}

#[component]
fn NunberFieldError(
    count: Signal<Vec<CurrentScore>>,
    mut raw_input: Signal<String>,
    leg: Signal<u16>,
    mut error_message: Signal<ErrorMessageMode>,
    score_message: Signal<ScoreMessageMode>,
    allow_score: Signal<bool>
) -> Element {
    rsx! {
        div {
            id: "NumberFieldError",
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
                            if key == Key::Enter && allow_score() {
                                input_wrapper(raw_input, leg, count, error_message, score_message).await;
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
    }

}
