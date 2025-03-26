use crate::components::calculations;
use crate::components::enter_panel::EnterPanel;
use crate::components::score_display::ScoreDisplay;
use crate::domain::ErrorMessageMode::{CreateNewLeg, TechnicalError};
use crate::domain::ScoreMessageMode::{LegFinished, NewShot, UndoLastShot};
use crate::domain::{ErrorMessageMode, IdOrder, Leg, Score, ScoreMessageMode, INIT_SCORE};
use crate::{backend, Route};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_logger::tracing::error;
use crate::components::breadcrumb::BreadCrumbComponent;

#[component]
pub fn MainScoreComponent(match_signal: Signal<u16>, set_signal: Signal<IdOrder>, leg_signal: Signal<IdOrder>) -> Element {
    let mut raw_input = use_signal(|| "".to_string());
    let mut scores = use_signal(|| vec![]);

    let mut score_message = use_signal(|| NewShot);
    let mut error_message = use_signal(|| ErrorMessageMode::None);

    let mut allow_score = use_signal(|| true);

    use_memo(move || {
        let allow: bool = { score_message }.read().to_owned() != LegFinished && { error_message }
            .read()
            .to_owned()
            != CreateNewLeg;
        allow_score.set(allow)
    });

    use_resource(move || async move {
        let init_score_val = backend::list_score(leg_signal().id).await;
        match init_score_val {
            Ok(val) if !val.is_empty() => scores.set(val),
            _ => error_message.set(CreateNewLeg),
        };
    });

    rsx! {

        div {
            id: "All",
            class: "container-self",


            div {
                BreadCrumbComponent {match_signal, set_signal, leg_signal}


                div {
                    EnterPanel {scores, raw_input, set_signal, leg_signal, error_message, score_message, allow_score}
                    ScoreDisplay {scores}
                }

            }

        }
   }


}

