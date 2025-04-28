use crate::domain::{Leg, Match, Set, SetStatus};
use crate::{backend, Route};
use dioxus::prelude::*;
use tracing::debug;
#[component]
pub fn BreadCrumbComponent(
    only_home: bool,
    match_signal: Option<Signal<Match>>,
    set_signal: Option<Signal<Set>>,
    leg_signal: Option<Signal<Leg>>,
) -> Element {
    //debug!( "Breadcrum set_signal {:?}, leg_signal {:?}", set_signal, leg_signal );
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
                if match_signal.is_some() {
                    li {
                        Link {
                            to: Route::WrapDisplaySets {
                                match_id: match_signal.as_ref().unwrap()().id,
                            },
                            class: "text-xl",
                            {{ format!("Match {}", match_signal.as_ref().unwrap()().title) }}
                        }
                    }
                    if set_signal.is_none() {
                        li {
                            class: "text-xl",
                            "List of Sets"
                        }
                    }
                }
                if set_signal.is_some() {
                    li {
                        Link {
                            to: Route::WrapDisplayLegs {
                                match_id: match_signal.as_ref().unwrap()().id,
                                set_id: set_signal.as_ref().unwrap()().id,
                            },
                            class: "text-xl",
                            if set_signal.as_ref().unwrap()().status == SetStatus::Finished.value() {
                                {
                                    {
                                        format!(
                                            "set {} finished ({} legs)",
                                            set_signal.as_ref().unwrap()().set_order.to_string(),
                                            set_signal.as_ref().unwrap()().leg_amount.to_string(),
                                        )
                                    }
                                }
                            } else {
                                {
                                    {
                                        format!(
                                            "Set {} ({} legs to win)",
                                            set_signal.as_ref().unwrap()().set_order.to_string(),
                                            set_signal.as_ref().unwrap()().leg_amount.to_string(),
                                        )
                                    }
                                }
                            }
                        }
                    }
                    if leg_signal.is_none() {
                        li {
                            class: "text-xl",
                            "List of legs"
                        }
                    }
                }
                if leg_signal.is_some() {
                    li {
                        Link {
                            to: Route::WrapDisplayScore {
                                match_id: match_signal.as_ref().unwrap()().id,
                                set_id: set_signal.as_ref().unwrap()().id,
                                leg_id: leg_signal.as_ref().unwrap()().id,
                            },
                            class: "text-xl",
                            {
                                {
                                    format!(
                                        "Leg {}/{} ({})",
                                        leg_signal.as_ref().unwrap()().leg_order.to_string(),
                                        set_signal.as_ref().unwrap()().leg_amount.to_string(),
                                        leg_signal.as_ref().unwrap()().status,
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
