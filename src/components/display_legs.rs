use crate::domain::{IdOrder, Leg, INIT_SCORE};
use crate::{backend, Route};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use crate::components::breadcrumb::BreadCrumbComponent;

#[component]
pub fn DisplayLegs(match_signal: Signal<u16>, set_signal: Signal<IdOrder>) -> Element {
    let mut legs_signal = use_signal(|| vec![]);

    use_resource(move || async move {
        let res = backend::list_leg(set_signal().id as i32).await;
        match res {
            Ok(val) if !val.is_empty() => legs_signal.set(val),
            _ => {}
        };
    });

    rsx! {

        div {
            id: "All",
            class: "container-self",


            div {
                BreadCrumbComponent {match_signal, set_signal, leg_signal: None}


                div {
                    LegTable{match_signal, set_signal, legs_signal}
                }

                        button {id: "newLegButton",
                            onclick: move |_| async move {
                                    let _ = new_leg(set_signal, legs_signal).await;
                            },
                            class:"btn btn-soft btn-info" , "New Leg" },


            }

        }
   }

}

async fn new_leg(set_signal: Signal<IdOrder>, mut legs_signal: Signal<Vec<Leg>>) -> Result<(), ServerFnError>{
    let new_leg = backend::new_leg_init_score(set_signal().id as i32).await?;
    legs_signal.push(new_leg);
    Ok(())
}

#[component]
pub fn LegTable(match_signal: Signal<u16>, set_signal: Signal<IdOrder>, legs_signal: Signal<Vec<Leg>>) -> Element {
    rsx! {

     div {
            "List of legs"
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
                        for (i, a) in legs_signal().into_iter().rev().enumerate() {
                            tr {
                                    td {
                                        class: if i == 0 {"px-6 py-4 bg-accent text-accent-content"},
                                        class: if i % 2 == 0 && i!=0 {"px-6 py-4 bg-base-200 text-base-content"},
                                        class: if i % 2 == 1 {"px-6 py-4 bg-base-300 text-base-content"},
                                        style:"white-space: pre; text-align: center;",

                                        li {
                                            Link {to: Route::WrapDisplayScore {matchval: match_signal(), setval: set_signal().id, legval: a.id}, {a.id.to_string()}}
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
