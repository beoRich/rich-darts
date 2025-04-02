use crate::components::breadcrumb::BreadCrumbComponent;
use crate::domain::{IdOrder, Leg, Set};
use crate::{backend, Route};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use tracing::debug;
#[component]
pub fn DisplayLegs(match_signal: Signal<u16>, set_signal: Signal<Set>) -> Element {
    debug!("DisplayLegs set_signal {:?}", set_signal);
    let mut legs_signal = use_signal(|| vec![]);
    let mut start_score_raw_signal: Signal<String> = use_signal(|| "501".to_string());
    let mut start_score_test_signal: Signal<bool> = use_signal(|| true);
    let mut start_score_signal: Signal<u16> = use_signal(|| 501);
    use_memo(move || {
        let raw_val = start_score_raw_signal();
        let result = raw_val.parse::<u16>();
        start_score_test_signal.set(result.is_ok() && result.clone().unwrap() > 0);
        result.map(|val| start_score_signal.set(val))
    });
    use_resource(move || async move {
        let res = backend::api::dart_leg::list_leg_with_last_score(set_signal().id).await;
        match res {
            Ok(val) if !val.is_empty() => legs_signal.set(val),
            _ => {}
        };
    });
    rsx! {
        div {
            id: "DisplayLegs",
            class: "container-self",
            div {
                BreadCrumbComponent {
                    only_home: false,
                    match_signal,
                    set_signal,
                    leg_signal: None,
                }
                div {
                    class: "bg-base-100 border-y-4 border-color-red-500 shadow-md rounded px-8 pt-6 pb-8 grid grid-cols-12 gap-4",
                    button {
                        id: "newLegButton",
                        onclick: move |_| async move {
                            let _ = new_leg(set_signal().id, legs_signal, start_score_signal()).await;
                        },
                        class: "btn btn-soft btn-primary col-span-1 grid",
                        disabled: if !start_score_test_signal() { "true" },
                        "New Leg"
                    }
                    input {
                        id: "numberField",
                        autofocus: true,
                        value: "501",
                        class: "text-1xl shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline\
                        col-span-1 grid",
                        r#type: "number",
                        oninput: move |e| start_score_raw_signal.set((*e.value()).parse().unwrap()),
                        onfocusin: move |_| {
                            document::eval(&"document.getElementById('numberField').select()".to_string());
                        },
                        onkeypress: move |e| async move {
                            let key = e.key();
                            if key == Key::Enter && start_score_test_signal() {
                                let _ = new_leg(set_signal().id, legs_signal, start_score_signal()).await;
                            }
                        },
                    
                    }
                
                }
                div {
                    LegTable {
                        match_signal,
                        set_signal,
                        legs_signal,
                    }
                }
            
            }
        
        }
    }
}
async fn new_leg(
    set_signal_id: u16,
    mut legs_signal: Signal<Vec<Leg>>,
    score_max: u16,
) -> Result<(), ServerFnError> {
    let new_leg = backend::api::dart_leg::new_leg_init_score(set_signal_id, score_max).await?;
    legs_signal.push(new_leg);
    Ok(())
}
#[component]
pub fn LegTable(
    match_signal: Signal<u16>,
    set_signal: Signal<Set>,
    legs_signal: Signal<Vec<Leg>>,
) -> Element {
    rsx! {
        div {
            id: "BottomHalf",
            class: "bg-neutral shadow-md rounded px-8 pt-6 pb-8 mb-4 overflow-y-auto",
            div {
                id: "numbers",
                class: "table-container ",
                table {
                    class: "text-xl bg-neutral-content rounded-lg",
                    style: "width: 40%",
                    thead {
                        tr {
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                class: "text-primary px-6 py-3",
                                "#Leg (click me)"
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
                                "Start Score"
                            }
                        }
                    }
                    tbody {
                        id: "numbers-body",
                        for (nr , leg) in legs_signal().into_iter().rev().enumerate() {
                            tr {
                                td {
                                    class: if nr == 0 { "px-6 py-4 bg-accent text-accent-content" },
                                    class: if nr % 2 == 0 && nr != 0 { "px-6 py-4 bg-base-200 text-base-content" },
                                    class: if nr % 2 == 1 { "px-6 py-4 bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    li {
                                        Link {
                                            to: Route::WrapDisplayScore {
                                                matchval: match_signal(),
                                                set_id: set_signal().id,
                                                leg_id: leg.id,
                                            },
                                            {leg.leg_order.to_string()}
                                        }
                                    }
                                
                                }
                                td {
                                    class: if nr == 0 { "px-6 py-4 bg-accent text-accent-content" },
                                    class: if nr % 2 == 0 && nr != 0 { "px-6 py-4 bg-base-200 text-base-content" },
                                    class: if nr % 2 == 1 { "px-6 py-4 bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    {format!("{:>3}", leg.status)}
                                }
                                td {
                                    class: if nr == 0 { "px-6 py-4 bg-accent text-accent-content" },
                                    class: if nr % 2 == 0 && nr != 0 { "px-6 py-4 bg-base-200 text-base-content" },
                                    class: if nr % 2 == 1 { "px-6 py-4 bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    {format!("{:>3}", leg.start_score)}
                                }
                            }
                        }
                    }
                
                }
            }
        }
    }
}
