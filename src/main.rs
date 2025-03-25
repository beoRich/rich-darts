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
    #[route("/score/:legval")]
    ManualLeg { legval: u16 },

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
fn WrapDisplayLegs(matchval: u16, setval: u16) -> Element {
    let mut set_signal = use_signal(|| 0);
    set_signal.set(setval);

    rsx! {
        DisplayLegs {set_signal}
    }
}

#[component]
fn WrapDisplaySets(matchval: u16) -> Element {
    let mut signal = use_signal(|| 0);
    signal.set(matchval);
    rsx! {
        DisplaySets {match_signal: signal}
    }
}

#[component]
fn ManualLeg(legval: u16) -> Element {
    let mut leg_signal = use_signal(|| 0);
    leg_signal.set(legval);
    rsx! {
        MainScoreComponent {leg_signal}
    }
}

#[component]
fn LatestLeg() -> Element {
    let mut leg_signal = use_signal(|| 0);
    let mut init_leg_db = use_server_future(backend::get_latest_leg)?.suspend()?;
    use_resource(move || {
        let init_leg_db_clone = init_leg_db.clone();
        async move {
            let init_leg_val = init_leg_db_clone();
            if init_leg_val.is_ok() {
                leg_signal.set(init_leg_val.clone().unwrap());
            }
        }
    });
    rsx! {
        MainScoreComponent {leg_signal}
    }
}
