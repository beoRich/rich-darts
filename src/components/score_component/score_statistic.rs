use dioxus::prelude::*;
use crate::components::calculation::statistic_calculation::{first_three_average, live_average, AverageValue};
use crate::domain::{Score};

#[component]
pub fn ScoreStatistic(
    scores: Signal<Vec<Score>>,
) -> Element {
    let mut leg_avg_signal = use_signal(move || AverageValue::NoValue);
    let mut leg_throws_signal = use_signal(move || AverageValue::NoValue);
    let mut first_nine_avg_signal = use_signal(move || AverageValue::NoValue);
    let mut hundred_plus_signal = use_signal(move || AverageValue::NoValue);
    use_memo(move ||
         {
             let avg_value = live_average(scores());
             leg_avg_signal.set(avg_value);
             let first_nine_avg_value = first_three_average(scores());
             first_nine_avg_signal.set(first_nine_avg_value);
             if scores().len() > 1 {
                 leg_throws_signal.set(AverageValue::HasValue(((scores.len() - 1) * 3) as u16));
                 let hundred_amount = scores().iter().filter(|val| val.thrown >=100).count() as u16;
                 hundred_plus_signal.set(AverageValue::HasValue(hundred_amount))
             }
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
                    "Average (l|s|m)"
                }
                div {
                    class: "stat-value text-primary",
                    {format!("{}|30|35", {leg_avg_signal().display()})}


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
                    "Double Quote (m)"
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
                    "#Throws (l|s|m)"
                }
                div {
                    class: "stat-value text-primary",
                    {format!("{}|12|15", {leg_throws_signal().display()})}
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
                    "First 9 (l|s|m)"
                }
                div {
                    class: "stat-value text-primary",
                    {format!("{}|100|120", {first_nine_avg_signal().display()})}
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
                    "100+ (l|s|m)"
                }
                div {
                    class: "stat-value text-primary",
                    {format!("{}|5|20", {hundred_plus_signal().display()})}
                }
                div {
                    class: "stat-desc",
                    "3 more than in the previous set"
                }
            }
        }
    }
}
