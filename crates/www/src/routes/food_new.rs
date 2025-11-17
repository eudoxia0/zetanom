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
use db::ServingUnit;
use error::Fallible;
use maud::Markup;
use maud::html;
use serde::Deserialize;

use crate::ui::label;
use crate::ui::number_input;
use crate::ui::page;
use crate::ui::text_input;
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
    let body: Markup = html! {
        p {
            h1 {
                "Library: New Food"
            }
            form method="post" action=(FoodNewHandler::url()) {
                (label("name", "Name"));
                (text_input("name"));
                br;
                (label("brand", "Brand"));
                (text_input("brand"));
                br;
                (label("serving_unit", "Serving Unit"));
                select id="serving_unit" name="serving_unit" {
                    option value="g" { "g" }
                    option value="ml" { "ml" }
                }
                br;
                (label("energy", "Energy (kcal)"));
                (number_input("energy"));
                br;
                (label("protein", "Protein (g)"));
                (number_input("protein"));
                br;
                (label("fat", "Fat (g)"));
                (number_input("fat"));
                br;
                (label("fat_saturated", "Fat — Saturated (g)"));
                (number_input("fat_saturated"));
                br;
                (label("carbs", "Carbohydrate (g)"));
                (number_input("carbs"));
                br;
                (label("carbs_sugars", "Carbohydrate — Sugars (g)"));
                (number_input("carbs_sugars"));
                br;
                (label("fibre", "Fibre (g)"));
                (number_input("fibre"));
                br;
                (label("sodium", "Sodium (mg)"));
                (number_input("sodium"));
                br;
                input type="submit" value="Save";
            }
        }
    };
    let html: Markup = page("zetanom", body);
    Ok((StatusCode::OK, Html(html.into_string())))
}

#[derive(Deserialize)]
struct CreateFoodForm {
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
    Form(form): Form<CreateFoodForm>,
) -> Fallible<Redirect> {
    let CreateFoodForm {
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
    let created_at = Utc::now();
    let input = CreateFoodInput {
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
        created_at,
    };
    let db = state.db.try_lock()?;
    let food_id: FoodId = db.create_food(input)?;
    Ok(Redirect::to(&format!("/library/{food_id}")))
}
