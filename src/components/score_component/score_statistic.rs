use crate::backend;
use crate::components::calculation::statistic_calculation::StatAbsolutValue::HasValue;
use crate::components::calculation::statistic_calculation::{
    calculate_compare_value, double_percentage, enhance_average_value, enhance_set_average,
    first_three_average, leg_live_average, parse_stat_absolut_value, CompareValue,
    StatAbsolutValue,
};
use crate::domain::ErrorMessageMode::CreateNewLeg;
use crate::domain::{Leg, LegStatus, Metric, Score, Set};
use dioxus::prelude::*;
use tracing::debug;
#[component]
pub fn ScoreStatistic(
    set_signal: Signal<Set>,
    init_set_metric_signal: Signal<Option<Metric>>,
    leg_signal: Signal<Leg>,
    legs_signal: Signal<Vec<Leg>>,
    scores: Signal<Vec<Score>>,
) -> Element {
    let mut leg_avg_signal = use_signal(move || StatAbsolutValue::NoValue);
    let mut set_avg_signal = use_signal(move || StatAbsolutValue::NoValue);
    let avg_trend_signal =
        use_memo(move || calculate_compare_value(&leg_avg_signal(), &set_avg_signal()));
    let mut leg_throws_signal = use_signal(move || StatAbsolutValue::NoValue);
    let mut set_throws_signal = use_signal(move || StatAbsolutValue::NoValue);
    let throws_trend_signal =
        use_memo(move || calculate_compare_value(&leg_throws_signal(), &set_throws_signal()));
    let mut leg_first_nine_avg_signal = use_signal(move || StatAbsolutValue::NoValue);
    let mut leg_hundred_plus_signal = use_signal(move || StatAbsolutValue::NoValue);
    let mut set_hundred_plus_signal = use_signal(move || StatAbsolutValue::NoValue);
    let hundred_plus_trend_signal = use_memo(move || {
        calculate_compare_value(&leg_hundred_plus_signal(), &set_hundred_plus_signal())
    });
    let mut double_succ_signal = use_signal(move || StatAbsolutValue::NoValue);
    let mut double_attempts_signal = use_signal(move || StatAbsolutValue::NoValue);
    let double_percentage_signal =
        use_memo(move || double_percentage(double_succ_signal(), double_attempts_signal()));
    use_effect(move || {
        let leg_avg_value = leg_live_average(scores());
        leg_avg_signal.set(leg_avg_value);
        let mut leg_throws_value = StatAbsolutValue::NoValue;
        let mut leg_hundred_amount_value = StatAbsolutValue::NoValue;
        let mut unique_legs: Vec<Leg> = legs_signal()
            .into_iter()
            .filter(|leg| leg.id != leg_signal().id)
            .collect();
        unique_legs.push(leg_signal());
        let mut double_succ_value: u16 = unique_legs
            .into_iter()
            .filter(|leg| leg.status == LegStatus::Finished.display())
            .count() as u16;
        let mut double_attempt_value = 0;
        if scores().len() > 1 {
            leg_throws_value = HasValue(((scores.len() - 1) * 3) as u16);
            leg_hundred_amount_value =
                HasValue(scores().iter().filter(|val| val.thrown >= 100).count() as u16);
            let leg_double_attempts: Vec<u16> = scores()
                .iter()
                .filter_map(|score| score.double_attempt)
                .collect();
            if !leg_double_attempts.is_empty() {
                double_attempt_value = leg_double_attempts.iter().sum()
            }
        }
        match init_set_metric_signal() {
            Some(metric) => {
                let Metric {
                    throws,
                    hundred_plus_amount,
                    amount_of_legs,
                    double_attempts: set_double_attempts,
                    ..
                } = metric;
                double_attempt_value += set_double_attempts;
                set_avg_signal.set(enhance_set_average(scores(), &metric));
                let (set_throw_value, set_hundred_amount_value) =
                    if leg_signal().status == LegStatus::Finished.display() {
                        (
                            enhance_average_value(Some(&leg_throws_value), throws, amount_of_legs),
                            enhance_average_value(
                                Some(&leg_hundred_amount_value),
                                hundred_plus_amount,
                                amount_of_legs,
                            ),
                        )
                    } else {
                        (
                            enhance_average_value(None, throws, amount_of_legs),
                            enhance_average_value(None, hundred_plus_amount, amount_of_legs),
                        )
                    };
                set_throws_signal.set(set_throw_value);
                set_hundred_plus_signal.set(set_hundred_amount_value);
            }
            _ => {}
        }
        double_succ_signal.set(parse_stat_absolut_value(double_succ_value));
        double_attempts_signal.set(parse_stat_absolut_value(double_attempt_value));
        leg_hundred_plus_signal.set(leg_hundred_amount_value);
        leg_throws_signal.set(leg_throws_value);
        let first_nine_avg_value = first_three_average(scores());
        leg_first_nine_avg_signal.set(first_nine_avg_value);
    });
    use_effect(move || {});
    let double_title_element = rsx!(
        div {
            class: "stat-title",
            "Double Quote (m)"
        }
    );
    let double_value_element = rsx!(
        div {
            class: "stat-value text-primary",
            "{double_succ_signal().display()}/{double_attempts_signal()} ({double_percentage_signal().display()}%)"
        
        }
    );
    rsx! {
        div {
            id: "ScoreStatisticsRow1",
            class: "join",
            StatisticPanelDifferentiated {
                title_input: "#Average ",
                hover_text: "Score average per Leg(L)/Set(S)/Match(M, not yet implemented)",
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
                hover_text: "Amount of throws per Leg(L)/Set(S, averaged on leg)/Match(M, not yet implemented)",
                leg_stat_signal: leg_throws_signal,
                set_stat_signal: set_throws_signal,
                desc_input: throws_trend_signal().display(),
            }
            StatisticPanelDifferentiated {
                title_input: "First 9 ",
                hover_text: "Score average of first 9 per Leg(L)/Set(S)/Match(M, not yet implemented)",
                leg_stat_signal: leg_first_nine_avg_signal,
                set_stat_signal: set_avg_signal,
                desc_input: "+10 compared to average",
            }
            StatisticPanelDifferentiated {
                title_input: "100+ ",
                hover_text: "Amount of 100+ throws per Leg(L)/Set(S, averaged on leg)/Match(M, not yet implemented)",
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
    hover_text: String,
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
            title: {hover_text},
            class: "stat-value text-primary",
            LegSetMatchDisplay {
                leg_val: {leg_stat_signal().display()},
                set_val: {set_stat_signal().display()},
                match_val: {set_stat_signal().display()},
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
