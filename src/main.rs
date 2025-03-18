use dioxus::prelude::*;

use components::Panel;

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


        Panel {}
    }
}
