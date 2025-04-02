use crate::domain::{IdOrder, Leg, Set, SetStatus};
use crate::{backend, Route};
use dioxus::prelude::*;
use tracing::debug;
#[component]
pub fn BreadCrumbComponent(
    only_home: bool,
    match_id: Option<u16>,
    set_input: Option<Set>,
    leg_input: Option<Leg>,
) -> Element {
    debug!(
        "Breadcrum set_signal {:?}, leg_signal {:?}",
        set_input, leg_input
    );
    rsx! {
        div {
            class: "breadcrumbs text-sm px-8 border-2 bg-base-100",
            ul {
                li {
                    Link {
                        to: Route::HomeScreen,
                        class: "text-xl",
                        "Home"
                    }
                }
                if !only_home {
                    li {
                        Link {
                            to: Route::DisplayMatches,
                            class: "text-xl",
                            "Matches"
                        }
                    }
                }
                if match_id.is_some() {
                    li {
                        Link {
                            to: Route::WrapDisplaySets {
                                matchval: match_id.unwrap(),
                            },
                            class: "text-xl",
                            {{ format!("Match {}", match_id.unwrap().to_string()) }}
                        }
                    }
                }
                if set_input.is_none() {
                    li {
                        class: "text-xl",
                        "List of sets"
                    }
                }
                if set_input.is_some() {
                    li {
                        Link {
                            to: Route::WrapDisplayLegs {
                                matchval: match_id.unwrap(),
                                set_id: set_input.as_ref().unwrap().id,
                            },
                            class: "text-xl",
                            if set_input.as_ref().unwrap().status == SetStatus::Finished.value() {
                                {
                                    {
                                        format!(
                                            "Set {} Finished ({} legs)",
                                            set_input.as_ref().unwrap().set_order.to_string(),
                                            set_input.as_ref().unwrap().leg_amount.to_string(),
                                        )
                                    }
                                }
                            } else {
                                {
                                    {
                                        format!(
                                            "Set {} ({} legs to win)",
                                            set_input.as_ref().unwrap().set_order.to_string(),
                                            set_input.as_ref().unwrap().leg_amount.to_string(),
                                        )
                                    }
                                }
                            }
                        }
                    }
                    if leg_input.is_none() {
                        li {
                            class: "text-xl",
                            "List of legs"
                        }
                    }
                }
                if leg_input.is_some() {
                    li {
                        Link {
                            to: Route::WrapDisplayScore {
                                matchval: match_id.unwrap(),
                                set_id: set_input.as_ref().unwrap().id,
                                leg_id: leg_input.as_ref().unwrap().id,
                            },
                            class: "text-xl",
                            {
                                {
                                    format!(
                                        "Leg {}/{}",
                                        leg_input.as_ref().unwrap().leg_order.to_string(),
                                        set_input.as_ref().unwrap().leg_amount.to_string(),
                                    )
                                }
                            }
                        }
                    }
                    li {
                        class: "text-xl",
                        "Score"
                    }
                }
            }
        }
    }
}
