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
use db::EditFoodInput;
use db::FoodEntry;
use db::FoodId;
use db::BasicUnit;
use error::Fallible;
use maud::html;
use serde::Deserialize;

use crate::routes::food_view::FoodViewHandler;
use crate::ui::*;
use crate::www::ServerState;

pub struct FoodEditHandler {}

impl FoodEditHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        let router = router.route("/library/{food_id}/edit", get(get_handler));
        router.route("/library/{food_id}/edit", post(post_handler))
    }

    pub fn url(food_id: FoodId) -> String {
        format!("/library/{food_id}/edit")
    }
}

async fn get_handler(
    State(state): State<ServerState>,
    Path(food_id): Path<FoodId>,
) -> Fallible<(StatusCode, Html<String>)> {
    let nav = default_nav("food_list");

    let db = state.db.try_lock()?;
    let food: FoodEntry = db.get_food(food_id)?;

    let form_content = html! {
        form method="post" action=(FoodEditHandler::url(food_id)) {
            // Basic Information Section
            (form_section("Basic Information", html! {
                (form_row(html! {
                    (form_group(html! {
                        (label_required("food_name", "Food Name"))
                        (text_input_value("food_name", "food_name", &food.name, "e.g., Rolled Oats"))
                    }))
                }))
                (form_row(html! {
                    (form_group_half(html! {
                        (label_with_hint("brand", "Brand", "(optional, leave blank for generic foods)"))
                        (text_input_value("brand", "brand", &food.brand, "e.g., Uncle Tobys"))
                    }))
                    (form_group_half(html! {
                        (label_required("serving_unit", "Base Unit"))
                        (select_with_selected("serving_unit", "serving_unit", vec![
                            ("g".to_string(), "Grams (g)".to_string()),
                            ("ml".to_string(), "Milliliters (ml)".to_string()),
                        ], food.serving_unit.as_str()))
                    }))
                }))
            }))

            // Nutrition Information Section
            (form_section("Nutrition Information (per 100g or 100ml)", html! {
                (nutrition_table(html! {
                    (nutrition_row_with_value("Energy *", "energy", "energy", "kcal", &format!("{:.1}", food.energy), 0))
                    (nutrition_row_with_value("Protein *", "protein", "protein", "g", &format!("{:.1}", food.protein), 0))
                    (nutrition_row_with_value("Fat, Total *", "fat", "fat", "g", &format!("{:.1}", food.fat), 0))
                    (nutrition_row_with_value("Saturated *", "fat_saturated", "fat_saturated", "g", &format!("{:.1}", food.fat_saturated), 1))
                    (nutrition_row_with_value("Carbohydrate *", "carbs", "carbs", "g", &format!("{:.1}", food.carbs), 0))
                    (nutrition_row_with_value("Sugars *", "carbs_sugars", "carbs_sugars", "g", &format!("{:.1}", food.carbs_sugars), 1))
                    (nutrition_row_with_value("Dietary Fibre *", "fibre", "fibre", "g", &format!("{:.1}", food.fibre), 0))
                    (nutrition_row_with_value("Sodium *", "sodium", "sodium", "mg", &format!("{:.0}", food.sodium), 0))
                }))
            }))

            // Action Buttons
            (button_bar(html! {
                (submit_button_primary("Save Changes"))
                (button_link("Cancel", &FoodViewHandler::url(food_id)))
            }))
        }
    };

    let content = html! {
        (panel(&format!("Edit Food: {}", food.name), form_content))
    };

    let html_page = page(&format!("Edit {} — zetanom", food.name), nav, content);
    Ok((StatusCode::OK, Html(html_page.into_string())))
}

#[derive(Deserialize)]
struct EditFoodForm {
    food_name: String,
    brand: String,
    serving_unit: String,
    energy: f64,
    protein: f64,
    fat: f64,
    fat_saturated: f64,
    carbs: f64,
    carbs_sugars: f64,
    fibre: f64,
    sodium: f64,
}

async fn post_handler(
    State(state): State<ServerState>,
    Path(food_id): Path<FoodId>,
    Form(form): Form<EditFoodForm>,
) -> Fallible<Redirect> {
    let EditFoodForm {
        food_name,
        brand,
        serving_unit,
        energy,
        protein,
        fat,
        fat_saturated,
        carbs,
        carbs_sugars,
        fibre,
        sodium,
    } = form;
    let input = EditFoodInput {
        food_id,
        name: food_name,
        brand,
        serving_unit: BasicUnit::try_from(serving_unit.as_ref())?,
        energy,
        protein,
        fat,
        fat_saturated,
        carbs,
        carbs_sugars,
        fibre,
        sodium,
    };
    let db = state.db.try_lock()?;
    db.edit_food(input)?;
    Ok(Redirect::to(&FoodViewHandler::url(food_id)))
}
