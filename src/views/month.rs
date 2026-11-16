use dioxus::prelude::*;

use crate::components::card::*;
use crate::components::input::Input;
use crate::components::select::*;

use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIter, strum::Display)]
enum IncomeType {
    #[strum(to_string = "ЗП")]
    Salary,
    #[strum(to_string = "Подработки")]
    SideJobs,
    #[strum(to_string = "Подарки")]
    Gifts,
    #[strum(to_string = "Прочее")]
    Other,
}

impl IncomeType {
    const fn emoji(&self) -> &'static str {
        match self {
            IncomeType::Salary => "💼",
            IncomeType::SideJobs => "🧰",
            IncomeType::Gifts => "🎁",
            IncomeType::Other => "📌",
        }
    }
}

#[component]
pub fn Month() -> Element {
    rsx! {
        Card { style: "width: 150%; max-width: 40rem;",
            CardHeader {
                CardTitle { "Доходы за месяц" }
            }
            CardContent {
                div {
                    style: "display: flex; justify-content: space-between; gap: 1rem;",

                    Input {
                        r#type: "date",
                        id: "date",
                        style: "width: 11rem;",
                    }
                    Input {
                        id: "income_name",
                        placeholder: "Поступление",
                        style: "flex: 1;",
                    }

                    Input {
                        id: "amount",
                        placeholder: "Сумма",
                        style: "width: 150px; text-align: right;",
                    }

                    IncomeTypeSelect {}
                }
            }
        }
    }
}

#[component]
pub fn IncomeTypeSelect() -> Element {
    let options = IncomeType::iter().enumerate().map(|(i, t)| {
        rsx! {
            SelectOption::<IncomeType> {
                index: i,
                value: t,
                text_value: "{t.emoji()}",
                "{t.emoji()} {t}"
                SelectItemIndicator {}
            }
        }
    });

    rsx! {
        Select::<IncomeType> {
            placeholder: "Тип",
            SelectTrigger {
                aria_label: "Тип",
                width: "5rem",
                SelectValue {}
            }
            SelectList {
                aria_label: "Список типов дохода",
                style: "width: 12rem;",
                SelectGroup {
                    SelectGroupLabel { "Доходы" }
                    {options}
                }
            }
        }
    }
}
