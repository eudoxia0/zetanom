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
use axum::response::Redirect;
use axum::routing::get;
use chrono::Local;
use chrono::NaiveDate;

use crate::routes::log_view::LogViewHandler;
use crate::www::ServerState;

pub struct RootHandler {}

impl RootHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(handler))
    }

    pub fn url() -> &'static str {
        "/"
    }
}

async fn handler() -> Redirect {
    let today: NaiveDate = Local::now().naive_local().date();
    Redirect::to(&LogViewHandler::url(today))
}
