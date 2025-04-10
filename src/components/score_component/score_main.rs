use crate::components::breadcrumb::BreadCrumbComponent;
use crate::components::score_component::enter_panel::{OkUndoButton, NumberFieldError, NewCancelButton};
use crate::components::score_component::score_display::ScoreDisplay;
use crate::domain::ErrorMessageMode::{CreateNewLeg, TechnicalError};
use crate::domain::ScoreMessageMode::{LegCancelled, LegFinished, UndoLastShot};
use crate::domain::{
    parse_score_message, ErrorMessageMode, Leg, Score, ScoreMessageMode, Set, SetStatus,
};
use crate::{backend, Route};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_logger::tracing::error;
use tracing::debug;
use crate::components::score_component::score_statistic::ScoreStatistic;

#[component]
pub fn ScoreComponent(match_id: u16, set_input: Set, leg_input: Leg) -> Element {
    debug!("ScoreComponent leg {:?}", leg_input);
    let set_signal = use_signal(|| set_input.clone());
    let leg_signal = use_signal(|| leg_input.clone());
    let mut legs_signal = use_signal(|| vec![]);
    let mut raw_input = use_signal(|| "".to_string());
    let mut scores = use_signal(|| vec![]);
    let mut score_message = use_signal(|| ScoreMessageMode::NewShot);
    let mut error_message = use_signal(|| ErrorMessageMode::None);
    let mut allow_score = use_signal(|| true);
    //only used because popup can have transfer of variable since it s called via js showModal()
    let new_score_signal: Signal<Option<Score>> = use_signal(|| None);
    let mut double_attempt_option_signal: Signal<Vec<u16>> = use_signal(|| vec![0,1,2,3]);

    use_memo(move || {
        if set_signal().status == SetStatus::Finished.value() {
            score_message.set(ScoreMessageMode::SetFinished)
        } else {
            score_message.set(parse_score_message(leg_signal().status))
        }
    });
    use_memo(move || {
        allow_score.set(score_message().allow_score() && error_message().allow_score())
    });
    let _ = use_resource(move || async move {
        let init_score_val = backend::api::dart_score::list_score(leg_signal().id).await;
        match init_score_val {
            Ok(val) if !val.is_empty() => scores.set(val),
            _ => error_message.set(CreateNewLeg),
        };
    });
    let _ = use_resource(move || async move {
        let res = backend::api::dart_leg::list_leg_with_last_score(set_signal().id).await;
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
                    match_id,
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
                                double_attempt_option_signal
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
                                double_attempt_option_signal
                            }
                        }
                        div {
                            id: "LegStatistic",
                            class: "col-span-4 grid bg-base-100 border-y-4 shadow-md rounded px-8",
                            ScoreStatistic {scores}
                        }
                        div {
                            id: "CancelUndoButton",
                            class: "col-span-1 grid bg-base-100 border-y-4 shadow-md rounded px-8 py-4",
                            NewCancelButton {
                                match_id,
                                scores,
                                set_signal,
                                leg_signal,
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
