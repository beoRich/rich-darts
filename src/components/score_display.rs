use crate::domain::CurrentScore;
use dioxus::prelude::*;

#[component]
pub fn ScoreDisplay(count: Signal<Vec<CurrentScore>>) -> Element {
    rsx! {
      div {
            id:"BottomHalf",
            class:"bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 overflow-y-auto",
            div { id: "numbers",
                    class: "table-container",
                table {
                    thead {
                        class: "text-xs uppercase bg-neutral-content",
                        tr {
                            th {
                                scope:"col",
                                style:"white-space: pre; text-align: center;",
                                class:"text-primary px-6 py-3",
                                "Thrown"
                            },
                            th {
                                scope:"col",
                                style:"white-space: pre; text-align: center;",
                                class:"text-secondary px-6 py-3",
                                "Remaining"
                            }
                        }
                    }
                    tbody {
                        id: "numbers-body",
                        for (i, a) in count().into_iter().rev().enumerate() {
                            tr {
                                    td {
                                        class: if i == 0 {"px-6 py-4 bg-accent text-accent-content"},
                                        class: if i % 2 == 0 && i!=0 {"px-6 py-4 bg-base-200 text-base-content"},
                                        class: if i % 2 == 1 {"px-6 py-4 bg-base-300 text-base-content"},
                                        style:"white-space: pre; text-align: center;",
                                        {format!("{:>3}", a.thrown.to_string())}
                                    },
                                    td {
                                        class: if i == 0 {"px-6 py-4 bg-accent text-accent-content"},
                                        class: if i % 2 == 0 && i!=0 {"px-6 py-4 bg-base-200 text-base-content"},
                                        class: if i % 2 == 1 {"px-6 py-4 bg-base-300 text-base-content"},
                                        style:"white-space: pre; text-align: center;",
                                        {format!("{:>3}", a.remaining.to_string())}
                                    },
                            }
                        }
                    }

                }
            }
      }

    }
}
