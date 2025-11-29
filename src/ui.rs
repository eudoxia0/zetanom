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

use crate::routes::assets::CssHandler;
use crate::routes::assets::CssResetHandler;
use crate::routes::food_list::FoodListHandler;
use crate::routes::root::RootHandler;

/// Page template with sidebar navigation
pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href=(CssResetHandler::url());
                link rel="stylesheet" href=(CssHandler::url());
                title { (title) " | zetanom" }
            }
            body {
                .root {
                    .sidebar {
                        nav {
                            ul {
                                li {
                                    a href=(RootHandler::url()) {
                                        "Today"
                                    }
                                }
                                li {
                                    a href=(FoodListHandler::url()) {
                                        "Library"
                                    }
                                }
                            }
                        }
                    }
                    .content-pane {
                        .title-container {
                            h1 .title {
                                (title)
                            }
                        }
                        .page-content {
                            (body)
                        }
                    }
                }
            }
        }
    }
}

/// Form section with title
pub fn form_section(title: &str, content: Markup) -> Markup {
    html! {
        div."form-section" {
            div."form-section-title" {
                (title)
            }
            (content)
        }
    }
}

/// Form row (horizontal layout)
pub fn form_row(content: Markup) -> Markup {
    html! {
        div."form-row" {
            (content)
        }
    }
}

/// Form group (label + input container)
pub fn form_group(content: Markup) -> Markup {
    html! {
        div."form-group" {
            (content)
        }
    }
}

/// Form group with half width
pub fn form_group_half(content: Markup) -> Markup {
    html! {
        div."form-group"."half" {
            (content)
        }
    }
}

/// Label (plain)
pub fn label(for_id: &str, text: &str) -> Markup {
    html! {
        label for=(for_id) { (text) }
    }
}

/// Label with required indicator (*)
pub fn label_required(for_id: &str, text: &str) -> Markup {
    html! {
        label."label-required" for=(for_id) { (text) }
    }
}

/// Label with hint text
pub fn label_with_hint(for_id: &str, text: &str, hint: &str) -> Markup {
    html! {
        label for=(for_id) {
            (text)
            " "
            span."label-hint" { (hint) }
        }
    }
}

/// Text input
pub fn text_input(id: &str, name: &str, placeholder: &str) -> Markup {
    html! {
        input type="text" id=(id) name=(name) placeholder=(placeholder) autocomplete="off";
    }
}

/// Text input with value
pub fn text_input_value(id: &str, name: &str, value: &str, placeholder: &str) -> Markup {
    html! {
        input type="text" id=(id) name=(name) value=(value) placeholder=(placeholder);
    }
}

/// Number input
pub fn number_input(id: &str, name: &str, step: &str, placeholder: &str) -> Markup {
    html! {
        input type="number" id=(id) name=(name) step=(step) placeholder=(placeholder);
    }
}

/// Select dropdown
pub fn select(id: &str, name: &str, options: Vec<(String, String)>) -> Markup {
    html! {
        select id=(id) name=(name) {
            @for (value, label) in options {
                option value=(value) { (label) }
            }
        }
    }
}

/// Select dropdown with selected value
pub fn select_with_selected(
    id: &str,
    name: &str,
    options: Vec<(String, String)>,
    selected: &str,
) -> Markup {
    html! {
        select id=(id) name=(name) {
            @for (value, label) in options {
                @if value == selected {
                    option value=(value) selected { (label) }
                } @else {
                    option value=(value) { (label) }
                }
            }
        }
    }
}

/// Button as link
pub fn button_link(text: &str, href: &str) -> Markup {
    html! {
        a.button href=(href) { (text) }
    }
}

/// Nutrition table row (for form input)
pub fn nutrition_row(
    label_text: &str,
    input_id: &str,
    input_name: &str,
    unit: &str,
    indent: u8,
) -> Markup {
    let class = match indent {
        1 => "nutrition-row indent-1",
        2 => "nutrition-row indent-2",
        _ => "nutrition-row",
    };
    let label_class = if indent > 0 {
        "nutrition-label sub"
    } else {
        "nutrition-label"
    };
    html! {
        div.(class) {
            div.(label_class) {
                (label_text)
            }
            div."nutrition-input" {
                input type="number" id=(input_id) name=(input_name) step="0.1" placeholder="0.0";
                span."nutrition-unit" { (unit) }
            }
        }
    }
}

/// Nutrition table row with value (for editing)
pub fn nutrition_row_with_value(
    label_text: &str,
    input_id: &str,
    input_name: &str,
    unit: &str,
    value: &str,
    indent: u8,
) -> Markup {
    let class = match indent {
        1 => "nutrition-row indent-1",
        2 => "nutrition-row indent-2",
        _ => "nutrition-row",
    };
    let label_class = if indent > 0 {
        "nutrition-label sub"
    } else {
        "nutrition-label"
    };
    html! {
        div.(class) {
            div.(label_class) {
                (label_text)
            }
            div."nutrition-input" {
                input type="number" id=(input_id) name=(input_name) value=(value) step="0.1" placeholder="0.0";
                span."nutrition-unit" { (unit) }
            }
        }
    }
}

/// Nutrition table container
pub fn nutrition_table(rows: Markup) -> Markup {
    html! {
        div."nutrition-table" {
            (rows)
        }
    }
}

/// Data table component
pub struct TableColumn {
    pub header: String,
    pub numeric: bool,
}
