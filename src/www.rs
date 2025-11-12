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
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use maud::DOCTYPE;
use maud::Markup;
use maud::html;

use crate::error::Fallible;

const PORT: u16 = 12001;

pub async fn start_server() -> Fallible<()> {
    let app = make_app();
    let bind = format!("0.0.0.0:{PORT}");
    println!("Started server on {bind}.");
    let listener = tokio::net::TcpListener::bind(bind).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn make_app() -> Router<()> {
    let app = Router::new();
    let app = app.route("/", get(index_handler));
    app
}

async fn index_handler() -> (StatusCode, Html<String>) {
    let body = html! {
    p {
        "Hello, world!"
    }
    };
    let html = page("zetanom", body);
    (StatusCode::OK, Html(html.into_string()))
}

pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
            }
            body {
                (body)
            }
        }
    }
}
