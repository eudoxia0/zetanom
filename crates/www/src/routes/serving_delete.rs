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
use axum::response::Redirect;
use axum::routing::post;
use db::FoodId;
use db::ServingId;
use error::Fallible;

use crate::www::ServerState;

pub struct ServingDeleteHandler {}

impl ServingDeleteHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(
            "/library/{food_id}/servings/{serving_id}/delete",
            post(handler),
        )
    }
}

async fn handler(
    State(state): State<ServerState>,
    Path((food_id, serving_id)): Path<(FoodId, ServingId)>,
) -> Fallible<Redirect> {
    let db = state.db.try_lock()?;
    db.delete_serving(serving_id)?;
    Ok(Redirect::to(&format!("/library/{food_id}")))
}
