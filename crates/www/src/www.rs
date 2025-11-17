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

use std::sync::Arc;
use std::sync::Mutex;

use axum::Form;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::Redirect;
use axum::routing::IntoMakeService;
use axum::routing::get;
use axum::routing::post;
use chrono::NaiveDate;
use chrono::Utc;
use db::Db;
use db::FoodId;
use db::ServingId;
use db::ServingInput;
use error::AppError;
use error::Fallible;
use maud::Markup;
use maud::html;
use serde::Deserialize;
use tokio::net::TcpListener;

use crate::routes::assets::CssHandler;
use crate::routes::assets::FaviconHandler;
use crate::routes::food_list::FoodListHandler;
use crate::routes::food_new::FoodNewHandler;
use crate::routes::food_view::FoodViewHandler;
use crate::routes::root::RootHandler;
use crate::ui::page;

const PORT: u16 = 12001;

#[derive(Clone)]
pub struct ServerState {
    pub db: Arc<Mutex<Db>>,
}

pub async fn start_server() -> Fallible<()> {
    let db: Db = Db::new()?;
    let state: ServerState = ServerState {
        db: Arc::new(Mutex::new(db)),
    };
    let app: Router<ServerState> = Router::new();
    let app = RootHandler::route(app);
    let app = FaviconHandler::route(app);
    let app = FoodListHandler::route(app);
    let app = FoodViewHandler::route(app);
    let app = FoodNewHandler::route(app);
    let app = app.route("/library/{food_id}/servings", post(create_serving_handler));
    let app = app.route(
        "/library/{food_id}/servings/{serving_id}/delete",
        post(delete_serving_handler),
    );
    let app = app.route("/log/{date}", get(date_handler));
    let app = CssHandler::route(app);
    let app: IntoMakeService<Router> = app.with_state(state).into_make_service();
    let bind: String = format!("0.0.0.0:{PORT}");
    println!("Started server on {bind}.");
    let listener: TcpListener = TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
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

#[derive(Deserialize)]
struct CreateServingForm {
    serving_name: String,
    serving_amount: f64,
}

async fn create_serving_handler(
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
    Ok(Redirect::to(&format!("/library/{food_id}")))
}

async fn delete_serving_handler(
    State(state): State<ServerState>,
    Path((food_id, serving_id)): Path<(FoodId, ServingId)>,
) -> Fallible<Redirect> {
    let db = state.db.try_lock()?;
    db.delete_serving(serving_id)?;
    Ok(Redirect::to(&format!("/library/{food_id}")))
}
