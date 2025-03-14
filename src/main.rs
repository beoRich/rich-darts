use dioxus::prelude::*;

use components::{Panel};

mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: TAILWIND_CSS
        }


        Panel {}
    }
}
