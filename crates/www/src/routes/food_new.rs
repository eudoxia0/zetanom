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
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::Redirect;
use axum::routing::get;
use axum::routing::post;
use chrono::Utc;
use db::CreateFoodInput;
use db::FoodId;
use error::Fallible;
use maud::html;
use serde::Deserialize;
use shared::basic_unit::BasicUnit;

use crate::routes::food_view::FoodViewHandler;
use crate::ui::*;
use crate::www::ServerState;

pub struct FoodNewHandler {}

impl FoodNewHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        let router = router.route(Self::url(), get(get_handler));
        router.route(Self::url(), post(post_handler))
    }

    pub fn url() -> &'static str {
        "/library/new"
    }
}

async fn get_handler() -> Fallible<(StatusCode, Html<String>)> {
    let nav = default_nav("food_new");

    let form_content = html! {
        form method="post" action=(FoodNewHandler::url()) {
            // Basic Information Section
            (form_section("Basic Information", html! {
                (form_row(html! {
                    (form_group(html! {
                        (label_required("food_name", "Food Name"))
                        (text_input("food_name", "food_name", "e.g., Rolled Oats"))
                    }))
                }))
                (form_row(html! {
                    (form_group_half(html! {
                        (label_with_hint("brand", "Brand", "(optional, leave blank for generic foods)"))
                        (text_input("brand", "brand", "e.g., Uncle Tobys"))
                    }))
                    (form_group_half(html! {
                        (label_required("serving_unit", "Base Unit"))
                        (select("serving_unit", "serving_unit", vec![
                            ("g".to_string(), "Grams (g)".to_string()),
                            ("ml".to_string(), "Milliliters (ml)".to_string()),
                        ]))
                    }))
                }))
            }))

            // Nutrition Information Section
            (form_section("Nutrition Information (per 100g or 100ml)", html! {
                (nutrition_table(html! {
                    (nutrition_row("Energy *", "energy", "energy", "kcal", 0))
                    (nutrition_row("Protein *", "protein", "protein", "g", 0))
                    (nutrition_row("Fat, Total *", "fat", "fat", "g", 0))
                    (nutrition_row("Saturated *", "fat_saturated", "fat_saturated", "g", 1))
                    (nutrition_row("Carbohydrate *", "carbs", "carbs", "g", 0))
                    (nutrition_row("Sugars *", "carbs_sugars", "carbs_sugars", "g", 1))
                    (nutrition_row("Dietary Fibre *", "fibre", "fibre", "g", 0))
                    (nutrition_row("Sodium *", "sodium", "sodium", "mg", 0))
                }))
            }))

            // Action Buttons
            (button_bar(html! {
                (submit_button_primary("Save Food"))
                (button("Cancel"))
            }))
        }
    };

    let help_content = html! {
        p {
            strong { "Where to find nutrition information:" }
            br;
            "Look at the nutrition information panel on the back of food packaging. In Australia, all values are shown per 100g or per 100ml."
        }
        p {
            strong { "Carbohydrate vs. Sugars:" }
            br;
            "\"Carbohydrate\" refers to available carbohydrate (excluding fiber). \"Sugars\" is a subset of carbohydrate and should be indented underneath it on labels."
        }
    };

    let content = html! {
        (panel("Add New Food", html! {
            (info_box(html! {
                strong { "Note:" }
                "All nutrition information should be entered per 100g or per 100ml as shown on the Australian nutrition label."
            }))
            (form_content)
        }))
        (panel("Help", help_content))
    };

    let html_page = page("Add New Food — zetanom", nav, content);
    Ok((StatusCode::OK, Html(html_page.into_string())))
}

#[derive(Deserialize)]
struct CreateFoodForm {
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
    Form(form): Form<CreateFoodForm>,
) -> Fallible<Redirect> {
    let CreateFoodForm {
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
    let created_at = Utc::now();
    let input = CreateFoodInput {
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
        created_at,
    };
    let db = state.db.try_lock()?;
    let food_id: FoodId = db.create_food(input)?;
    Ok(Redirect::to(&FoodViewHandler::url(food_id)))
}
