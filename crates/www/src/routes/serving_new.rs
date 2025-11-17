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
use axum::response::Redirect;
use axum::routing::post;
use chrono::Utc;
use db::FoodId;
use db::ServingInput;
use error::Fallible;
use serde::Deserialize;

use crate::www::ServerState;

pub struct ServingNewHandler {}

impl ServingNewHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/library/{food_id}/servings", post(handler))
    }
}

#[derive(Deserialize)]
struct CreateServingForm {
    serving_name: String,
    serving_amount: f64,
}

async fn handler(
    State(state): State<ServerState>,
    Path(food_id): Path<FoodId>,
    Form(form): Form<CreateServingForm>,
) -> Fallible<Redirect> {
    let CreateServingForm {
        serving_name,
        serving_amount,
    } = form;
    let created_at = Utc::now();
    let input = ServingInput {
        food_id,
        serving_name,
        serving_amount,
        created_at,
    };
    let db = state.db.try_lock()?;
    db.create_serving(input)?;
    Ok(Redirect::to(&FoodViewHandler::url(food_id)))
}
