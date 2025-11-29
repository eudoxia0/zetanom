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
use maud::html;

use crate::db::FoodListEntry;
use crate::error::Fallible;
use crate::routes::food_new::FoodNewHandler;
use crate::routes::food_view::FoodViewHandler;
use crate::ui::*;
use crate::www::ServerState;

pub struct FoodListHandler {}

impl FoodListHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(handler))
    }

    pub fn url() -> &'static str {
        "/library"
    }
}

async fn handler(State(state): State<ServerState>) -> Fallible<(StatusCode, Html<String>)> {
    let nav = default_nav("food_list");

    let db = state.db.try_lock()?;
    let foods: Vec<FoodListEntry> = db.list_foods()?;

    let table_content = if foods.is_empty() {
        empty_state("No foods in library yet.")
    } else {
        let columns = vec![
            TableColumn {
                header: "Name".to_string(),
                numeric: false,
            },
            TableColumn {
                header: "Brand".to_string(),
                numeric: false,
            },
        ];

        let rows = html! {
            @for food in &foods {
                tr {
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
                }
            }
        };

        data_table(columns, rows)
    };

    let content = html! {
        (panel("Food Library", html! {
            (button_bar(html! {
                (button_link_primary("Add New Food", FoodNewHandler::url()))
            }))
            (table_content)
        }))
    };

    let html_page = page("Food Library — zetanom", nav, content);
    Ok((StatusCode::OK, Html(html_page.into_string())))
}
