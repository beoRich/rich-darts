use dioxus::prelude::*;

use components::Test;
use components::{DisplayLegs, MainScoreComponent};
use crate::components::DisplaySets;
use crate::components::DisplayMatches;

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
    #[route("/match/:matchval/:setval/:legval")]
    WrapDisplayScore {matchval: u16, setval: u16, legval: u16 },

    #[route("/match/:matchval/:setval")]
    WrapDisplayLegs{matchval: u16, setval: u16},

    #[route("/match")]
    DisplayMatches,

    #[route("/match/:matchval")]
    WrapDisplaySets{matchval: u16},

    //todo someday resolve this via redirect
    #[route("/")]
    LatestLeg,

    #[route("/test")]
    Test,
}


#[component]
fn WrapDisplayScore(matchval: u16, setval: u16, legval: u16) -> Element {
    let mut set_signal = use_signal(|| 0);
    set_signal.set(setval);
    let mut leg_signal = use_signal(|| 0);
    leg_signal.set(legval);
    rsx! {
        MainScoreComponent {set_signal, leg_signal}
    }
}

#[component]
fn WrapDisplayLegs(matchval: u16, setval: u16) -> Element {
    let mut set_signal = use_signal(|| 0);
    set_signal.set(setval);

    let mut match_signal = use_signal(|| 0);
    match_signal.set(matchval);

    rsx! {
        DisplayLegs {match_signal, set_signal}
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
    let mut leg_signal = use_signal(|| 0);
    let mut set_signal = use_signal(|| 0);

    let mut init_latest_leg = use_server_future(backend::get_latest_leg)?.suspend()?;
    use_resource(move || {
        let latest_leg_signal = init_latest_leg.clone();
        async move {
            let latest_leg = latest_leg_signal()?;
            match latest_leg {
                Some((set_id, leg)) => {
                    set_signal.set(set_id);
                    leg_signal.set(leg.id);

                }
                _ => (
                    //todo
                    )
            }
            Ok::<(), ServerFnError>(())
        }
    });
    rsx! {
        MainScoreComponent {set_signal, leg_signal}
    }
}
