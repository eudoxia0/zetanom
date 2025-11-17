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
use db::ServingUnit;
use error::Fallible;
use maud::Markup;
use maud::html;
use serde::Deserialize;

use crate::ui::label;
use crate::ui::page;
use crate::www::ServerState;

pub struct FoodEditHandler {}

impl FoodEditHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        let router = router.route("/library/{food_id}/edit", get(get_handler));
        router.route("/library/{food_id}/edit", post(post_handler))
    }
}

async fn get_handler(
    State(state): State<ServerState>,
    Path(food_id): Path<FoodId>,
) -> Fallible<(StatusCode, Html<String>)> {
    let db = state.db.try_lock()?;
    let food: FoodEntry = db.get_food(food_id)?;
    let body: Markup = html! {
        p {
            h1 {
                "Edit Food: " (food.name)
            }
            form method="post" action={(format!("/library/{}/edit", food_id))} {
                (label("name", "Name"));
                input type="text" name="name" id="name" value=(food.name);
                br;
                (label("brand", "Brand"));
                input type="text" name="brand" id="brand" value=(food.brand);
                br;
                (label("serving_unit", "Serving Unit"));
                select id="serving_unit" name="serving_unit" {
                    option value="g" selected[food.serving_unit.as_str() == "g"] { "g" }
                    option value="ml" selected[food.serving_unit.as_str() == "ml"] { "ml" }
                }
                br;
                (label("energy", "Energy (kcal)"));
                input type="number" name="energy" id="energy" value=(food.energy) step="0.01";
                br;
                (label("protein", "Protein (g)"));
                input type="number" name="protein" id="protein" value=(food.protein) step="0.01";
                br;
                (label("fat", "Fat (g)"));
                input type="number" name="fat" id="fat" value=(food.fat) step="0.01";
                br;
                (label("fat_saturated", "Fat — Saturated (g)"));
                input type="number" name="fat_saturated" id="fat_saturated" value=(food.fat_saturated) step="0.01";
                br;
                (label("carbs", "Carbohydrate (g)"));
                input type="number" name="carbs" id="carbs" value=(food.carbs) step="0.01";
                br;
                (label("carbs_sugars", "Carbohydrate — Sugars (g)"));
                input type="number" name="carbs_sugars" id="carbs_sugars" value=(food.carbs_sugars) step="0.01";
                br;
                (label("fibre", "Fibre (g)"));
                input type="number" name="fibre" id="fibre" value=(food.fibre) step="0.01";
                br;
                (label("sodium", "Sodium (mg)"));
                input type="number" name="sodium" id="sodium" value=(food.sodium) step="0.01";
                br;
                input type="submit" value="Save";
                " "
                a href={(format!("/library/{}", food_id))} {
                    "Cancel"
                }
            }
        }
    };
    let html: Markup = page("zetanom", body);
    Ok((StatusCode::OK, Html(html.into_string())))
}

#[derive(Deserialize)]
struct EditFoodForm {
    name: String,
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
        name,
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
        name,
        brand,
        serving_unit: ServingUnit::try_from(serving_unit.as_ref())?,
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
    Ok(Redirect::to(&format!("/library/{food_id}")))
}
