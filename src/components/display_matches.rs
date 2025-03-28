use crate::domain::{Leg, Match, Set, INIT_SCORE};
use crate::{backend, Route};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use crate::components::breadcrumb::BreadCrumbComponent;
use crate::domain::ErrorMessageMode::CreateNewLeg;

#[component]
pub fn DisplayMatches() -> Element {
    let mut matches = use_signal(|| vec![]);

    use_resource(move || async move {
        let res = backend::api::dart_match::list_matches().await;
        match res {
            Ok(val) if !val.is_empty() => matches.set(val),
            _ => {}
        };
    });

    rsx! {

        div {
            id: "DisplayMatches",
            class: "container-self",


            div {
                BreadCrumbComponent {only_home: false, match_signal: None, set_signal: None, leg_signal: None}

                 div {

                    class:"bg-base-100 border-y-4 shadow-md rounded px-8 pt-6 pb-8",

                     button {id: "newLegButton",
                         onclick: move |_| async move {
                                        let _ = new_match(matches).await;

                         },
                         class:"btn btn-soft btn-primary" , "New Match"
                     },

                 }


                div {
                        MatchTable{matches}
                }

                }

        }
   }
}
async fn new_match(mut matches: Signal<Vec<Match>>) ->  Result<(), ServerFnError> {
    let new_match = backend::api::dart_match::new_match().await?;
    matches.push(new_match.clone());
    Ok(())
}

#[component]
pub fn MatchTable(matches: Signal<Vec<Match>>) -> Element {
    rsx! {
      div {
            id:"BottomHalf",
            class:"bg-neutral shadow-md rounded px-8 pt-6 pb-8 mb-4 overflow-y-auto",
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
                                "Nr (click me)"
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
                        for (i, a) in matches().into_iter().rev().enumerate() {
                            tr {
                                    td {
                                        class: if i == 0 {"px-6 py-4 bg-accent text-accent-content"},
                                        class: if i % 2 == 0 && i!=0 {"px-6 py-4 bg-base-200 text-base-content"},
                                        class: if i % 2 == 1 {"px-6 py-4 bg-base-300 text-base-content"},
                                        style:"white-space: pre; text-align: center;",

                                        li {
                                            Link {to: Route::WrapDisplaySets {matchval: a.id}, {a.id.to_string()}}
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
