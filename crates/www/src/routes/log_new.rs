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
use db::FoodListEntry;
use db::ServingId;
use error::AppError;
use error::Fallible;
use maud::Markup;
use maud::html;
use serde::Deserialize;

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
}

async fn get_handler(
    State(state): State<ServerState>,
    Path(date): Path<String>,
) -> Fallible<(StatusCode, Html<String>)> {
    // TODO: show the user a list of foods. Clicking on a food sends them to the next page.
    todo!()
}

async fn get_handler_with_food_id(
    State(state): State<ServerState>,
    Path(date): Path<String>,
    Path(food_id): Path<FoodId>,
) -> Fallible<(StatusCode, Html<String>)> {
    // TODO: show the user a form to log the selected food.
    todo!()
}

#[derive(Deserialize)]
struct LogFoodForm {
    food_id: FoodId,
    serving_id: String,
    amount: f64,
}

async fn post_handler(
    State(state): State<ServerState>,
    Path(date): Path<String>,
    Path(food_id): Path<FoodId>,
    Form(form): Form<LogFoodForm>,
) -> Fallible<Redirect> {
    todo!()
}
