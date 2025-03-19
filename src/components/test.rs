use crate::backend;
use dioxus::prelude::*;
use dioxus::logger::tracing::info;

#[component]
pub fn Test() -> Element {
    let mut favorites = use_server_future(backend::list_throws)?.suspend()?;

    rsx! {
        div { id: "favorites",
            "test2"
            }
             div {
                id: "ButtonDiv",
                    button {id: "confirmButton", onclick: move |_| { info!("button clicked") }, "CLICK ME" }
            }

    }
}
