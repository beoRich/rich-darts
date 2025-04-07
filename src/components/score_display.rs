use crate::components::calculation::recommendation_calculation;
use crate::domain::Score;
use dioxus::prelude::*;
#[component]
pub fn ScoreDisplay(scores: Signal<Vec<Score>>) -> Element {
    let remaining_column_width = "30";
    let thrown_column_width = "15";

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
                    border: "1px solid black;",
                    thead {
                        tr {
                            th {
                                scope: "col",
                                style: "width: {remaining_column_width}%; white-space: pre; text-align: center;",
                                class: "text-primary px-6 py-3",
                                style: "border-right: 1px solid black; border-radius: 10px;",
                                "Remaining"
                            }
                            th {
                                scope: "col",
                                style: "white-space: pre; text-align: center;",
                                style: "border-right: 1px solid black; border-radius: 10px;",
                                class: "text-secondary px-6 py-3",
                                "Recommendation"
                            }
                            th {
                                scope: "col",
                                style: "width: {thrown_column_width}%; white-space: pre; text-align: center;",
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
                                    class: if i == 0 { "text-4xl bg-accent text-accent-content" },
                                    class: if i % 2 == 0 && i != 0 { "bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "bg-base-300 text-base-content" },
                                    style: "width: {remaining_column_width}%; white-space: pre; text-align: center;  ",
                                    style: "border-right: 1px solid black; border-radius: 10px;",
                                    {format!("{:>3}", a.remaining.to_string())}
                                }
                                td {
                                    class: "px-6 py-4",
                                    class: if i == 0 { "text-4xl bg-accent text-accent-content" },
                                    class: if i % 2 == 0 { "bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "bg-base-300 text-base-content" },
                                    style: "white-space: pre; text-align: center;",
                                    style: "border-right: 1px solid black; border-radius: 10px;",
                                    {recommendation_calculation::basic(a.remaining).display().clone()}
                                }
                                td {
                                    class: "px-6 py-4",
                                    class: if i == 0 { "text-4xl bg-accent text-accent-content" },
                                    class: if i % 2 == 0 { "bg-base-200 text-base-content" },
                                    class: if i % 2 == 1 { "bg-base-300 text-base-content" },
                                    style: "width: {thrown_column_width}%; white-space: pre; text-align: center;",
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
