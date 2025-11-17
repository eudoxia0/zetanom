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

use axum::Router;
use axum::routing::IntoMakeService;
use db::Db;
use error::Fallible;
use tokio::net::TcpListener;

use crate::config::Config;
use crate::routes::assets::CssHandler;
use crate::routes::assets::CssResetHandler;
use crate::routes::food_edit::FoodEditHandler;
use crate::routes::food_list::FoodListHandler;
use crate::routes::food_new::FoodNewHandler;
use crate::routes::food_view::FoodViewHandler;
use crate::routes::log_delete::LogDeleteHandler;
use crate::routes::log_new::LogNewHandler;
use crate::routes::log_view::LogViewHandler;
use crate::routes::root::RootHandler;
use crate::routes::serving_delete::ServingDeleteHandler;
use crate::routes::serving_new::ServingNewHandler;

#[derive(Clone)]
pub struct ServerState {
    pub db: Arc<Mutex<Db>>,
}

pub async fn start_server() -> Fallible<()> {
    let config: Config = Config::load()?;
    println!("Database: {}", config.db_path.display());
    let port: u16 = config.port;
    let db: Db = Db::new(&config.db_path)?;
    let state: ServerState = ServerState {
        db: Arc::new(Mutex::new(db)),
    };
    let app: Router<ServerState> = Router::new();

    let app = CssHandler::route(app);
    let app = CssResetHandler::route(app);
    let app = FaviconHandler::route(app);
    let app = FoodEditHandler::route(app);
    let app = FoodListHandler::route(app);
    let app = FoodNewHandler::route(app);
    let app = FoodViewHandler::route(app);
    let app = LogDeleteHandler::route(app);
    let app = LogNewHandler::route(app);
    let app = LogViewHandler::route(app);
    let app = RootHandler::route(app);
    let app = ServingDeleteHandler::route(app);
    let app = ServingNewHandler::route(app);

    let app: IntoMakeService<Router> = app.with_state(state).into_make_service();
    let bind: String = format!("0.0.0.0:{port}");
    println!("Started server on {bind}.");
    let listener: TcpListener = TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
