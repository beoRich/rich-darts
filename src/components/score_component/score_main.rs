use crate::components::breadcrumb::BreadCrumbComponent;
use crate::components::score_component::enter_panel::{
    LegSelectButtons, NumberFieldError, OkUndoButton,
};
use crate::components::score_component::score_display::ScoreDisplay;
use crate::components::score_component::score_statistic::ScoreStatistic;
use crate::domain::ErrorMessageMode::{CreateNewLeg, TechnicalError};
use crate::domain::ScoreMessageMode::{LegCancelled, LegFinished, UndoLastShot};
use crate::domain::{parse_score_message, ErrorMessageMode, Leg, Match, Score, ScoreMessageMode, Set, SetStatus};
use crate::{backend, Route};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_logger::tracing::error;
use tracing::debug;
#[component]
pub fn ScoreComponent(match_signal: Signal<Match>, set_input: Set, leg_signal: Signal<Leg>) -> Element {
    debug!("ScoreComponent leg {:?}", leg_signal());
    let set_signal = use_signal(|| set_input.clone());
    let mut legs_signal = use_signal(|| vec![]);
    let mut raw_input = use_signal(|| "".to_string());
    let mut scores = use_signal(|| vec![]);
    let mut score_message = use_signal(|| ScoreMessageMode::NewShot);
    let mut error_message = use_signal(|| ErrorMessageMode::None);
    let mut allow_score =
        use_memo(move || score_message().allow_score() && error_message().allow_score());
    //
    //only used because popup can have transfer of variable since it s called via js showModal()
    let new_score_signal: Signal<Option<Score>> = use_signal(|| None);
    let mut double_attempt_option_signal: Signal<Vec<u16>> = use_signal(|| vec![0, 1, 2, 3]);
    let mut leg_signal_id = use_signal(|| 0);
    //hack to avoid big warning and for setting the leg_signal_id only if the value changes
    //needed for use_resources that only load initally
    let mut leg_signal_id2 = use_signal(|| 0);
    let mut init_set_metric_signal = use_signal(move || None);
    use_effect(move || leg_signal_id2.set(leg_signal_id()));
    use_effect(move || {
        if leg_signal_id2() != leg_signal().id {
            leg_signal_id.set(leg_signal().id)
        }
    });
    let _ = use_resource(move || async move {
        let init_set_metric_value = backend::api::dart_set::get_metrics_of_other_legs_by_set_id(
            set_signal().id,
            leg_signal_id(),
        )
        .await;
        init_set_metric_signal.set(init_set_metric_value.ok());
    });
    use_effect(move || {
        if set_signal().status == SetStatus::Finished.value() {
            score_message.set(ScoreMessageMode::SetFinished)
        } else {
            score_message.set(parse_score_message(leg_signal().status))
        }
    });
    let _ = use_resource(move || async move {
        let init_score_val = backend::api::dart_score::list_score(leg_signal_id()).await;
        match init_score_val {
            Ok(val) if !val.is_empty() => scores.set(val),
            _ => error_message.set(CreateNewLeg),
        };
    });
    let _ = use_resource(move || async move {
        let res = backend::api::dart_leg::list_leg(set_signal().id).await;
        match res {
            Ok(val) if !val.is_empty() => legs_signal.set(val),
            _ => {}
        };
    });
    rsx! {
        div {
            id: "MainScoreComponent",
            class: "container-self",
            div {
                BreadCrumbComponent {
                    only_home: false,
                    match_signal,
                    set_signal,
                    leg_signal,
                }
                div {
                    div {
                        id: "TopBar",
                        class: "grid grid-cols-6 gap-1",
                        div {
                            id: "EnterPanel",
                            class: "col-span-1 grid bg-base-100 border-y-4 shadow-md rounded px-8 py-4",
                            NumberFieldError {
                                scores,
                                raw_input,
                                set_signal,
                                leg_signal,
                                error_message,
                                score_message,
                                allow_score,
                                new_score_signal,
                                double_attempt_option_signal,
                            }
                            OkUndoButton {
                                scores,
                                raw_input,
                                set_signal,
                                leg_signal,
                                error_message,
                                score_message,
                                allow_score,
                                new_score_signal,
                                double_attempt_option_signal,
                            }
                        }
                        div {
                            id: "LegStatistic",
                            class: "col-span-4 grid bg-base-100 border-y-4 shadow-md rounded px-8",
                            ScoreStatistic {
                                set_signal,
                                init_set_metric_signal,
                                leg_signal,
                                legs_signal,
                                scores,
                            }
                        }
                        div {
                            id: "CancelUndoButton",
                            class: "col-span-1 grid bg-base-100 border-y-4 shadow-md rounded px-8 py-4",
                            LegSelectButtons {
                                match_id: match_signal().id,
                                scores,
                                set_signal,
                                leg_signal,
                                legs_signal,
                                error_message,
                                score_message,
                            }
                        }
                    
                    }
                    ScoreDisplay {
                        scores,
                    }
                }
            
            }
        
        }
    }
}
