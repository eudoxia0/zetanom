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

use axum::Form;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::Redirect;
use axum::routing::get;
use axum::routing::post;
use chrono::Utc;
use maud::html;
use serde::Deserialize;

use crate::db::CreateEntryInput;
use crate::db::FoodId;
use crate::db::ServingId;
use crate::error::Fallible;
use crate::routes::log_view::LogViewHandler;
use crate::types::Date;
use crate::ui::*;
use crate::www::ServerState;

pub struct LogNewHandler {}

impl LogNewHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        let router = router.route("/log/{date}/new", get(get_handler));
        let router = router.route(
            "/log/{date}/new/food/{food_id}",
            get(get_handler_with_food_id),
        );
        router.route("/log/{date}/new/food/{food_id}", post(post_handler))
    }

    pub fn url(date: Date) -> String {
        format!("/log/{date}/new")
    }

    pub fn url_with_food_id(date: Date, food_id: FoodId) -> String {
        format!("/log/{date}/new/food/{food_id}")
    }
}

async fn get_handler(
    State(state): State<ServerState>,
    Path(date): Path<String>,
) -> Fallible<(StatusCode, Html<String>)> {
    let db = state.db.try_lock()?;
    let foods = db.list_foods()?;
    let date = Date::try_from(date)?;

    let table_content = if foods.is_empty() {
        html! {
            p {
                "No foods."
            }
        }
    } else {
        html! {
            table {
                thead {
                    tr {
                        th {
                            "Name"
                        }
                        th {
                            "Brand"
                        }
                    }
                }
                tbody {
                    @for food in &foods {
                        tr {
                            td {
                                a href=(LogNewHandler::url_with_food_id(date, food.food_id)) {
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
                        }
                    }
                }
            }
        }
    };

    let content = table_content;

    let html_page = page("Add Food Entry", content);
    Ok((StatusCode::OK, Html(html_page.into_string())))
}

async fn get_handler_with_food_id(
    State(state): State<ServerState>,
    Path((date, food_id)): Path<(String, FoodId)>,
) -> Fallible<(StatusCode, Html<String>)> {
    let db = state.db.try_lock()?;
    let food = db.get_food(food_id)?;
    let servings = db.list_servings(food_id)?;
    let date = Date::try_from(date)?;

    let food_title = if food.brand.is_empty() {
        food.name.clone()
    } else {
        format!("{} — {}", food.name, food.brand)
    };

    let form_content = html! {
        form .main-form method="post" action=(LogNewHandler::url_with_food_id(date, food_id)) {
            input type="hidden" name="food_id" value={(food_id.to_string())};
            .form-group {
                (label_required("amount", "Amount"))
                (number_input("amount", "amount", "0.1", "e.g., 1.5"))
            }
            .form-group {
                (label_required("serving_id", "Serving Size"))
                (select("serving_id", "serving_id", {
                    let mut options = vec![
                        ("".to_string(), format!("Base serving (100{})", food.serving_unit.as_str()))
                    ];
                    for serving in &servings {
                        options.push((
                            serving.serving_id.to_string(),
                            format!("{} ({} {})", serving.serving_name, serving.serving_amount, food.serving_unit.as_str())
                        ));
                    }
                    options
                }))
            }
            .button-bar {
                input .button type="submit" value="Log Entry";
            }
        }
    };

    let content = form_content;

    let html_page = page(&format!("Log {}", food_title), content);
    Ok((StatusCode::OK, Html(html_page.into_string())))
}

#[derive(Deserialize)]
struct LogFoodForm {
    food_id: FoodId,
    serving_id: String,
    amount: f64,
}

async fn post_handler(
    State(state): State<ServerState>,
    Path((date, _food_id)): Path<(String, FoodId)>,
    Form(form): Form<LogFoodForm>,
) -> Fallible<Redirect> {
    let date = Date::try_from(date)?;

    let serving_id = if form.serving_id.is_empty() {
        None
    } else {
        Some(form.serving_id.parse::<ServingId>()?)
    };

    let input = CreateEntryInput {
        date,
        food_id: form.food_id,
        serving_id,
        amount: form.amount,
        created_at: Utc::now(),
    };

    let db = state.db.try_lock()?;
    db.create_entry(input)?;

    Ok(Redirect::to(&LogViewHandler::url(date)))
}
