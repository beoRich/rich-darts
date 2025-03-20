use dioxus::prelude::*;

use components::{MainComponent, DisplayLegs};
use components::Test;
use crate::domain::ErrorMessageMode::CreateNewLeg;

mod components;
mod backend;
mod domain;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const CUSTOM_CSS: Asset = asset!("/assets/main.css");


fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
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

    #[route("/leg/:legval")]
    ManualLeg {
        legval: u16
    },

    #[route("/leg")]
    DisplayLegs,
    //todo someday resolve this via redirect
    #[route("/")]
    LatestLeg,


    #[route("/test")]
    Test,
}


#[component]
fn ManualLeg(legval: u16) -> Element {
    let mut leg = use_signal(|| 0);
    leg.set(legval);
    rsx! {
        MainComponent {leg}
    }

}


#[component]
fn LatestLeg() -> Element {
    let mut leg = use_signal(|| 0);
    let mut init_leg_db = use_server_future(backend::get_latest_leg)?.suspend()?;
    use_resource(move || {
        let init_leg_db_clone = init_leg_db.clone();
        async move {
            let init_leg_val = init_leg_db_clone();
            if init_leg_val.is_ok() {
                leg.set(init_leg_val.clone().unwrap());
            }
        }
    });
    rsx! {
        MainComponent {leg}
    }
}
