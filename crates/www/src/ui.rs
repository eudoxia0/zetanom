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

use maud::DOCTYPE;
use maud::Markup;
use maud::html;

/// Page template.
pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href="/static/style.css";
                title { (title) }
            }
            body {
                div.dt-container {
                    (sidebar())
                    div.dt-main-content {
                        (body)
                    }
                }
            }
        }
    }
}

/// Sidebar navigation component.
fn sidebar() -> Markup {
    html! {
        div.dt-sidebar {
            div.dt-app-title { "DIET TRACKER" }
            (nav_section("Views", &[
                ("Today", true),
                ("Week View", false),
                ("Calendar", false),
            ]))
            (nav_section("Library", &[
                ("All Foods", false),
                ("Add New Food", false),
                ("Recent Foods", false),
            ]))
            (nav_section("Reports", &[
                ("Weekly Summary", false),
                ("Export Data", false),
            ]))
        }
    }
}

/// Navigation section with title and items.
fn nav_section(title: &str, items: &[(&str, bool)]) -> Markup {
    html! {
        div.dt-nav-section {
            div.dt-nav-section-title { (title) }
            @for (item_text, is_active) in items {
                (nav_item(item_text, *is_active))
            }
        }
    }
}

/// Navigation item.
fn nav_item(text: &str, is_active: bool) -> Markup {
    let class = if is_active {
        "dt-nav-item dt-nav-item-active"
    } else {
        "dt-nav-item"
    };
    html! {
        div class=(class) { (text) }
    }
}

/// Main content for today's view.
pub fn today_view() -> Markup {
    html! {
        (daily_log_panel())
        (daily_totals_panel())
    }
}

/// Daily log panel with date navigation and food table.
fn daily_log_panel() -> Markup {
    html! {
        div.dt-panel {
            div.dt-panel-header { "Daily Log — Saturday, 08 November 2025" }
            div.dt-panel-content {
                div.dt-button-bar {
                    button.dt-button { "← Previous Day" }
                    button.dt-button { "Today" }
                    button.dt-button { "Next Day →" }
                    span.dt-spacer {}
                    input.dt-input-text.dt-search-input type="text" placeholder="Search food to add...";
                    button.dt-button { "Add" }
                }
                (food_table())
            }
        }
    }
}

/// Food log table with sample data.
fn food_table() -> Markup {
    html! {
        table.dt-food-table {
            thead {
                tr {
                    th { "Time" }
                    th { "Food" }
                    th { "Brand" }
                    th { "Amount" }
                    th.dt-numeric { "Energy (kcal)" }
                    th.dt-numeric { "Protein (g)" }
                    th.dt-numeric { "Fat (g)" }
                    th.dt-numeric { "Sat Fat (g)" }
                    th.dt-numeric { "Carbs (g)" }
                    th.dt-numeric { "Fiber (g)" }
                    th.dt-numeric { "Sodium (mg)" }
                    th {}
                }
            }
            tbody {
                (food_row("08:30", "Rolled Oats", "—", "50g", "185", "6.5", "3.5", "0.6", "28.0", "5.0", "2"))
                (food_row("08:35", "Full Cream Milk", "Dairy Farmers", "200ml", "132", "6.8", "7.2", "4.8", "9.8", "0.0", "90"))
                (food_row("13:15", "Chicken Breast", "—", "180g", "297", "62.3", "6.5", "1.4", "0.0", "0.0", "144"))
                (food_row("13:20", "White Rice", "—", "150g", "195", "3.5", "0.5", "0.1", "42.5", "0.6", "3"))
                (food_row("19:45", "Tikka Masala", "Trader Joe's", "1 package (340g)", "408", "17.7", "20.4", "10.2", "37.4", "3.4", "884"))
            }
        }
    }
}

/// Food table row.
fn food_row(
    time: &str,
    food: &str,
    brand: &str,
    amount: &str,
    energy: &str,
    protein: &str,
    fat: &str,
    sat_fat: &str,
    carbs: &str,
    fiber: &str,
    sodium: &str,
) -> Markup {
    html! {
        tr {
            td { (time) }
            td { (food) }
            td { (brand) }
            td { (amount) }
            td.dt-numeric { (energy) }
            td.dt-numeric { (protein) }
            td.dt-numeric { (fat) }
            td.dt-numeric { (sat_fat) }
            td.dt-numeric { (carbs) }
            td.dt-numeric { (fiber) }
            td.dt-numeric { (sodium) }
            td {
                button.dt-button { "Edit" }
                " "
                button.dt-button { "Delete" }
            }
        }
    }
}

/// Daily totals summary panel.
fn daily_totals_panel() -> Markup {
    html! {
        div.dt-summary-box {
            div.dt-panel-header { "Daily Totals" }
            div.dt-summary-content {
                table.dt-summary-table {
                    (summary_row("Energy", "1,217 kcal", Some("Target: 2,000 kcal"), false))
                    (summary_row("Protein", "96.8 g", None, false))
                    (summary_row("Fat", "38.1 g", None, false))
                    (summary_row("Saturated Fat", "17.1 g", Some("Limit: 15g (EXCEEDED)"), true))
                    (summary_row("Carbohydrate", "117.7 g", None, false))
                    (summary_row("Fiber", "9.0 g", None, false))
                    (summary_row("Sodium", "1,123 mg", Some("Limit: 2,300 mg"), false))
                }
            }
        }
    }
}

/// Summary table row.
fn summary_row(label: &str, value: &str, target_info: Option<&str>, is_over_limit: bool) -> Markup {
    let value_class = if is_over_limit {
        "dt-numeric dt-over-limit"
    } else {
        "dt-numeric"
    };

    let target_class = if is_over_limit {
        "dt-target-info dt-over-limit"
    } else {
        "dt-target-info"
    };

    html! {
        tr {
            td.dt-summary-table-label { (label) }
            td class=(value_class) { (value) }
            td class=(target_class) {
                @if let Some(info) = target_info {
                    (info)
                }
            }
        }
    }
}
