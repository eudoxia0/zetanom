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
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use db::FoodListEntry;
use error::Fallible;
use maud::Markup;
use maud::html;

use crate::ui::page;
use crate::www::ServerState;

pub struct FoodListHandler {}

impl FoodListHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/library", get(handler))
    }

    pub fn url() -> &'static str {
        "/library"
    }
}

async fn handler(State(state): State<ServerState>) -> Fallible<(StatusCode, Html<String>)> {
    let db = state.db.try_lock()?;
    let foods: Vec<FoodListEntry> = db.list_foods()?;
    let list: Markup = html! {
        ul {
            @for food in &foods {
                li {
                    a href={(format!("/library/{}", food.food_id))} {
                        (food.name) " — " (food.brand)
                    }
                }
            }
        }
    };
    let body: Markup = html! {
        h1 {
            "Library"
        }
        p {
            a href="/library/new" {
                "Add New Food"
            }
        }
        (list)
    };
    let html: Markup = page("zetanom", body);
    Ok((StatusCode::OK, Html(html.into_string())))
}
