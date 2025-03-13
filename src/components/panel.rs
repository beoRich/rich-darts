use std::num::ParseIntError;
use dioxus::prelude::*;


#[component]
pub fn Panel() -> Element {
    let init: Vec<u16> = vec![501];
    let currentInput = use_signal(|| 1u16);
    let mut count = use_signal(|| init);
    rsx! {
        div { id: "numbers",
            for a in count() {
                div {
                    {a.to_string()}
                }
            }
        }


        "Enter your latest result:",
        div {
            id: "panelDiv",
            input {id: "numberField", type: "number", maxlength:10, min:0, onchange: move |event| input_changed(count, event)}
            button {id: "confirmButton", class: "bg-purple-200 px-4 py-2 rounded-lg border border-black hover:border-indigo-500 active:scale-95 transition-all", "Ok" }
        }
    }
}

fn input_changed(mut count: Signal<Vec<u16>>, event: Event<FormData>) {
    let result = event.value().parse();
    match result {
        Ok(val) => {
            count.write().push(val)
        }
        Err(_) => {}
    }
}
