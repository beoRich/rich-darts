use dioxus::prelude::*;
use crate::components::calculation::statistic_calculation::{live_average, AverageValue};
use crate::domain::{Leg, Score, Set};

#[component]
pub fn ScoreStatistic(
    scores: Signal<Vec<Score>>,
) -> Element {
    let mut leg_avg_signal = use_signal(move || AverageValue::NoValue);
    use_memo(move ||
         {
             let avg_value = live_average(scores());
             leg_avg_signal.set(avg_value)
         }
    );
    rsx! {
        div {
            id: "ScoreStatisticsRow1",
            class: "join",
            div {
                class: "stat join-item",
                div {
                    class: "stat-figure text-primary",
                }
                div {
                    class: "stat-title",
                    "Average"
                }
                div {
                    class: "stat-value text-primary",
                    {leg_avg_signal().display()}
                }
                div {
                    class: "stat-desc",
                    "Tendency: downwards"
                }
            }

            div {
                class: "stat join-item",
                div {
                    class: "stat-figure text-primary",
                }
                div {
                    class: "stat-title",
                    "First 9"
                }
                div {
                    class: "stat-value text-primary",
                    "76"
                }
                div {
                    class: "stat-desc",
                    "+10 compared to average"
                }
            }

            div {
                class: "stat join-item",
                div {
                    class: "stat-figure text-primary",
                }
                div {
                    class: "stat-title",
                    "Double Quote"
                }
                div {
                    class: "stat-value text-primary",
                    "4/10 (40%)"
                }
                div {
                    class: "stat-desc",
                    "Improving"
                }
            }

            div {
                class: "stat join-item",
                div {
                    class: "stat-figure text-primary",
                }
                div {
                    class: "stat-title",
                    "100+"
                }
                div {
                    class: "stat-value text-primary",
                    "0"
                }
                div {
                    class: "stat-desc",
                    "0.3 less than average"
                }
            }
        }
    }
}
