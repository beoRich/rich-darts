use crate::domain::Score;
use crate::components::calculation::recommendation_calculation;
use dioxus::prelude::*;
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
                    style: "width: 65%",
                    thead {
                        tr {
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                class: "text-primary px-6 py-3",
                                "Remaining"
                            }
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                class: "text-secondary px-6 py-3",
                                "Thrown"
                            }
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                class: "text-info px-6 py-3",
                                "Recommendation"
                            }
                        }
                    }
                    tbody {
                        id: "numbers-body",
                        for (i , a) in scores().into_iter().rev().enumerate() {
                            tr {
                                td {
                                    class: "px-6 py-4",
                                    class: if i == 0 { "text-4xl bg-accent text-accent-content" },
                                    class: if i % 2 == 0 && i != 0 { "bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    {format!("{:>3}", a.remaining.to_string())}
                                }
                                td {
                                    class: "px-6 py-4",
                                    class: if i == 0 { "text-4xl bg-accent text-accent-content" },
                                    class: if i % 2 == 0 { "bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    {format!("{:>3}", a.thrown.to_string())}
                                }
                                td {
                                    class: "px-6 py-4",
                                    class: if i == 0 { "text-4xl bg-accent text-accent-content" },
                                    class: if i % 2 == 0 { "bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    {recommendation_calculation::basic(a.remaining).display().clone()}
                                }
                            }
                        }
                    }
                
                }
            }
        }
    }
}
