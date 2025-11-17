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
use axum::routing::get;

use crate::www::ServerState;

pub struct CssResetHandler {}

pub struct CssHandler {}

pub struct FaviconHandler {}

impl CssResetHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(css_reset_handler))
    }

    pub fn url() -> &'static str {
        "/static/reset.css"
    }
}

impl CssHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(css_handler))
    }

    pub fn url() -> &'static str {
        "/static/style.css"
    }
}

impl FaviconHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(favicon_handler))
    }

    pub fn url() -> &'static str {
        "/favicon.ico"
    }
}

async fn css_reset_handler() -> (StatusCode, [(HeaderName, &'static str); 2], &'static [u8]) {
    let bytes = include_bytes!("reset.css");
    (
        StatusCode::OK,
        [(CONTENT_TYPE, "text/css"), (CACHE_CONTROL, "no-cache")],
        bytes,
    )
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
