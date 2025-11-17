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
use chrono::NaiveDate;
use error::AppError;
use error::Fallible;
use maud::Markup;
use maud::html;

use crate::ui::page;
use crate::www::ServerState;

pub struct LogViewHandler {}

impl LogViewHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/log/{date}", get(handler))
    }
}

async fn handler(
    State(state): State<ServerState>,
    Path(date): Path<String>,
) -> Fallible<(StatusCode, Html<String>)> {
    let date: NaiveDate = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| AppError::new(format!("Failed to parse date: '{date}'.")))?;

    // Get database lock and query entries for this date
    let db = state.db.try_lock()?;
    let entries = db.list_entries(date)?;

    // Build table of logged foods
    let table: Markup = if entries.is_empty() {
        html! {
            p {
                "No food logged for this date."
            }
        }
    } else {
        html! {
            table {
                thead {
                    tr {
                        th { "Food" }
                        th { "Brand" }
                        th { "Amount" }
                        th { "Unit" }
                        th { "Energy (kcal)" }
                        th { "Protein (g)" }
                        th { "Fat (g)" }
                        th { "Carbs (g)" }
                    }
                }
                tbody {
                    @for entry in &entries {
                        @if let Ok(food) = db.get_food(entry.food_id) {
                            @let (unit, multiplier) = if let Some(serving_id) = entry.serving_id {
                                // If there's a serving, get its details
                                if let Ok(servings) = db.list_servings(food.food_id) {
                                    if let Some(serving) = servings.iter().find(|s| s.serving_id == serving_id) {
                                        (serving.serving_name.clone(), serving.serving_amount)
                                    } else {
                                        (food.serving_unit.as_str().to_string(), 1.0)
                                    }
                                } else {
                                    (food.serving_unit.as_str().to_string(), 1.0)
                                }
                            } else {
                                // No serving, use base unit
                                (food.serving_unit.as_str().to_string(), 1.0)
                            };
                            tr {
                                td { (food.name) }
                                td { (food.brand) }
                                td { (format!("{:.1}", entry.amount)) }
                                td { (unit) }
                                td { (format!("{:.1}", food.energy * entry.amount * multiplier)) }
                                td { (format!("{:.1}", food.protein * entry.amount * multiplier)) }
                                td { (format!("{:.1}", food.fat * entry.amount * multiplier)) }
                                td { (format!("{:.1}", food.carbs * entry.amount * multiplier)) }
                            }
                        }
                    }
                }
            }
        }
    };

    let body: Markup = html! {
        h2 {
            (format!("Log: {date}"))
        }
        p {
            a href=(format!("/log/{date}/new")) {
                "Log Food"
            }
        }
        (table)
    };
    let html: Markup = page("zetanom", body);
    Ok((StatusCode::OK, Html(html.into_string())))
}
