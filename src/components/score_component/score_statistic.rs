use crate::components::calculation::statistic_calculation::{first_three_average, leg_live_average, enhance_set_average, StatAbsolutValue, enhance_average_value, CompareValue, calculate_compare_value};
use crate::domain::{Leg, LegStatus, Metric, Score, Set};
use dioxus::prelude::*;
use tracing::debug;
use crate::backend;
use crate::components::calculation::statistic_calculation::StatAbsolutValue::HasValue;
use crate::domain::ErrorMessageMode::CreateNewLeg;

#[component]
pub fn ScoreStatistic(set_signal: Signal<Set>, init_set_metric_signal: Signal<Option<Metric>>, leg_signal: Signal<Leg>, scores: Signal<Vec<Score>>) -> Element {
    let mut leg_avg_signal = use_signal(move || StatAbsolutValue::NoValue);
    let mut set_avg_signal = use_signal(move || StatAbsolutValue::NoValue);
    let mut avg_trend_signal = use_signal(move || CompareValue::NoValue);

    let mut leg_throws_signal = use_signal(move || StatAbsolutValue::NoValue);
    let mut set_throws_signal = use_signal(move || StatAbsolutValue::NoValue);
    let mut throws_trend_signal = use_signal(move || CompareValue::NoValue);

    let mut leg_first_nine_avg_signal = use_signal(move || StatAbsolutValue::NoValue);

    let mut leg_hundred_plus_signal = use_signal(move || StatAbsolutValue::NoValue);
    let mut set_hundred_plus_signal = use_signal(move || StatAbsolutValue::NoValue);
    let mut hundred_plus_trend_signal = use_signal(move || CompareValue::NoValue);

    let mut double_attempts_signal = use_signal(move|| StatAbsolutValue::NoValue);

    use_memo(move || {
        let leg_avg_value = leg_live_average(scores());
        leg_avg_signal.set(leg_avg_value);
        let mut leg_throws_value = StatAbsolutValue::NoValue;
        let mut leg_hundred_amount_value = StatAbsolutValue::NoValue;
        let mut leg_double_attempt_value = StatAbsolutValue::NoValue;
        if scores().len() > 1 {
            leg_throws_value = HasValue(((scores.len() - 1) * 3) as u16);
            leg_hundred_amount_value = HasValue(scores().iter().filter(|val| val.thrown >= 100).count() as u16);
            let leg_double_attempts: Vec<u16> = scores().iter().filter_map(|score| score.double_attempt).collect();
            if !leg_double_attempts.is_empty() {
                leg_double_attempt_value = HasValue(leg_double_attempts.iter().sum())
            }

        }
        match init_set_metric_signal() {
            Some(metric) =>  {
                let Metric{throws,hundred_plus_amount, amount_of_legs, .. } = metric;
                set_avg_signal.set(enhance_set_average(scores(), &metric));
                let (set_throw_value, set_hundred_amount_value) = if leg_signal().status == LegStatus::Finished.display() {
                    (enhance_average_value(Some(&leg_throws_value), throws, amount_of_legs),
                     enhance_average_value(Some(&leg_hundred_amount_value), hundred_plus_amount, amount_of_legs))
                } else {
                    (enhance_average_value(None, throws, amount_of_legs),
                    enhance_average_value(None, hundred_plus_amount, amount_of_legs))
                };
                set_throws_signal.set(set_throw_value);
                set_hundred_plus_signal.set(set_hundred_amount_value);
            }
            _ => {}
        }
        double_attempts_signal.set(leg_double_attempt_value);
        leg_hundred_plus_signal.set(leg_hundred_amount_value);
        leg_throws_signal.set(leg_throws_value);
        let first_nine_avg_value = first_three_average(scores());
        leg_first_nine_avg_signal.set(first_nine_avg_value);
    });

    use_memo(move || {
        avg_trend_signal.set(calculate_compare_value(&leg_avg_signal(), &set_avg_signal()));
        throws_trend_signal.set(calculate_compare_value(&leg_throws_signal(), &set_throws_signal()));
        hundred_plus_trend_signal.set(calculate_compare_value(&leg_hundred_plus_signal(), &set_hundred_plus_signal()));
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
            "4/{double_attempts_signal()} (40%)"
        
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
                desc_input: avg_trend_signal().display(),
            }
            StatisticPanelBase {
                title_element: double_title_element,
                value_element: double_value_element,
                desc_input: "Improving",
            }
            StatisticPanelDifferentiated {
                title_input: "#Throws ",
                leg_stat_signal: leg_throws_signal,
                set_stat_signal: set_throws_signal,
                desc_input: throws_trend_signal().display(),
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
                set_stat_signal: set_hundred_plus_signal,
                desc_input: hundred_plus_trend_signal().display(),
            }
        }
    }
}
#[component]
pub fn StatisticPanelDifferentiated(
    title_input: String,
    leg_stat_signal: Signal<StatAbsolutValue>,
    set_stat_signal: Signal<StatAbsolutValue>,
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
