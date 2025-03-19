use crate::backend;
use dioxus::prelude::*;

#[component]
pub fn Test() -> Element {
    let mut favorites = use_resource(backend::list_throws).suspend()?;

    rsx! {
        div { id: "favorites",
            "test2"
            }

    }
}
