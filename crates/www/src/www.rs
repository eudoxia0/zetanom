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
use axum::http::HeaderName;
use axum::http::StatusCode;
use axum::http::header::CACHE_CONTROL;
use axum::http::header::CONTENT_TYPE;
use axum::response::Html;
use axum::routing::get;
use error::Fallible;
use maud::Markup;
use maud::html;
use tokio::net::TcpListener;

use crate::ui::page;

const PORT: u16 = 12001;

pub async fn start_server() -> Fallible<()> {
    let app: Router<()> = make_app();
    let bind: String = format!("0.0.0.0:{PORT}");
    println!("Started server on {bind}.");
    let listener: TcpListener = TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn make_app() -> Router<()> {
    let app = Router::new();
    let app = app.route("/", get(index_handler));
    let app = app.route("/static/style.css", get(css_handler));
    app.route("/favicon.ico", get(favicon_handler))
}

async fn index_handler() -> (StatusCode, Html<String>) {
    let body: Markup = html! {
        p {
            "Hello, world!"
        }
    };
    let html: Markup = page("zetanom", body);
    (StatusCode::OK, Html(html.into_string()))
}

async fn css_handler() -> (StatusCode, [(HeaderName, &'static str); 2], &'static [u8]) {
    let bytes = include_bytes!("style.css");
    (
        StatusCode::OK,
        [(CONTENT_TYPE, "text/css"), (CACHE_CONTROL, "no-cache")],
        bytes,
    )
}

async fn favicon_handler() -> (StatusCode, [(HeaderName, &'static str); 2], &'static [u8]) {
    let bytes = include_bytes!("favicon.png");
    (
        StatusCode::OK,
        [(CONTENT_TYPE, "image/png"), (CACHE_CONTROL, "no-cache")],
        bytes,
    )
}
