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

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::sync::TryLockError;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;

#[derive(Debug)]
pub struct AppError {
    message: String,
}

impl AppError {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

pub type Fallible<T> = Result<T, AppError>;

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(value: rusqlite::Error) -> Self {
        AppError {
            message: format!("rusqlite: {value}"),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError {
            message: format!("I/O error: {value:#?}"),
        }
    }
}

impl<T> From<TryLockError<T>> for AppError {
    fn from(_: TryLockError<T>) -> Self {
        AppError {
            message: "Failed to acquire lock on the database.".to_string(),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(_: ParseIntError) -> Self {
        AppError {
            message: format!("failed to parse integer."),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let msg = self.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
    }
}
