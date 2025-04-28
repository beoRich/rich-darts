use crate::components::breadcrumb::BreadCrumbComponent;
use crate::domain::Match;
use crate::{backend, Route};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use tracing::debug;
#[component]
pub fn DisplayMatches() -> Element {
    let mut matches = use_signal(|| vec![]);
    let _ = use_resource(move || async move {
        let res = backend::api::dart_match::list_matches().await;
        match res {
            Ok(val) if !val.is_empty() => matches.set(val),
            _ => {}
        };
    });
    let mut match_name_raw_signal: Signal<String> = use_signal(|| "".to_string());
    let mut match_name_signal: Signal<Option<String>> = use_signal(|| Some("".to_string()));
    use_effect(move || {
        let binding = match_name_raw_signal();
        let title = binding.trim();
        if title.is_empty() {
            match_name_signal.set(None)
        } else {
            match_name_signal.set(Some(title.to_string()))
        }
    });
    rsx! {
        div {
            id: "DisplayMatches",
            class: "container-self",
            div {
                BreadCrumbComponent {
                    only_home: false,
                }
                div {
                    class: "bg-base-100 border-y-4 shadow-md rounded px-8 pt-6 pb-8 grid grid-cols-12 gap-4",
                    button {
                        id: "newLegButton",
                        onclick: move |_| async move {
                            let _ = new_match(matches, match_name_signal).await;
                        },
                        class: "btn btn-soft btn-primary",
                        "New Match"
                    }
                    label {
                        class: "floating-label col-span-2 grid",
                        span {
                            "Match Title"
                        }
                        input {
                            id: "textField",
                            autofocus: true,
                            value: "",
                            placeholder: "Optional Title",
                            class: "input input-primary text-xl shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline",
                            r#type: "text",
                            oninput: move |e| match_name_raw_signal.set((*e.value()).parse().unwrap()),
                            onkeypress: move |e| async move {
                                let key = e.key();
                                if key == Key::Enter {
                                    let _ = new_match(matches, match_name_signal).await;
                                }
                            },
                        
                        }
                    }
                }
                div {
                    MatchTable {
                        matches,
                    }
                }
            
            }
        
        }
    }
}
async fn new_match(
    mut matches: Signal<Vec<Match>>,
    match_name_signal: Signal<Option<String>>,
) -> Result<(), ServerFnError> {
    debug!("new match {:?}", match_name_signal());
    let new_match = backend::api::dart_match::new_match(match_name_signal()).await?;
    matches.push(new_match.clone());
    Ok(())
}
#[component]
pub fn MatchTable(matches: Signal<Vec<Match>>) -> Element {
    rsx! {
        div {
            id: "BottomHalf",
            class: "bg-neutral shadow-md rounded px-8 pt-6 pb-8 mb-4 overflow-y-auto",
            div {
                id: "numbers",
                class: "table-container",
                table {
                    class: "text-xl bg-neutral-content",
                    thead {
                        tr {
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                class: "text-primary px-6 py-3",
                                "Title (click me)"
                            }
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                class: "text-secondary px-6 py-3",
                                "Status"
                            }
                        }
                    }
                    tbody {
                        id: "numbers-body",
                        for (i , a) in matches().into_iter().rev().enumerate() {
                            tr {
                                td {
                                    class: if i == 0 { "px-6 py-4 bg-accent text-accent-content" },
                                    class: if i % 2 == 0 && i != 0 { "px-6 py-4 bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "px-6 py-4 bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    li {
                                        Link {
                                            to: Route::WrapDisplaySets {
                                                match_id: a.id,
                                            },
                                            class: "link",
                                            {a.title}
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
                            }
                        }
                    }
                
                }
            }
        }
    }
}
