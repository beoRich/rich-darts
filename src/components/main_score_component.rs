use crate::components::breadcrumb::BreadCrumbComponent;
use crate::components::calculations;
use crate::components::enter_panel::{Buttons, NumberFieldError};
use crate::components::score_display::ScoreDisplay;
use crate::domain::ErrorMessageMode::{CreateNewLeg, TechnicalError};
use crate::domain::ScoreMessageMode::{LegCancelled, LegFinished, UndoLastShot};
use crate::domain::{
    parse_score_message, ErrorMessageMode, IdOrder, Leg, Score, ScoreMessageMode, Set, SetStatus,
    INIT_SCORE,
};
use crate::{backend, Route};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_logger::tracing::error;
use tracing::debug;
use web_sys::js_sys::JSON::parse;
use web_sys::window;
#[component]
pub fn MainScoreComponent(match_id: u16, set_input: Set, leg_input: Leg) -> Element {
    debug!("MainScoreComponent leg {:?}", leg_input);
    let set_signal = use_signal(|| set_input);
    let leg_signal = use_signal(|| leg_input);
    let mut raw_input = use_signal(|| "".to_string());
    let mut scores = use_signal(|| vec![]);
    let mut score_message = use_signal(|| ScoreMessageMode::NewShot);
    let mut error_message = use_signal(|| ErrorMessageMode::None);
    let mut allow_score = use_signal(|| true);
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
    use_resource(move || async move {
        let init_score_val = backend::api::dart_score::list_score(leg_signal().id).await;
        match init_score_val {
            Ok(val) if !val.is_empty() => scores.set(val),
            _ => error_message.set(CreateNewLeg),
        };
    });
    rsx! {
        div {
            id: "DisplayScore",
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
                        id: "EnterPanel",
                        class: "bg-base-100 border-y-4 border-color-red-500 shadow-md rounded px-8 pt-6 pb-8",
                        NumberFieldError {
                            scores,
                            raw_input,
                            set_signal,
                            leg_signal,
                            error_message,
                            score_message,
                            allow_score,
                        }
                        Buttons {
                            scores,
                            raw_input,
                            set_signal,
                            leg_signal,
                            error_message,
                            score_message,
                            allow_score,
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
