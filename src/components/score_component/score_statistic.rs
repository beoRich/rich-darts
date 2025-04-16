use crate::components::calculation::statistic_calculation::{
    first_three_average, live_average, AverageValue,
};
use crate::domain::{Metric, Score, Set};
use dioxus::prelude::*;
use crate::backend;
use crate::components::calculation::statistic_calculation::AverageValue::HasValue;
use crate::domain::ErrorMessageMode::CreateNewLeg;

#[component]
pub fn ScoreStatistic(set_signal: Signal<Set>, scores: Signal<Vec<Score>>) -> Element {
    let mut leg_avg_signal = use_signal(move || AverageValue::NoValue);
    let mut set_avg_signal = use_signal(move || AverageValue::NoValue);
    let mut leg_throws_signal = use_signal(move || AverageValue::NoValue);
    let mut leg_first_nine_avg_signal = use_signal(move || AverageValue::NoValue);
    let mut leg_hundred_plus_signal = use_signal(move || AverageValue::NoValue);
    let mut init_set_metric = use_signal(move || None);

    let _ = use_resource(move || async move {
        let init_set_metric_value = backend::api::dart_set::get_cascaded_metrics_by_id(set_signal().id).await;
        match init_set_metric_value.clone() {
           Ok(Metric{sum, score_amount}) => {
               if score_amount != 0 {
                   set_avg_signal.set(HasValue(sum/ score_amount));
               }

           },
            _ => {}
        }
        init_set_metric.set(init_set_metric_value.ok())

    });

    use_memo(move || {
        let avg_value = live_average(scores());
        leg_avg_signal.set(avg_value);
        let first_nine_avg_value = first_three_average(scores());
        leg_first_nine_avg_signal.set(first_nine_avg_value);
        if scores().len() > 1 {
            leg_throws_signal.set(AverageValue::HasValue(((scores.len() - 1) * 3) as u16));
            let hundred_amount = scores().iter().filter(|val| val.thrown >= 100).count() as u16;
            leg_hundred_plus_signal.set(AverageValue::HasValue(hundred_amount))
        }
    });
    let double_title_element = rsx!(
        div {
            class: "stat-title",
            "Double Quote (m)"
        }
    );
    let double_value_element = rsx!(
        div {
            class: "stat-value text-primary",
            "4/10 (40%)"
        
        }
    );
    rsx! {
        div {
            id: "ScoreStatisticsRow1",
            class: "join",
            StatisticPanelDifferentiated {
                title_input: "#Average ",
                leg_stat_signal: leg_avg_signal,
                set_stat_signal: set_avg_signal,
                desc_input: "Tendency: downwards",
            }
            StatisticPanelBase {
                title_element: double_title_element,
                value_element: double_value_element,
                desc_input: "Improving",
            }
            StatisticPanelDifferentiated {
                title_input: "#Throws ",
                leg_stat_signal: leg_throws_signal,
                set_stat_signal: set_avg_signal,
                desc_input: "+10 compared to average",
            }
            StatisticPanelDifferentiated {
                title_input: "First 9 ",
                leg_stat_signal: leg_first_nine_avg_signal,
                set_stat_signal: set_avg_signal,
                desc_input: "+10 compared to average",
            }
            StatisticPanelDifferentiated {
                title_input: "100+ ",
                leg_stat_signal: leg_hundred_plus_signal,
                set_stat_signal: set_avg_signal,
                desc_input: "3 more than in the previous set",
            }
        }
    }
}
#[component]
pub fn StatisticPanelDifferentiated(
    title_input: String,
    leg_stat_signal: Signal<AverageValue>,
    set_stat_signal: Signal<AverageValue>,
    desc_input: String,
) -> Element {
    let title_element = rsx! {
        div {
            class: "stat-title",
            {title_input}
            "("
            LegSetMatchDisplay {
                leg_val: "l",
                set_val: "s",
                match_val: "m",
            }
            ")"
        }
    };
    let value_element = rsx!(
        div {
            class: "stat-value text-primary",
            LegSetMatchDisplay {
                leg_val: {leg_stat_signal().display()},
                set_val: {set_stat_signal().display()},
                match_val: "10",
            }
        }
    );
    rsx! {
        StatisticPanelBase {
            title_element,
            value_element,
            desc_input,
        }
    }
}
#[component]
pub fn StatisticPanelBase(
    title_element: Element,
    value_element: Element,
    desc_input: String,
) -> Element {
    use_effect(|| {});
    rsx! {
        div {
            class: "stat join-item",
            div {
                class: "stat-figure text-primary",
            }
            {title_element}
            {value_element}
            div {
                class: "stat-desc",
                {desc_input}
            }
        }
    }
}
#[component]
fn LegSetMatchDisplay(leg_val: String, set_val: String, match_val: String) -> Element {
    rsx! {
        span {
            class: "text-primary",
            "{leg_val}"
        }
        "|"
        span {
            class: "text-secondary",
            "{set_val}"
        }
        "|"
        span {
            class: "text-info",
            "{match_val}"
        }
        ""
    }
}
