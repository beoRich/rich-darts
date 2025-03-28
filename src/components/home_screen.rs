use crate::domain::{Leg, Set, INIT_SCORE};
use crate::{backend, Route};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use tracing::debug;
use crate::components::breadcrumb::BreadCrumbComponent;
use crate::domain::ErrorMessageMode::CreateNewLeg;

#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        div {
            id: "DisplayScore",
            class: "container-self",

            div {
                BreadCrumbComponent {only_home: true}

                div {
                    class:"bg-base-100 border-y-12 shadow-md rounded px-8 pt-6 pb-8 grid grid-cols-12 gap-4",
                    Link {to: Route::DisplayMatches , class:"text-xl", "Matches"}
                    Link {to: Route::DisplayMatches , class:"text-xl", "Latest Sets Todo"}
                    Link {to: Route::DisplayMatches , class:"text-xl", "Latest Legs Todo"}
                    Link {to: Route::LatestLeg , class:"text-xl", "Latest Score"}
                }

            }

        }

    }
}
