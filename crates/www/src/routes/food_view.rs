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
use db::FoodEntry;
use db::FoodId;
use db::Serving;
use error::Fallible;
use maud::Markup;
use maud::html;

use crate::routes::food_list::FoodListHandler;
use crate::ui::label;
use crate::ui::number_input;
use crate::ui::page;
use crate::ui::text_input;
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
    let body: Markup = html! {
        h1 {
            (food.name)
        }
        h2 {
            (food.brand)
        }
        p {
            "Serving size: 100" (food.serving_unit.as_str())
        }
        table {
            tr {
                th { "Nutrient" }
                th { "Amount" }
            }
            tr {
                td { "Energy" }
                td { (food.energy) " kcal" }
            }
            tr {
                td { "Protein" }
                td { (food.protein) " g" }
            }
            tr {
                td { "Fat" }
                td { (food.fat) " g" }
            }
            tr {
                td { "— Saturated Fat" }
                td { (food.fat_saturated) " g" }
            }
            tr {
                td { "Carbohydrate" }
                td { (food.carbs) " g" }
            }
            tr {
                td { "— Sugars" }
                td { (food.carbs_sugars) " g" }
            }
            tr {
                td { "Fibre" }
                td { (food.fibre) " g" }
            }
            tr {
                td { "Sodium" }
                td { (food.sodium) " mg" }
            }
        }
        h2 {
            "Serving Sizes"
        }
        @if servings.is_empty() {
            p {
                "No custom serving sizes defined."
            }
        } @else {
            ul {
                @for serving in &servings {
                    li {
                        (serving.serving_name) ": " (serving.serving_amount) (food.serving_unit.as_str())
                        " "
                        form method="post" action={(format!("/library/{}/servings/{}/delete", food_id, serving.serving_id))} style="display: inline;" {
                            input type="submit" value="Delete";
                        }
                    }
                }
            }
        }
        h3 {
            "Add Serving Size"
        }
        form method="post" action={(format!("/library/{}/servings", food_id))} {
            (label("serving_name", "Name (e.g., cup, slice, package)"));
            (text_input("serving_name"));
            br;
            (label("serving_amount", &format!("Amount ({})", food.serving_unit.as_str())));
            (number_input("serving_amount"));
            br;
            input type="submit" value="Add Serving Size";
        }
        p {
            a href=(format!("/library/{food_id}/edit")) {
                "Edit"
            }
            a href=(FoodListHandler::url()) {
                "Back to Library"
            }
        }
    };
    let html: Markup = page("zetanom", body);
    Ok((StatusCode::OK, Html(html.into_string())))
}
