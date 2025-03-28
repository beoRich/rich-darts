use dioxus::prelude::*;
use tracing::debug;
use crate::{backend, Route};
use crate::domain::{IdOrder, Leg, Set};

#[component]
pub fn BreadCrumbComponent(
    match_signal: Option<Signal<u16>>,
    set_signal: Option<Signal<IdOrder>>,
    leg_signal: Option<Signal<Leg>>,
) -> Element {

    debug!("Breadcrum set_signal {:?}, leg_signal {:?}", set_signal, leg_signal);

    rsx! {
        div {
          class:"breadcrumbs text-sm px-8 border-2 bg-base-100",
          ul {
                    li {
                        Link {to: Route::DisplayMatches , class:"text-xl", "Matches"}

                    },

                    if match_signal.is_some() {
                        li {
                            Link {to: Route::WrapDisplaySets {matchval: match_signal.unwrap()()},
                            class:"text-xl text-base-content",
                            {format!{"Match {}", match_signal.unwrap()().to_string()}}}
                        }
                       if set_signal.is_none() {
                            li {
                                    class:"text-xl",
                                    "List of sets"

                            }
                        }
                     }

                    if set_signal.is_some() {
                        li {
                            Link {to: Route::WrapDisplayLegs {matchval: match_signal.unwrap()(), set_id: set_signal.unwrap()().id},
                            class:"text-xl",
                            {format!{"Set {}", set_signal.unwrap()().order.to_string()}}}
                        }
                       if leg_signal.is_none() {
                            li {
                                    class:"text-xl",
                                    "List of legs"

                            }
                        }
                    }

                    if leg_signal.is_some() {
                        li {
                            Link {to: Route::WrapDisplayScore {matchval: match_signal.unwrap()(), set_id: set_signal.unwrap()().id,
                            leg_id: leg_signal.unwrap()().id
                        },
                            class:"text-xl",
                            {format!{"Leg {}", leg_signal.unwrap()().leg_order.to_string()}}}
                        }
                    }
                },
        }
    }
}
