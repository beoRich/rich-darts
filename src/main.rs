use crate::components::DisplayMatches;
use crate::components::DisplaySets;
use crate::domain::{IdOrder, IdOrderParent, Leg, Set};
use components::Test;
use components::{DisplayLegs, MainScoreComponent, HomeScreen};
use dioxus::prelude::*;
use tracing::{debug, Id};
use web_sys::window;

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
            href: TAILWIND_CSS
        }
        document::Stylesheet {
            href: CUSTOM_CSS
        }
        Router::<Route> {}


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

    #[route("/latest")]
    LatestLeg,

    #[route("/test")]
    Test,
}

#[component]
fn WrapDisplayScore(matchval: u16, set_id: u16, leg_id: u16) -> Element {
    let mut match_signal = use_signal(|| matchval);

    let mut leg_read_only_signal: ReadOnlySignal<Option<Result<Leg, ServerFnError>>> =
        use_server_future(move || backend::api::dart_leg::get_leg_by_id(leg_id as i32))?.value();
    let mut set: ReadOnlySignal<Option<Result<Set, ServerFnError>>> =
        use_server_future(move || backend::api::dart_set::get_set_by_id(set_id as i32))?.value();

    debug!("WrapDisplayScore set{:?}", set);

    match (&*leg_read_only_signal.read_unchecked(), &*set.read_unchecked()) {
        (Some(Ok(leg_val)), Some(Ok(set_val))) => {
            let mut set_signal = use_signal(|| IdOrder {
                id: set_val.id,
                order: set_val.set_order,
            });
            let mut leg_signal = use_signal(|| leg_val.clone());
            rsx! { MainScoreComponent {match_signal, set_signal, leg_signal}}

        }
        _ => rsx! { "Error or loading" },
    }
}

#[component]
fn WrapDisplayLegs(matchval: u16, set_id: u16) -> Element {
    debug!("WrapDisplayLegs Set_Id {:?}", set_id);
    let mut set: ReadOnlySignal<Option<Result<Set, ServerFnError>>> =
        use_server_future(move || backend::api::dart_set::get_set_by_id(set_id as i32))?.value();

    let mut match_signal = use_signal(|| matchval);
    debug!("WrapDisplayLegs2 Set_Id {:?}", set_id);
    debug!("WrapDisplayLegs2 matchval {:?}", matchval);

    match &*set.read_unchecked() {
        Some(Ok(set_val)) => {
            let mut set_signal = use_signal(|| IdOrder { id: set_id, order: set_val.set_order });
            rsx! {
                DisplayLegs {match_signal, set_signal}
            }
        }
        _ => rsx! { "Error" },
    }
}

#[component]
fn WrapDisplaySets(matchval: u16) -> Element {
    let mut match_signal = use_signal(|| 0);
    match_signal.set(matchval);
    rsx! {
        DisplaySets {match_signal }
    }
}

#[component]
fn LatestLeg() -> Element {

    let mut latest_leg_with_set_order: ReadOnlySignal<Option<Result<(IdOrderParent, Leg), ServerFnError>>> =
        use_server_future(move || backend::api::dart_leg::get_latest_leg())?.value();

    match &*latest_leg_with_set_order.read_unchecked() {
        Some(Ok((set_id_oder_par_ref, leg_ref))) => {
            let set_id_order_par = *set_id_oder_par_ref;
            let mut set_signal = use_signal(|| IdOrder{id: set_id_order_par.id, order: set_id_order_par.order});
            let mut match_signal = use_signal(|| set_id_oder_par_ref.parent_id);
            let mut leg_signal = use_signal(|| leg_ref.clone());
            rsx! { MainScoreComponent {match_signal, set_signal, leg_signal}}
        }
        _ => rsx! { "Error or loading" },
    }
}
