use crate::components::DisplayMatches;
use crate::components::DisplaySets;
use crate::domain::{IdOrder, Leg, Set};
use components::Test;
use components::{DisplayLegs, MainScoreComponent};
use dioxus::prelude::*;
use tracing::debug;

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
    LatestLeg,

    #[route("/test")]
    Test,
}

#[component]
fn WrapDisplayScore(matchval: u16, set_id: u16, leg_id: u16) -> Element {
    let mut match_signal = use_signal(|| matchval);

    let mut leg: ReadOnlySignal<Option<Result<Leg, ServerFnError>>> =
        use_server_future(move || backend::get_leg_by_id(leg_id as i32))?.value();
    let mut set: ReadOnlySignal<Option<Result<Set, ServerFnError>>> =
        use_server_future(move || backend::get_set_by_id(set_id as i32))?.value();

    let mut set_signal = use_signal(|| IdOrder { id: 0, order: 0 });
    let mut leg_signal = use_signal(|| IdOrder { id: 0, order: 0 });

    match (&*leg.read_unchecked(), &*set.read_unchecked()) {
        (Some(Ok(leg_val)), Some(Ok(set_val))) => {
            leg_signal.set(IdOrder {
                id: leg_id,
                order: leg_val.leg_order,
            });
            set_signal.set(IdOrder {
                id: set_id,
                order: set_val.set_order,
            });
            rsx! { MainScoreComponent {match_signal, set_signal, leg_signal}}
        }
        _ => rsx! { "Error" },
    }
}

#[component]
fn WrapDisplayLegs(matchval: u16, set_id: u16) -> Element {
    debug!("WrapDisplayLegs Set_Id {:?}", set_id);
    let mut set: ReadOnlySignal<Option<Result<Set, ServerFnError>>> =
        use_server_future(move || backend::get_set_by_id(set_id as i32))?.value();
    let mut set_signal = use_signal(|| IdOrder { id: 0, order: 0 });

    let mut match_signal = use_signal(|| matchval);

    match &*set.read_unchecked() {
        Some(Ok(set_val)) => {
            set_signal.set(IdOrder {
                id: set_id,
                order: set_val.set_order,
            });
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
    let mut match_signal = use_signal(|| 0);

    let mut set_signal: Signal<IdOrder> = use_signal(|| IdOrder { id: 0, order: 0 });
    let mut leg_signal: Signal<IdOrder> = use_signal(|| IdOrder { id: 0, order: 0 });

    let mut init_latest_leg = use_server_future(backend::get_latest_leg)?.suspend()?;
    use_resource(move || {
        let latest_leg_signal = init_latest_leg.clone();
        async move {
            let latest_leg = latest_leg_signal()?;
            match latest_leg {
                Some((set_id_order, leg)) => {
                    set_signal.set(set_id_order);
                    leg_signal.set(IdOrder {
                        id: leg.id,
                        order: leg.leg_order,
                    });
                }
                _ => (
                    //todo
                    ),
            }
            Ok::<(), ServerFnError>(())
        }
    });
    rsx! {
        MainScoreComponent {match_signal, set_signal, leg_signal}
    }
}
