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

use std::fmt::Display;
use std::fmt::Formatter;

use chrono::Local;
use chrono::NaiveDate;
use error::AppError;
use rusqlite::ToSql;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlError;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;

#[derive(Clone, Copy)]
pub struct Date(NaiveDate);

impl Date {
    pub fn new(naive_date: NaiveDate) -> Self {
        Self(naive_date)
    }

    pub fn today() -> Self {
        Self(Local::now().naive_local().date())
    }

    pub fn into_inner(self) -> NaiveDate {
        self.0
    }

    pub fn humanize(&self) -> String {
        self.0.format("%A, %d %B %Y").to_string()
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl TryFrom<String> for Date {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let date = NaiveDate::parse_from_str(&value, "%Y-%m-%d")
            .map_err(|_| AppError::new(format!("invalid date: {}", value)))?;
        Ok(Date(date))
    }
}

impl ToSql for Date {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let str = self.to_string();
        Ok(ToSqlOutput::from(str))
    }
}

impl FromSql for Date {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let string: String = FromSql::column_result(value)?;
        Date::try_from(string).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}
