use crate::components::calculation::recommendation_calculation;
use crate::domain::Score;
use dioxus::prelude::*;
use crate::components::calculation::recommendation_calculation::{display_score_types, FinishRecValue, NonFinishRecValue, RecValue};

#[component]
pub fn ScoreDisplay(scores: Signal<Vec<Score>>) -> Element {

    rsx! {
        div {
            id: "BottomHalf",
            class: "bg-neutral shadow-md rounded px-8 pt-6 pb-8 mb-4 ",
            div {
                id: "numbers",
                class: "table-container",
                table {
                    class: "text-xl uppercase bg-neutral-content rounded",
                    style: "width: 50%; border: 1px solid black;",
                    thead {
                        tr {
                            th {
                                scope: "col",
                                style: "width: 30%; white-space: pre; text-align: center;",
                                class: "text-primary px-6 py-3",
                                style: "border-right: 1px solid black; border-radius: 10px;",
                                "Remaining"
                            }
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                style: "width: 50%; border-right: 1px solid black; border-radius: 10px;",
                                class: "text-secondary px-6 py-3",
                                "Recommendation"
                            }
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                class: "text-info px-6 py-3",
                                style: "border-right: 1px solid black; border-radius: 10px;",
                                "Thrown"
                            }
                        }
                    }
                    tbody {
                        id: "numbers-body",
                        for (i , a) in scores().into_iter().rev().enumerate() {
                            tr {
                                td {
                                    class: "px-6 py-4",
                                    class: if i == 0 { "text-5xl bg-accent text-accent-content" },
                                    class: if i % 2 == 0 && i != 0 { "bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "bg-base-300 text-base-content" },
                                    style: "width: 30%; white-space: pre; text-align: center;  ",
                                    style: "border-right: 1px solid black; border-radius: 10px;",
                                    {format!("{:>3}", a.remaining.to_string())}
                                }
                                td {
                                    key: {a.order},
                                    class: "px-6 py-4",
                                    class: if i == 0 { "text-3xl bg-accent text-accent-content" },
                                    class: if i % 2 == 0 { "bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "bg-base-300 text-base-content" },
                                    style: "width: 50%; white-space: pre; text-align: center;",
                                    style: "border-right: 1px solid black; border-radius: 10px;",
                                    RecommendationRow {rec_value: recommendation_calculation::determine_rec(a.remaining), row_nr: i as u16}

                                }
                                td {
                                    class: "px-6 py-4",
                                    class: if i == 0 { "text-4xl bg-accent text-accent-content" },
                                    class: if i % 2 == 0 { "bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    style: "border-right: 1px solid black; border-radius: 10px;",
                                    {format!("{:>3}", a.thrown.to_string())}
                                }
                            }
                        }
                    }

                }
            }
        }
    }
}

#[component]
fn RecommendationRow(rec_value: RecValue, row_nr: u16) -> Element {
    match rec_value {
        RecValue::IsFinish(finish_rec_value) => {
            rsx! {
                FinishRecommendationRow{finish_rec_value, row_nr}
            }
        }
        RecValue::NoFinish(no_finished_rec_value) => {
            rsx! {
                NoFinishedRecommendationRow{no_finished_rec_value, row_nr}
            }
        }
    }
}

#[component]
fn NoFinishedRecommendationRow(no_finished_rec_value: NonFinishRecValue, row_nr: u16) -> Element {
    let NonFinishRecValue{rec, goal} = no_finished_rec_value;
    rsx! {

                                div {
                                    div {
                                        "{rec} -> {goal}"
                                    }
                                }

    }
}

#[component]
fn FinishRecommendationRow(finish_rec_value: FinishRecValue, row_nr: u16) -> Element {
    let FinishRecValue {primary_rec, secondary_rec} = finish_rec_value;
    let primary_rec_display = display_score_types(&primary_rec);
    let secondary_rec_display = display_score_types(&secondary_rec);
    rsx! {
          if row_nr == 0  {
                                    div {
                                        class: "flex",
                                        div {
                                            class: "card bg-base-300 rounded-box grid h-16 grow place-items-center",
                                            "{primary_rec_display}"
                                        }
                                        if secondary_rec.as_ref().is_some() {
                                            div {
                                                class: "divider divider-secondary divider-horizontal",
                                            }
                                            div {
                                                class: "card bg-base-300 rounded-box grid h-16 grow place-items-center",
                                                "{secondary_rec_display}"
                                            }

                                        }

                                    }

                                }
        if row_nr > 0 {
                                        if secondary_rec.as_ref().is_none() {
                                            {primary_rec_display.clone()}
                                        }

                                        if secondary_rec.as_ref().is_some() {
                                            p {
                                                "{primary_rec_display} | {secondary_rec_display}"}
                                        }
        }
    }
}
