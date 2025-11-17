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
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::Redirect;
use axum::routing::get;
use axum::routing::post;
use chrono::NaiveDate;
use chrono::Utc;
use db::CreateEntryInput;
use db::FoodId;
use db::FoodListEntry;
use db::ServingId;
use error::AppError;
use error::Fallible;
use maud::Markup;
use maud::html;
use serde::Deserialize;

use crate::ui::label;
use crate::ui::number_input;
use crate::ui::page;
use crate::www::ServerState;

pub struct LogNewHandler {}

impl LogNewHandler {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        let app = router.route("/log/{date}/new", get(get_handler));
        app.route("/log/{date}/new", post(post_handler))
    }
}

async fn get_handler(
    Path(date): Path<String>,
    State(state): State<ServerState>,
) -> Fallible<(StatusCode, Html<String>)> {
    let date: NaiveDate = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| AppError::new(format!("Failed to parse date: '{date}'.")))?;

    let db = state.db.try_lock()?;
    let foods: Vec<FoodListEntry> = db.list_foods()?;

    // Generate JavaScript data for fuzzy search
    let foods_json = foods
        .iter()
        .map(|f| {
            format!(
                r#"{{"id":{},"name":"{}","brand":"{}"}}"#,
                f.food_id,
                f.name.replace('"', "\\\""),
                f.brand.replace('"', "\\\"")
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let body: Markup = html! {
        h1 {
            "Log Food for " (date)
        }
        p {
            (label("search", "Search Food"));
            input type="text" id="search" autocomplete="off" placeholder="Type to search...";
            div id="search-results" style="display: none; border: 1px solid #ccc; max-height: 200px; overflow-y: auto;" {}
        }
        form method="post" action={(format!("/log/{}/new", date))} id="log-form" {
            input type="hidden" id="food_id" name="food_id" required;
            br;
            div id="selected-food" style="display: none;" {
                p {
                    strong { "Selected: " }
                    span id="selected-food-name" {}
                }
            }
            br;
            (label("serving_id", "Serving Size (optional)"));
            select id="serving_id" name="serving_id" {
                option value="" { "Base unit (100g or 100ml)" }
            }
            br;
            (label("amount", "Amount"));
            (number_input("amount"));
            br;
            input type="submit" value="Log Food";
        }

        script {
            (maud::PreEscaped(format!(r#"
                const foods = [{}];
                const searchInput = document.getElementById('search');
                const searchResults = document.getElementById('search-results');
                const foodIdInput = document.getElementById('food_id');
                const selectedFoodDiv = document.getElementById('selected-food');
                const selectedFoodName = document.getElementById('selected-food-name');
                const servingSelect = document.getElementById('serving_id');

                function fuzzyMatch(pattern, str) {{
                    pattern = pattern.toLowerCase();
                    str = str.toLowerCase();
                    let patternIdx = 0;
                    let strIdx = 0;

                    while (patternIdx < pattern.length && strIdx < str.length) {{
                        if (pattern[patternIdx] === str[strIdx]) {{
                            patternIdx++;
                        }}
                        strIdx++;
                    }}

                    return patternIdx === pattern.length;
                }}

                searchInput.addEventListener('input', function() {{
                    const query = this.value.trim();

                    if (query === '') {{
                        searchResults.style.display = 'none';
                        searchResults.innerHTML = '';
                        return;
                    }}

                    const matches = foods.filter(food =>
                        fuzzyMatch(query, food.name) || fuzzyMatch(query, food.brand)
                    ).slice(0, 10);

                    if (matches.length === 0) {{
                        searchResults.innerHTML = '<div style="padding: 5px;">No matches found</div>';
                        searchResults.style.display = 'block';
                        return;
                    }}

                    searchResults.innerHTML = matches.map(food =>
                        `<div class="search-result-item" data-id="${{food.id}}" data-name="${{food.name}}" data-brand="${{food.brand}}" style="padding: 5px; cursor: pointer; border-bottom: 1px solid #eee;">
                            ${{food.name}} — ${{food.brand}}
                        </div>`
                    ).join('');
                    searchResults.style.display = 'block';

                    document.querySelectorAll('.search-result-item').forEach(item => {{
                        item.addEventListener('click', function() {{
                            const foodId = this.getAttribute('data-id');
                            const foodName = this.getAttribute('data-name');
                            const foodBrand = this.getAttribute('data-brand');

                            foodIdInput.value = foodId;
                            selectedFoodName.textContent = foodName + ' — ' + foodBrand;
                            selectedFoodDiv.style.display = 'block';
                            searchInput.value = '';
                            searchResults.style.display = 'none';

                            // Load serving sizes for this food
                            loadServingSizes(foodId);
                        }});
                    }});
                }});

                async function loadServingSizes(foodId) {{
                    // For now, just clear the serving sizes
                    // In a full implementation, you would fetch serving sizes from the server
                    servingSelect.innerHTML = '<option value="">Base unit (100g or 100ml)</option>';
                }}

                // Hide search results when clicking outside
                document.addEventListener('click', function(e) {{
                    if (e.target !== searchInput && e.target !== searchResults) {{
                        searchResults.style.display = 'none';
                    }}
                }});
            "#, foods_json)))
        }
    };

    let html: Markup = page("zetanom", body);
    Ok((StatusCode::OK, Html(html.into_string())))
}

#[derive(Deserialize)]
struct LogFoodForm {
    food_id: FoodId,
    serving_id: String,
    amount: f64,
}

async fn post_handler(
    Path(date): Path<String>,
    State(state): State<ServerState>,
    Form(form): Form<LogFoodForm>,
) -> Fallible<Redirect> {
    let date: NaiveDate = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| AppError::new(format!("Failed to parse date: '{date}'.")))?;

    let LogFoodForm {
        food_id,
        serving_id,
        amount,
    } = form;

    let serving_id: Option<ServingId> = match serving_id.as_ref() {
        "" => None,
        s => Some(s.parse()?),
    };

    let created_at = Utc::now();
    let input = CreateEntryInput {
        date,
        food_id,
        serving_id,
        amount,
        created_at,
    };

    let db = state.db.try_lock()?;
    db.create_entry(input)?;

    Ok(Redirect::to(&format!("/log/{}", date)))
}
