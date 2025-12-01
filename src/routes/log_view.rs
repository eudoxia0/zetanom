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
use chrono::Local;
use maud::Markup;
use maud::html;

use crate::db::Db;
use crate::db::Entry;
use crate::db::FoodEntry;
use crate::error::Fallible;
use crate::routes::food_view::FoodViewHandler;
use crate::routes::log_delete::LogDeleteHandler;
use crate::routes::log_new::LogNewHandler;
use crate::types::Date;
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
    let db = state.db.try_lock()?;
    let entries: Vec<Entry> = db.list_entries(date)?;
    let tbl = render_log_table(&db, &entries, date)?;
    let content = html! {
        .button-bar {
            a .button href=(LogViewHandler::url(date.prev_day())) {
                "← Previous"
            }
            a .button href=(LogViewHandler::url(Date::today())) {
                "Today"
            }
            a .button href=(LogViewHandler::url(date.next_day())) {
                "Next →"
            }
            .spacer {}
            a .button href=(LogNewHandler::url(date)) {
                "Log Food"
            }
        }
        (tbl)
    };
    let title = format!("Log: {}", date.humanize());
    let html_page = page(&title, content);
    Ok((StatusCode::OK, Html(html_page.into_string())))
}

fn render_log_table(db: &Db, entries: &[Entry], date: Date) -> Fallible<Markup> {
    if entries.is_empty() {
        Ok(html! {
            p {
                "No food logged for this date."
            }
        })
    } else {
        Ok(html! {
            table {
                thead {
                    tr {
                        th {
                            "Time"
                        }
                        th {
                            "Food"
                        }
                        th {
                            "Brand"
                        }
                        th {
                            "Amount"
                        }
                        th .numeric {
                            "Energy (kcal)"
                        }
                        th .numeric {
                            "Protein (g)"
                        }
                        th .numeric {
                            "Fat (g)"
                        }
                        th .numeric {
                            "Sat Fat (g)"
                        }
                        th .numeric {
                            "Carbs (g)"
                        }
                        th .numeric {
                            "Fiber (g)"
                        }
                        th .numeric {
                            "Sodium (mg)"
                        }
                        th {
                            ""
                        }
                    }
                }
                tbody {
                    @for entry in entries {
                        (render_log_entry_row(db, entry, date)?)
                    }
                }
            }
        })
    }
}

fn render_log_entry_row(db: &Db, entry: &Entry, date: Date) -> Fallible<Markup> {
    let food: FoodEntry = db.get_food(entry.food_id)?;

    let (unit, multiplier): (String, f64) = if let Some(serving_id) = entry.serving_id {
        // If there's a serving, get its details.
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

    let factor = entry.amount * multiplier;
    let energy = food.energy * factor;
    let protein = food.protein * factor;
    let fat = food.fat * factor;
    let fat_saturated = food.fat_saturated * factor;
    let carbs = food.carbs * factor;
    let fibre = food.fibre * factor;
    let sodium = food.sodium * factor;

    let time_str = entry
        .created_at
        .with_timezone(&Local)
        .format("%H:%M")
        .to_string();
    let amount_str = format!("{:.1} {}", entry.amount, unit);
    let energy_str = format!("{:.0}", energy);
    let protein_str = format!("{:.1}", protein);
    let fat_str = format!("{:.1}", fat);
    let fat_saturated_str = format!("{:.1}", fat_saturated);
    let carbs_str = format!("{:.1}", carbs);
    let fibre_str = format!("{:.1}", fibre);
    let sodium_str = format!("{:.0}", sodium);

    Ok(html! {
        tr {
            td .center {
                (time_str)
            }
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
            td {
                (amount_str)
            }
            td .numeric {
                (energy_str)
            }
            td .numeric {
                (protein_str)
            }
            td .numeric {
                (fat_str)
            }
            td .numeric {
                (fat_saturated_str)
            }
            td .numeric {
                (carbs_str)
            }
            td .numeric {
                (fibre_str)
            }
            td .numeric {
                (sodium_str)
            }
            td .center {
                form method="POST" action=(LogDeleteHandler::url(date, entry.entry_id)) {
                    input .button type="submit" value="Delete";
                }
            }
        }
    })
}
