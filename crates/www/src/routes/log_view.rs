// Copyright 2025 Fernando Borretti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use error::Fallible;
use maud::html;
use shared::date::Date;

use crate::routes::food_view::FoodViewHandler;
use crate::routes::log_delete::LogDeleteHandler;
use crate::routes::log_new::LogNewHandler;
use crate::ui::*;
use crate::www::ServerState;

pub struct LogViewHandler {}

impl LogViewHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/log/{date}", get(handler))
    }

    pub fn url(date: Date) -> String {
        format!("/log/{date}")
    }
}

async fn handler(
    State(state): State<ServerState>,
    Path(date): Path<String>,
) -> Fallible<(StatusCode, Html<String>)> {
    let date: Date = Date::try_from(date)?;

    let nav = default_nav("today");

    // Get database lock and query entries for this date
    let db = state.db.try_lock()?;
    let entries = db.list_entries(date)?;

    // Format the date nicely
    let formatted_date = date.humanize();

    // Build table of logged foods
    let table_content = if entries.is_empty() {
        empty_state("No food logged for this date.")
    } else {
        let columns = vec![
            TableColumn {
                header: "Time".to_string(),
                numeric: false,
            },
            TableColumn {
                header: "Food".to_string(),
                numeric: false,
            },
            TableColumn {
                header: "Brand".to_string(),
                numeric: false,
            },
            TableColumn {
                header: "Amount".to_string(),
                numeric: false,
            },
            TableColumn {
                header: "Energy (kcal)".to_string(),
                numeric: true,
            },
            TableColumn {
                header: "Protein (g)".to_string(),
                numeric: true,
            },
            TableColumn {
                header: "Fat (g)".to_string(),
                numeric: true,
            },
            TableColumn {
                header: "Sat Fat (g)".to_string(),
                numeric: true,
            },
            TableColumn {
                header: "Carbs (g)".to_string(),
                numeric: true,
            },
            TableColumn {
                header: "Fiber (g)".to_string(),
                numeric: true,
            },
            TableColumn {
                header: "Sodium (mg)".to_string(),
                numeric: true,
            },
            TableColumn {
                header: "".to_string(),
                numeric: false,
            },
        ];

        // Calculate daily totals
        let mut total_energy = 0.0;
        let mut total_protein = 0.0;
        let mut total_fat = 0.0;
        let mut total_fat_saturated = 0.0;
        let mut total_carbs = 0.0;
        let mut total_fibre = 0.0;
        let mut total_sodium = 0.0;

        for entry in &entries {
            if let Ok(food) = db.get_food(entry.food_id) {
                let multiplier = if let Some(serving_id) = entry.serving_id {
                    // If there's a serving, get its details
                    if let Ok(servings) = db.list_servings(food.food_id) {
                        if let Some(serving) = servings.iter().find(|s| s.serving_id == serving_id)
                        {
                            serving.serving_amount / 100.0
                        } else {
                            0.01
                        }
                    } else {
                        0.01
                    }
                } else {
                    // No serving, use base unit
                    0.01
                };

                let factor = entry.amount * multiplier;
                total_energy += food.energy * factor;
                total_protein += food.protein * factor;
                total_fat += food.fat * factor;
                total_fat_saturated += food.fat_saturated * factor;
                total_carbs += food.carbs * factor;
                total_fibre += food.fibre * factor;
                total_sodium += food.sodium * factor;
            }
        }

        // Render table rows
        let rows = html! {
            @for entry in &entries {
                @if let Ok(food) = db.get_food(entry.food_id) {
                    @let (unit, multiplier) = if let Some(serving_id) = entry.serving_id {
                        // If there's a serving, get its details
                        if let Ok(servings) = db.list_servings(food.food_id) {
                            if let Some(serving) = servings.iter().find(|s| s.serving_id == serving_id) {
                                (serving.serving_name.clone(), serving.serving_amount / 100.0)
                            } else {
                                (food.serving_unit.as_str().to_string(), 0.01)
                            }
                        } else {
                            (food.serving_unit.as_str().to_string(), 0.01)
                        }
                    } else {
                        // No serving, use base unit
                        (food.serving_unit.as_str().to_string(), 0.01)
                    };

                    @let factor = entry.amount * multiplier;
                    @let energy = food.energy * factor;
                    @let protein = food.protein * factor;
                    @let fat = food.fat * factor;
                    @let fat_saturated = food.fat_saturated * factor;
                    @let carbs = food.carbs * factor;
                    @let fibre = food.fibre * factor;
                    @let sodium = food.sodium * factor;

                    tr {
                        td { (entry.created_at.format("%H:%M").to_string()) }
                        td {
                            a href=(FoodViewHandler::url(food.food_id)) {
                                (food.name)
                            }
                        }
                        td {
                            @if food.brand.is_empty() {
                                "—"
                            } @else {
                                (food.brand)
                            }
                        }
                        td { (format!("{:.1}{}", entry.amount, unit)) }
                        td.numeric { (format!("{:.0}", energy)) }
                        td.numeric { (format!("{:.1}", protein)) }
                        td.numeric { (format!("{:.1}", fat)) }
                        td.numeric { (format!("{:.1}", fat_saturated)) }
                        td.numeric { (format!("{:.1}", carbs)) }
                        td.numeric { (format!("{:.1}", fibre)) }
                        td.numeric { (format!("{:.0}", sodium)) }
                        td {
                            (form_button("Delete", &LogDeleteHandler::url(date, entry.entry_id)))
                        }
                    }
                }
            }
        };

        let totals_summary = summary_box(
            "Daily Totals",
            html! {
                (summary_table(html! {
                    tr {
                        td { "Energy" }
                        td.numeric { (format!("{:.0} kcal", total_energy)) }
                        td.target-info { "Target: 2,000 kcal" }
                    }
                    tr {
                        td { "Protein" }
                        td.numeric { (format!("{:.1} g", total_protein)) }
                        td {}
                    }
                    tr {
                        td { "Fat" }
                        td.numeric { (format!("{:.1} g", total_fat)) }
                        td {}
                    }
                    tr {
                        td { "Saturated Fat" }
                        @if total_fat_saturated > 15.0 {
                            td.numeric.over-limit { (format!("{:.1} g", total_fat_saturated)) }
                            td.target-info.over-limit { "Limit: 15g (EXCEEDED)" }
                        } @else {
                            td.numeric { (format!("{:.1} g", total_fat_saturated)) }
                            td.target-info { "Limit: 15g" }
                        }
                    }
                    tr {
                        td { "Carbohydrate" }
                        td.numeric { (format!("{:.1} g", total_carbs)) }
                        td {}
                    }
                    tr {
                        td { "Fiber" }
                        td.numeric { (format!("{:.1} g", total_fibre)) }
                        td {}
                    }
                    tr {
                        td { "Sodium" }
                        td.numeric { (format!("{:.0} mg", total_sodium)) }
                        td.target-info { "Limit: 2,300 mg" }
                    }
                }))
            },
        );

        html! {
            (data_table(columns, rows))
            (totals_summary)
        }
    };

    let content = html! {
        (panel(&format!("Daily Log — {}", formatted_date), html! {
            (button_bar(html! {
                (button_link("← Yesterday", &LogViewHandler::url(date.prev_day())))
                (button_link("Today", &LogViewHandler::url(date)))
                (button_link("Tomorrow →", &LogViewHandler::url(date.next_day())))
                (spacer())
                (button_link_primary("Log Food", &LogNewHandler::url(date)))
            }))
            (table_content)
        }))
    };

    let html_page = page("Daily Log — zetanom", nav, content);
    Ok((StatusCode::OK, Html(html_page.into_string())))
}
