use crate::domain::{Leg, Set, INIT_SCORE};
use crate::{backend, Route};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use crate::components::main_score_component::new_leg_wrapper;
use crate::domain::ErrorMessageMode::CreateNewLeg;

#[component]
pub fn DisplaySets(match_signal: Signal<u16>) -> Element {
    let mut sets = use_signal(|| vec![]);

    use_resource(move || async move {
        let match_val = match_signal();
        let res = backend::list_set(match_val as i32).await;
        match res {
            Ok(val) if !val.is_empty() => sets.set(val),
            _ => {}
        };
    });

    rsx! {

        div {
            id: "DisplaySetDiv",
            div {
                    SetTable{sets}
            }

                        button {id: "newSetButton",
                            onclick: move |_| async move {
                                    let res = backend::get_latest_leg().await;
                                    let new_leg_val = res.map(|val| val +1).unwrap_or(1);
                                    //new_leg(new_leg_val, sets).await;
                            },
                            class:"btn btn-soft btn-info" , "New Set" },

            }

    }
}

async fn new_leg(leg_val: u16, mut legs: Signal<Vec<Leg>>) {
    let new_leg = Leg {
        id: leg_val,
        status: "New".to_string(),
    };
    legs.push(new_leg.clone());
    backend::save_leg(new_leg)
        .await
        .expect(&format!("Could not save leg {}", leg_val));
    let _ = backend::save_score(leg_val, INIT_SCORE).await;
}

#[component]
pub fn SetTable(sets: Signal<Vec<Set>>) -> Element {
    //todo coalesce into generic with score_display
    rsx! {
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
                        for (i, a) in sets().into_iter().rev().enumerate() {
                            tr {
                                    td {
                                        class: if i == 0 {"px-6 py-4 bg-accent text-accent-content"},
                                        class: if i % 2 == 0 && i!=0 {"px-6 py-4 bg-base-200 text-base-content"},
                                        class: if i % 2 == 1 {"px-6 py-4 bg-base-300 text-base-content"},
                                        style:"white-space: pre; text-align: center;",

                                        li {
                                            Link {to: Route::ManualLeg {legval: a.id}, {a.id.to_string()}}
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
