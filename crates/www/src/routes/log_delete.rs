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
use chrono::NaiveDate;
use db::EntryId;
use error::AppError;
use error::Fallible;
use shared::date::Date;

use crate::routes::log_view::LogViewHandler;
use crate::www::ServerState;

pub struct LogDeleteHandler {}

impl LogDeleteHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/log/{date}/entry/{entry_id}/delete", post(post_handler))
    }

    pub fn url(date: Date, entry_id: EntryId) -> String {
        format!("/log/{date}/entry/{entry_id}/delete")
    }
}

async fn post_handler(
    State(state): State<ServerState>,
    Path((date, entry_id)): Path<(String, EntryId)>,
) -> Fallible<Redirect> {
    let date = Date::try_from(date)?;
    let db = state.db.try_lock()?;
    db.delete_entry(entry_id)?;
    Ok(Redirect::to(&LogViewHandler::url(date)))
}
