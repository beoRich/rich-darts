use crate::domain::{Leg, Set, INIT_SCORE};
use crate::{backend, Route};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use crate::components::breadcrumb::BreadCrumbComponent;
use crate::components::main_score_component::new_leg_wrapper;
use crate::domain::ErrorMessageMode::CreateNewLeg;

#[component]
pub fn DisplaySets(match_signal: Signal<u16>) -> Element {
    let mut sets_signal = use_signal(|| vec![]);

    use_resource(move || async move {
        let match_val = match_signal();
        let res = backend::list_set(match_val as i32).await;
        match res {
            Ok(val) if !val.is_empty() => sets_signal.set(val),
            _ => {}
        };
    });

    rsx! {

        div {
            id: "All",
            class: "container-self",


            div {
                BreadCrumbComponent {match_signal, set_signal: None, leg_signal: None}


                div {
                    SetTable{match_signal, sets_signal}
                }

                        button {id: "newSetButton",
                            onclick: move |_| async move {
                                    let _ = new_set(match_signal, sets_signal).await;
                            },
                            class:"btn btn-soft btn-info" , "New Set" },

                }

        }
   }
}

async fn new_set(mut match_signal: Signal<u16>, mut sets_signal: Signal<Vec<Set>>) ->  Result<(), ServerFnError> {
    let match_id = match_signal();
    let new_set = backend::new_set(match_id as i32).await?;
    sets_signal.push(new_set);
    Ok(())
}

#[component]
pub fn SetTable(mut match_signal: Signal<u16>, mut sets_signal: Signal<Vec<Set>>) -> Element {
    //todo coalesce into generic with score_display
    rsx! {

     div {
            "List of sets"
        }

      div {
            id:"BottomHalf",
            class:"bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 overflow-y-auto",
            div { id: "numbers",
                    class: "table-container",
                table {
                    class: "text-xl uppercase bg-neutral-content",
                    thead {
                        tr {
                            th {
                                scope:"col",
                                style:"white-space: pre; text-align: center;",
                                class:"text-primary px-6 py-3",
                                "Id (click me)"
                            },
                            th {
                                scope:"col",
                                style:"white-space: pre; text-align: center;",
                                class:"text-secondary px-6 py-3",
                                "Status"
                            }
                        }
                    }
                    tbody {
                        id: "numbers-body",
                        for (i, a) in sets_signal().into_iter().rev().enumerate() {
                            tr {
                                    td {
                                        class: if i == 0 {"px-6 py-4 bg-accent text-accent-content"},
                                        class: if i % 2 == 0 && i!=0 {"px-6 py-4 bg-base-200 text-base-content"},
                                        class: if i % 2 == 1 {"px-6 py-4 bg-base-300 text-base-content"},
                                        style:"white-space: pre; text-align: center;",

                                        li {
                                            Link {to: Route::WrapDisplayLegs {matchval: match_signal(),setval:  a.id}, {a.id.to_string()}}
                                        }

                                    },
                                    td {
                                        class: if i == 0 {"px-6 py-4 bg-accent text-accent-content"},
                                        class: if i % 2 == 0 && i!=0 {"px-6 py-4 bg-base-200 text-base-content"},
                                        class: if i % 2 == 1 {"px-6 py-4 bg-base-300 text-base-content"},
                                        style:"white-space: pre; text-align: center;",
                                        {format!("{:>3}", a.status)}
                                    },
                            }
                        }
                    }

                }
            }
      }

    }
}
