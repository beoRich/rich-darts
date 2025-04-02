use crate::components::breadcrumb::BreadCrumbComponent;
use crate::domain::ErrorMessageMode::CreateNewLeg;
use crate::domain::{Leg, Set, INIT_SCORE};
use crate::{backend, Route};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use tracing::debug;
#[component]
pub fn DisplaySets(match_id: u16) -> Element {
    let mut match_signal = use_signal(|| match_id);
    let mut sets_signal = use_signal(|| vec![]);
    let mut leg_amount_raw_signal: Signal<String> = use_signal(|| "5".to_string());
    let mut leg_amount_test_signal: Signal<bool> = use_signal(|| true);
    let mut leg_amount_signal: Signal<u16> = use_signal(|| 5);
    use_memo(move || {
        let raw_val = leg_amount_raw_signal();
        let result = raw_val.parse::<u16>();
        leg_amount_test_signal.set(result.is_ok() && result.clone().unwrap() > 0);
        result.map(|val| leg_amount_signal.set(val))
    });
    use_resource(move || async move {
        let match_val = match_signal();
        let res = backend::api::dart_set::list_set(match_val as i32).await;
        match res {
            Ok(val) if !val.is_empty() => sets_signal.set(val),
            _ => {}
        };
    });
    rsx! {
        div {
            id: "DisplaySets",
            class: "container-self",
            div {
                BreadCrumbComponent {
                    only_home: false,
                    match_id,
                    set_input: None,
                    leg_input: None,
                }
                div {
                    class: "bg-base-100 border-y-4 border-color-red-500 shadow-md rounded px-8 pt-6 pb-8 grid grid-cols-12 gap-4",
                    button {
                        id: "newLegButton",
                        onclick: move |_| async move {
                            let _ = new_set(match_signal(), sets_signal, leg_amount_signal()).await;
                        },
                        class: "btn btn-soft btn-primary",
                        disabled: if !leg_amount_test_signal() { "true" },
                        "New Set"
                    
                    }
                    input {
                        id: "numberField",
                        autofocus: true,
                        placeholder: "#winlegs",
                        class: "input text-1xl shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline\
                             col-span-1 grid",
                        r#type: "number",
                        oninput: move |e| leg_amount_raw_signal.set((*e.value()).parse().unwrap()),
                        onfocusin: move |_| {
                            document::eval(&"document.getElementById('numberField').select()".to_string());
                        },
                        onkeypress: move |e| async move {
                            let key = e.key();
                            if key == Key::Enter && leg_amount_test_signal() {
                                let _ = new_set(match_signal(), sets_signal, leg_amount_signal()).await;
                            }
                        },
                    
                    }
                }
                div {
                    SetTable {
                        match_signal,
                        sets_signal,
                    }
                }
            
            }
        
        }
    }
}
async fn new_set(
    match_id: u16,
    mut sets_signal: Signal<Vec<Set>>,
    leg_amount: u16,
) -> Result<(), ServerFnError> {
    let new_set = backend::api::dart_set::new_set(match_id, leg_amount).await?;
    sets_signal.push(new_set);
    Ok(())
}
#[component]
pub fn SetTable(mut match_signal: Signal<u16>, mut sets_signal: Signal<Vec<Set>>) -> Element {
    debug!("{:?}", sets_signal());
    rsx! {
        div {
            id: "BottomHalf",
            class: "bg-neutral shadow-md rounded px-8 pt-6 pb-8 mb-4 overflow-y-auto",
            div {
                id: "numbers",
                class: "table-container",
                table {
                    class: "text-xl bg-neutral-content rounded",
                    style: "width: 60%",
                    thead {
                        tr {
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                class: "text-primary px-6 py-3",
                                "#Set (click me)"
                            }
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                class: "text-secondary px-6 py-3",
                                "Status"
                            }
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                class: "text-secondary px-6 py-3",
                                "#Legs to win "
                            }
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                class: "text-secondary px-6 py-3",
                                "Best of"
                            }
                        }
                    }
                    tbody {
                        id: "numbers-body",
                        for (i , a) in sets_signal().into_iter().rev().enumerate() {
                            tr {
                                td {
                                    class: if i == 0 { "px-6 py-4 bg-accent text-accent-content" },
                                    class: if i % 2 == 0 && i != 0 { "px-6 py-4 bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "px-6 py-4 bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    li {
                                        Link {
                                            to: Route::WrapDisplayLegs {
                                                matchval: match_signal(),
                                                set_id: a.id,
                                            },
                                            {a.set_order.to_string()}
                                        }
                                    }
                                
                                }
                                td {
                                    class: if i == 0 { "px-6 py-4 bg-accent text-accent-content" },
                                    class: if i % 2 == 0 && i != 0 { "px-6 py-4 bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "px-6 py-4 bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    {format!("{:>3}", a.status)}
                                }
                                td {
                                    class: if i == 0 { "px-6 py-4 bg-accent text-accent-content" },
                                    class: if i % 2 == 0 && i != 0 { "px-6 py-4 bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "px-6 py-4 bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    {format!("{:>3}", a.leg_amount)}
                                }
                                td {
                                    class: if i == 0 { "px-6 py-4 bg-accent text-accent-content" },
                                    class: if i % 2 == 0 && i != 0 { "px-6 py-4 bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "px-6 py-4 bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    {format!("{:>3}", a.best_of)}
                                }
                            }
                        }
                    }
                
                }
            }
        }
    }
}
