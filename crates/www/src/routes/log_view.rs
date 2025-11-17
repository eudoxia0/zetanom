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
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use chrono::NaiveDate;
use error::AppError;
use error::Fallible;
use maud::Markup;
use maud::html;

use crate::ui::page;
use crate::www::ServerState;

pub struct LogViewHandler {}

impl LogViewHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/log/{date}", get(handler))
    }

    pub fn url(date: NaiveDate) -> String {
        let date = date.format("%Y-%m-%d");
        format!("/log/{date}")
    }
}

async fn handler(Path(date): Path<String>) -> Fallible<(StatusCode, Html<String>)> {
    let date: NaiveDate = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| AppError::new(format!("Failed to parse date: '{date}'.")))?;
    // TODO: Show a table of food the user logged today.
    let table: Markup = todo!();
    let body: Markup = html! {
        p {
            (format!("Log: {date}"))
        }
        p {
            a href=(format!("/log/{date}/new")) {
                "Log Food"
            }
        }
        (table)
    };
    let html: Markup = page("zetanom", body);
    Ok((StatusCode::OK, Html(html.into_string())))
}
