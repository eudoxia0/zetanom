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
use crate::types::Nutrition;
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
    let totals: Nutrition = calculate_totals(&db, &entries)?;
    let totals: Markup = render_totals(totals);
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
        h2 {
            "Totals"
        }
        (totals)
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
    let nutrition: Nutrition = entry.nutrition(db)?;
    // If there's a custom unit, use that. Otherwise, use the base unit name.
    let unit_name: String = if let Some(serving_id) = entry.serving_id {
        let serving = db.get_serving_by_id(serving_id)?;
        serving.serving_name
    } else {
        food.serving_unit.as_str().to_string()
    };
    let time_str: String = entry
        .created_at
        .with_timezone(&Local)
        .format("%H:%M")
        .to_string();
    let amount_str = format!("{:.0} {}", entry.amount, unit_name);
    let energy_str = format!("{:.0}", nutrition.energy);
    let protein_str = format!("{:.1}", nutrition.protein);
    let fat_str = format!("{:.1}", nutrition.fat);
    let fat_saturated_str = format!("{:.1}", nutrition.fat_saturated);
    let carbs_str = format!("{:.1}", nutrition.carbs);
    let fibre_str = format!("{:.1}", nutrition.fibre);
    let sodium_str = format!("{:.0}", nutrition.sodium);

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

fn calculate_totals(db: &Db, entries: &[Entry]) -> Fallible<Nutrition> {
    let mut n: Nutrition = Nutrition {
        energy: 0.0,
        protein: 0.0,
        fat: 0.0,
        fat_saturated: 0.0,
        carbs: 0.0,
        carbs_sugars: 0.0,
        fibre: 0.0,
        sodium: 0.0,
    };
    for entry in entries {
        let en: Nutrition = entry.nutrition(db)?;
        n = n + en;
    }
    Ok(n)
}

fn humanize_float(f: f64) -> String {
    format!("{:.1}", f)
}

fn render_totals(t: Nutrition) -> Markup {
    let Nutrition {
        energy,
        protein,
        fat,
        fat_saturated,
        carbs,
        carbs_sugars,
        fibre,
        sodium,
    } = t;
    html! {
        table .totals {
            tr {
                th {
                    "Energy"
                }
                td {
                    (humanize_float(energy))
                }
            }
            tr {
                th {
                    "Protein"
                }
                td {
                    (humanize_float(protein))
                }
            }
            tr {
                th {
                    "Fat"
                }
                td {
                    (humanize_float(fat))
                }
            }
            tr {
                th {
                    "Fat — Saturated"
                }
                td {
                    (humanize_float(fat_saturated))
                }
            }
            tr {
                th {
                    "Carbs"
                }
                td {
                    (humanize_float(carbs))
                }
            }
            tr {
                th {
                    "Carbs — Sugars"
                }
                td {
                    (humanize_float(carbs_sugars))
                }
            }
            tr {
                th {
                    "Fibre"
                }
                td {
                    (humanize_float(fibre))
                }
            }
            tr {
                th {
                    "Sodium"
                }
                td {
                    (humanize_float(sodium))
                }
            }
        }
    }
}
