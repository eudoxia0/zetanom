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
use crate::routes::food_list::FoodListHandler;
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
                tr style="background: #f8f8f8;" {
                    td { "— Saturated" }
                    td.numeric { (format!("{:.1} g", food.fat_saturated)) }
                }
                tr {
                    td { "Carbohydrate" }
                    td.numeric { (format!("{:.1} g", food.carbs)) }
                }
                tr style="background: #f8f8f8;" {
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

    // Servings section
    let servings_list = if servings.is_empty() {
        html! {
            p {
                "No food logged for this date."
            }
        }
    } else {
        html! {
            div."serving-list" {
                @for serving in &servings {
                    div."serving-item" {
                        span {
                            (serving.serving_name) ": " (serving.serving_amount) (food.serving_unit.as_str())
                        }
                        form method="post" action=(ServingDeleteHandler::url(food_id, serving.serving_id)) style="display: inline;" {
                            button type="submit" { "Delete" }
                        }
                    }
                }
            }
        }
    };

    let add_serving_form = html! {
        form method="post" action=(ServingNewHandler::url(food_id)) {
            (form_section("Add Custom Serving Size", html! {
                div."add-serving-row" {
                    (form_group(html! {
                        (label("serving_name", "Serving Name"))
                        (text_input("serving_name", "serving_name", "e.g., cup, slice, package"))
                    }))
                    (form_group(html! {
                        (label("serving_amount", &format!("Amount ({})", food.serving_unit.as_str())))
                        (number_input("serving_amount", "serving_amount", "0.1", "e.g., 250"))
                    }))
                    (submit_button("Add Serving"))
                }
            }))
        }
    };

    let content = html! {
        (panel(&food_title, html! {
            (button_bar(html! {
                (button_link("Edit Food", &FoodEditHandler::url(food_id)))
                (button_link("Back to Library", FoodListHandler::url()))
            }))
            (nutrition_table)
        }))
        (panel("Custom Serving Sizes", html! {
            (servings_list)
            (add_serving_form)
        }))
    };

    let html_page = page(&format!("{}", food_title), content);
    Ok((StatusCode::OK, Html(html_page.into_string())))
}
