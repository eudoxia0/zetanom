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
use chrono::NaiveDate;
use chrono::Utc;
use db::CreateEntryInput;
use db::FoodId;
use db::ServingId;
use error::AppError;
use error::Fallible;
use maud::Markup;
use maud::html;
use serde::Deserialize;

use crate::routes::log_view::LogViewHandler;
use crate::ui::label;
use crate::ui::number_input;
use crate::ui::page;
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

    pub fn url(date: NaiveDate) -> String {
        let date = date.format("%Y-%m-%d");
        format!("/log/{date}/new")
    }

    pub fn url_with_food_id(date: NaiveDate, food_id: FoodId) -> String {
        let date = date.format("%Y-%m-%d");
        format!("/log/{date}/new/food/{food_id}")
    }
}

async fn get_handler(
    State(state): State<ServerState>,
    Path(date): Path<String>,
) -> Fallible<(StatusCode, Html<String>)> {
    let db = state.db.try_lock()?;
    let foods = db.list_foods()?;
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| AppError::new(format!("Failed to parse date: '{date}'.")))?;

    let body: Markup = html! {
        h2 { "Add Food Entry for " (date) }
        ul {
            @for food in &foods {
                li {
                    a href=(LogNewHandler::url_with_food_id(date, food.food_id)) {
                        (food.name) " — " (food.brand)
                    }
                }
            }
        }
    };

    let html = page("Add Food Entry", body);
    Ok((StatusCode::OK, Html(html.into_string())))
}

async fn get_handler_with_food_id(
    State(state): State<ServerState>,
    Path((date, food_id)): Path<(String, FoodId)>,
) -> Fallible<(StatusCode, Html<String>)> {
    let db = state.db.try_lock()?;
    let food = db.get_food(food_id)?;
    let servings = db.list_servings(food_id)?;
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| AppError::new(format!("Failed to parse date: '{date}'.")))?;

    let body: Markup = html! {
        h2 { "Log: " (food.name) " — " (food.brand) }
        form method="post" action=(LogNewHandler::url_with_food_id(date, food_id)){
            input type="hidden" name="food_id" value={(food_id.to_string())};

            (label("serving_id", "Serving"));
            select id="serving_id" name="serving_id" {
                option value="" { "Base serving (" (food.serving_unit.as_str()) ")" }
                @for serving in &servings {
                    option value={(serving.serving_id.to_string())} {
                        (serving.serving_name) " (" (serving.serving_amount) " " (food.serving_unit.as_str()) ")"
                    }
                }
            }
            br;

            (label("amount", "Amount"));
            (number_input("amount"));
            br;

            input type="submit" value="Log Food";
        }
    };

    let html = page("Log Food", body);
    Ok((StatusCode::OK, Html(html.into_string())))
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
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| AppError::new(format!("Failed to parse date: '{date}'.")))?;

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
