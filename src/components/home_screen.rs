use crate::components::breadcrumb::BreadCrumbComponent;
use crate::domain::ErrorMessageMode::CreateNewLeg;
use crate::domain::{Leg, Set, INIT_SCORE};
use crate::{backend, Route};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use tracing::debug;
#[component]
pub fn HomeScreen() -> Element {
    let mut latest_leg_exists_signal = use_signal(|| false);
    let mut latest_set_exists_signal = use_signal(|| false);
    let mut latest_match_exists_signal = use_signal(|| false);
    use_resource(move || async move {
        let leg_exists = backend::api::dart_leg::get_latest_leg().await;
        latest_leg_exists_signal.set(leg_exists.is_ok());
        let set_exists = backend::api::dart_set::get_latest_set().await;
        latest_set_exists_signal.set(set_exists.is_ok());
        let match_exists = backend::api::dart_match::get_latest_match().await;
        latest_match_exists_signal.set(match_exists.is_ok());
    });
    rsx! {
        div {
            id: "DisplayScore",
            class: "container-self",
            div {
                BreadCrumbComponent {
                    only_home: true,
                }
                div {
                    class: "bg-base-100 border-y-12 shadow-md rounded px-8 pt-6 pb-8 grid grid-cols-12 gap-4",
                    if latest_leg_exists_signal() {
                        Link {
                            to: Route::LatestLeg,
                            class: "text-xl text-primary ",
                            title: "Show score of the latest leg",
                            "Latest Score"
                        }
                    } else {
                        p {
                            class: "text-xl text-base-content",
                            title: "No Score available",
                            "Latest Score"
                        }
                    }
                    if latest_set_exists_signal() {
                        Link {
                            to: Route::LatestSet,
                            class: "text-xl text-secondary ",
                            title: "Show legs of the latest set ",
                            "Latest Legs"
                        }
                    } else {
                        p {
                            class: "text-xl text-base-content",
                            title: "No Set available",
                            "Latest Legs"
                        }
                    }
                    if latest_match_exists_signal() {
                        Link {
                            to: Route::LatestMatch,
                            class: "text-xl text-secondary",
                            title: "Show sets of latest match",
                            "Latest Sets"
                        }
                    } else {
                        p {
                            class: "text-xl text-base-content",
                            title: "No Match available",
                            "Latest Set"
                        }
                    }
                    Link {
                        to: Route::DisplayMatches,
                        class: "text-xl text-secondary",
                        title: "Show list of matches",
                        "Matches"
                    }
                
                }
            
            }
            div {
                "About:"
                p {
                    "A work in progress page for creating dart matches and tracking dart scores"
                }
            
            }
        
        }
    }
}
