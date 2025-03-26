use dioxus::prelude::*;
use crate::{backend, Route};
use crate::domain::{IdOrder, Leg, Set};

#[component]
pub fn BreadCrumbComponent(
    match_signal: Option<Signal<u16>>,
    set_signal: Option<Signal<IdOrder>>,
    leg_signal: Option<Signal<IdOrder>>,
) -> Element {
    rsx! {
        div {
          class:"breadcrumbs text-sm",
          ul {
                    li {
                        Link {to: Route::DisplayMatches , class:"text-xl", "Matches"}

                    },

                    if match_signal.is_some() {
                        li {
                            Link {to: Route::WrapDisplaySets {matchval: match_signal.unwrap()()},
                            class:"text-xl",
                            {format!{"Match {}", match_signal.unwrap()().to_string()}}}
                        }
                     }

                    if set_signal.is_some() {
                        li {
                            Link {to: Route::WrapDisplayLegs {matchval: match_signal.unwrap()(), set_order: set_signal.unwrap()().order, set_id: set_signal.unwrap()().id},
                            class:"text-xl",
                            {format!{"Set {}", set_signal.unwrap()().order.to_string()}}}
                        }
                    }

                    if leg_signal.is_some() {
                        li {
                            Link {to: Route::WrapDisplayScore {matchval: match_signal.unwrap()(), setval: set_signal.unwrap()().id,
                            legval: leg_signal.unwrap()().id
                        },
                            class:"text-xl",
                            {format!{"Leg {}", leg_signal.unwrap()().order.to_string()}}}
                        }
                    }
                },
        }
    }
}
