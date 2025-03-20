use crate::domain::ScoreMessageMode::NewShot;
use crate::domain::{ErrorMessageMode, Leg, Score, ScoreMessageMode, INIT_SCORE};
use crate::{backend, Route};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub fn DisplayLegs() -> Element {
    let mut legs = use_signal(|| vec![]);
    let mut init_leg_db = use_server_future(backend::list_leg)?.suspend()?;
    use_resource(move || {
        let init_leg_db_clone = init_leg_db.clone();
        async move {
            let init_leg_val = init_leg_db_clone();
            if init_leg_val.is_ok() {
                legs.set(init_leg_val.clone().unwrap());
            }
        }
    });
    rsx! {

        div {
            id: "DisplayLegDiv",
            div {
                    LegTable{legs}
            }

                        button {id: "newLegButton",
                            onclick: move |_| async move {
                                    let res = backend::get_latest_leg().await;
                                    let new_leg_val = res.map(|val| val +1).unwrap_or(1);
                                    new_leg(new_leg_val, legs).await;
                            },
                            class:"btn btn-soft btn-info" , "New Leg" },

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
    let db_op_res = backend::save_score(leg_val, INIT_SCORE).await;
}

#[component]
pub fn LegTable(legs: Signal<Vec<Leg>>) -> Element {
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
                        for (i, a) in legs().into_iter().rev().enumerate() {
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
