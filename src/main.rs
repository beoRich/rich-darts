use crate::components::DisplayMatches;
use crate::components::DisplaySets;
use crate::domain::{Leg, Set};
use components::Test;
use components::{DisplayLegs, HomeScreen, MainScoreComponent};
use dioxus::prelude::*;
mod backend;
mod components;
mod domain;
mod schema_manual;
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const CUSTOM_CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    use_server_future(move || backend::log_init())?;
    use_server_future(move || backend::run_migrations())?;
    rsx! {
        // Global app resources
        document::Stylesheet {
            href: TAILWIND_CSS,
        }
        document::Stylesheet {
            href: CUSTOM_CSS,
        }
        Router::<Route> {
        
        }
    }
}
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/match/:matchval/:set_id/:leg_id")]
    WrapDisplayScore {
        matchval: u16,
        set_id: u16,
        leg_id: u16,
    },
    #[route("/match/:matchval/:set_id")]
    WrapDisplayLegs { matchval: u16, set_id: u16 },
    #[route("/match")]
    DisplayMatches,
    #[route("/match/:matchval")]
    WrapDisplaySets { matchval: u16 },
    //todo someday resolve this via redirect
    #[route("/")]
    HomeScreen,
    #[route("/latest/leg")]
    LatestLeg,
    #[route("/latest/set")]
    LatestSet,
    #[route("/latest/match")]
    LatestMatch,
    #[route("/test")]
    Test,
}
#[component]
fn WrapDisplayScore(matchval: u16, set_id: u16, leg_id: u16) -> Element {
    let mut leg_read_only_signal: ReadOnlySignal<Option<Result<Leg, ServerFnError>>> =
        use_server_future(move || backend::api::dart_leg::get_leg_by_id(leg_id as i32))?.value();
    let mut set_signal: ReadOnlySignal<Option<Result<Set, ServerFnError>>> =
        use_server_future(move || backend::api::dart_set::get_set_by_id(set_id as i32))?.value();
    match (
        &*leg_read_only_signal.read_unchecked(),
        &*set_signal.read_unchecked(),
    ) {
        (Some(Ok(leg_val)), Some(Ok(set_val))) => {
            rsx! {
                MainScoreComponent {
                    match_id: matchval,
                    set_input: set_val.clone(),
                    leg_input: leg_val.clone(),
                }
            }
        }
        _ => rsx! { "Error or loading" },
    }
}
#[component]
fn WrapDisplayLegs(matchval: u16, set_id: u16) -> Element {
    let mut set: ReadOnlySignal<Option<Result<Set, ServerFnError>>> =
        use_server_future(move || backend::api::dart_set::get_set_by_id(set_id as i32))?.value();
    let mut match_signal = use_signal(|| matchval);
    match &*set.read_unchecked() {
        Some(Ok(set_val)) => {
            rsx! {
                DisplayLegs {
                    match_id: matchval,
                    set_input: set_val.clone(),
                }
            }
        }
        _ => rsx! { "Error" },
    }
}
#[component]
fn WrapDisplaySets(matchval: u16) -> Element {
    rsx! {
        DisplaySets {
            match_id: matchval,
        }
    }
}
#[component]
fn LatestLeg() -> Element {
    let latest_leg_with_set_order: ReadOnlySignal<Option<Result<(u16, Set, Leg), ServerFnError>>> =
        use_server_future(move || backend::api::dart_leg::get_latest_leg())?.value();
    match &*latest_leg_with_set_order.read_unchecked() {
        Some(Ok((match_id_ref, set_ref, leg_ref))) => {
            rsx! {
                MainScoreComponent {
                    match_id: *match_id_ref,
                    set_input: set_ref.clone(),
                    leg_input: leg_ref.clone(),
                }
            }
        }
        _ => rsx! { "Error or loading" },
    }
}
#[component]
fn LatestSet() -> Element {
    let latest_set_parent_id: ReadOnlySignal<Option<Result<(u16, Set), ServerFnError>>> =
        use_server_future(move || backend::api::dart_set::get_latest_set())?.value();
    match &*latest_set_parent_id.read_unchecked() {
        Some(Ok((parent_id, set_ref))) => {
            rsx! {
                DisplayLegs {
                    match_id: *parent_id,
                    set_input: set_ref.clone(),
                }
            }
        }
        _ => rsx! { "Error or loading" },
    }
}
#[component]
fn LatestMatch() -> Element {
    let latest_match_id: ReadOnlySignal<Option<Result<u16, ServerFnError>>> =
        use_server_future(move || backend::api::dart_match::get_latest_match())?.value();
    match &*latest_match_id.read_unchecked() {
        Some(Ok((match_id_ref))) => {
            rsx! {
                DisplaySets {
                    match_id: *match_id_ref,
                }
            }
        }
        _ => rsx! { "Error or loading" },
    }
}
