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
use maud::html;

use crate::db::FoodEntry;
use crate::db::FoodId;
use crate::db::Serving;
use crate::error::Fallible;
use crate::routes::food_edit::FoodEditHandler;
use crate::routes::serving_delete::ServingDeleteHandler;
use crate::routes::serving_new::ServingNewHandler;
use crate::ui::*;
use crate::www::ServerState;

pub struct FoodViewHandler {}

impl FoodViewHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/library/{food_id}", get(handler))
    }

    pub fn url(food_id: FoodId) -> String {
        format!("/library/{food_id}")
    }
}

async fn handler(
    State(state): State<ServerState>,
    Path(food_id): Path<FoodId>,
) -> Fallible<(StatusCode, Html<String>)> {
    let db = state.db.try_lock()?;
    let food: FoodEntry = db.get_food(food_id)?;
    let servings: Vec<Serving> = db.list_servings(food_id)?;

    let food_title = if food.brand.is_empty() {
        food.name.clone()
    } else {
        format!("{} — {}", food.name, food.brand)
    };

    // Nutrition table
    let nutrition_table = html! {
        table {
            thead {
                tr {
                    th { "Nutrient" }
                    th.numeric { "Per 100" (food.serving_unit.as_str()) }
                }
            }
            tbody {
                tr {
                    td { "Energy" }
                    td.numeric { (format!("{:.1} kcal", food.energy)) }
                }
                tr {
                    td { "Protein" }
                    td.numeric { (format!("{:.1} g", food.protein)) }
                }
                tr {
                    td { "Fat, Total" }
                    td.numeric { (format!("{:.1} g", food.fat)) }
                }
                tr {
                    td { "— Saturated" }
                    td.numeric { (format!("{:.1} g", food.fat_saturated)) }
                }
                tr {
                    td { "Carbohydrate" }
                    td.numeric { (format!("{:.1} g", food.carbs)) }
                }
                tr {
                    td { "— Sugars" }
                    td.numeric { (format!("{:.1} g", food.carbs_sugars)) }
                }
                tr {
                    td { "Dietary Fibre" }
                    td.numeric { (format!("{:.1} g", food.fibre)) }
                }
                tr {
                    td { "Sodium" }
                    td.numeric { (format!("{:.0} mg", food.sodium)) }
                }
            }
        }
    };

    let content = html! {
        .button-bar {
            a .button href=(FoodEditHandler::url(food_id)) {
                "Edit Food"
            }
        }
        (nutrition_table)
        h2 {
            "Custom Serving Sizes"
        }
        table {
            thead {
                tr {
                    th { "Name" }
                    th { "Equals" }
                    th { "Delete" }
                }
            }
            tbody {
                @for serving in &servings {
                    tr {
                        td {
                            (serving.serving_name)
                        }
                        td {
                            (serving.serving_amount) (food.serving_unit.as_str())
                        }
                        td {
                            form method="post" action=(ServingDeleteHandler::url(food_id, serving.serving_id)) {
                                input .button type="submit" value="Delete";
                            }
                        }
                    }
                }
            }
        }

        h2 {
            "Add Custom Serving Size"
        }
        form method="post" action=(ServingNewHandler::url(food_id)) {
            .form-group {
                (label("serving_name", "Serving Name"))
                (text_input("serving_name", "serving_name", "e.g., cup, slice, package"))
            }
            .form-group {
                (label("serving_amount", &format!("Amount ({})", food.serving_unit.as_str())))
                (number_input("serving_amount", "serving_amount", "0.1", "e.g., 250"))
            }
            input .button type="submit" value="Add Serving";
        }
    };

    let html_page = page(&food_title.to_string(), content);
    Ok((StatusCode::OK, Html(html_page.into_string())))
}
