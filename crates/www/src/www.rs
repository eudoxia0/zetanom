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
use axum::http::HeaderName;
use axum::http::StatusCode;
use axum::http::header::CACHE_CONTROL;
use axum::http::header::CONTENT_TYPE;
use axum::response::Html;
use axum::response::Redirect;
use axum::routing::get;
use axum::routing::post;
use chrono::Local;
use chrono::NaiveDate;
use error::AppError;
use error::Fallible;
use maud::Markup;
use maud::html;
use serde::Deserialize;
use tokio::net::TcpListener;

use crate::ui::label;
use crate::ui::number_input;
use crate::ui::page;
use crate::ui::text_input;

const PORT: u16 = 12001;

pub async fn start_server() -> Fallible<()> {
    let app: Router<()> = make_app();
    let bind: String = format!("0.0.0.0:{PORT}");
    println!("Started server on {bind}.");
    let listener: TcpListener = TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[allow(clippy::let_and_return)]
fn make_app() -> Router<()> {
    let app = Router::new();
    let app = app.route("/", get(index_handler));
    let app = app.route("/favicon.ico", get(favicon_handler));
    let app = app.route("/library", get(library_handler));
    let app = app.route("/library/new", get(library_new_handler));
    let app = app.route("/library/new", post(library_new_post_handler));
    let app = app.route("/log/{date}", get(date_handler));
    let app = app.route("/static/style.css", get(css_handler));
    app
}

async fn index_handler() -> Redirect {
    let today: NaiveDate = Local::now().naive_local().date();
    let url: String = format!("/log/{}", today.format("%Y-%m-%d"));
    Redirect::to(&url)
}

async fn date_handler(Path(date): Path<String>) -> Fallible<(StatusCode, Html<String>)> {
    let date: NaiveDate = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| AppError::new(format!("Failed to parse date: '{date}'.")))?;
    let body: Markup = html! {
        p {
            (format!("Log: {date}"))
        }
    };
    let html: Markup = page("zetanom", body);
    Ok((StatusCode::OK, Html(html.into_string())))
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

async fn library_handler() -> Fallible<(StatusCode, Html<String>)> {
    let body: Markup = html! {
        p {
            h1 {
                "Library"
            }
        }
    };
    let html: Markup = page("zetanom", body);
    Ok((StatusCode::OK, Html(html.into_string())))
}

async fn library_new_handler() -> Fallible<(StatusCode, Html<String>)> {
    let body: Markup = html! {
        p {
            h1 {
                "Library: New Food"
            }
            form method="post" action="/library/new" {
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
    sodium: f64,
}

async fn library_new_post_handler(Form(form): Form<CreateFoodForm>) -> Redirect {
    todo!()
}
