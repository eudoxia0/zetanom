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

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use error::AppError;
use error::Fallible;
use rusqlite::Connection;
use rusqlite::config::DbConfig;
use rusqlite::params;

pub struct Db {
    conn: Connection,
}

pub type FoodId = i64;

#[derive(Clone, Copy)]
pub enum ServingUnit {
    Grams,
    Milliliters,
}

impl ServingUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Grams => "g",
            Self::Milliliters => "ml",
        }
    }
}

impl TryFrom<&str> for ServingUnit {
    type Error = AppError;

    fn try_from(value: &str) -> Fallible<Self> {
        match value {
            "g" => Ok(Self::Grams),
            "ml" => Ok(Self::Milliliters),
            _ => Err(AppError::new("Invalid value for serving unit.")),
        }
    }
}

/// The name of a food.
pub type FoodName = String;

/// The name of a brand.
pub type BrandName = String;

/// An amount of energy in kcal.
pub type Energy = f64;

/// An amount of protein in grams.
pub type Protein = f64;

/// An amount of fat in grams.
pub type Fat = f64;

/// An amount of saturated fat in grams.
pub type SaturatedFat = f64;

/// An amount of carbohydrate in grams.
pub type Carbs = f64;

/// An amount of sugar in grams.
pub type Sugars = f64;

/// An amount of fibre in grams.
pub type Fibre = f64;

/// An amount of sodium in milligrams.
pub type Sodium = f64;

/// Data needed to create a new food.
pub struct CreateFoodInput {
    pub name: FoodName,
    pub brand: BrandName,
    pub serving_unit: ServingUnit,
    pub energy: Energy,
    pub protein: Protein,
    pub fat: Fat,
    pub fat_saturated: SaturatedFat,
    pub carbs: Carbs,
    pub carbs_sugars: Sugars,
    pub fibre: Fibre,
    pub sodium: Sodium,
    pub created_at: DateTime<Utc>,
}

/// Summary information for a food entry.
pub struct FoodListEntry {
    pub food_id: FoodId,
    pub name: FoodName,
    pub brand: BrandName,
}

/// A food entry.
pub struct FoodEntry {
    pub food_id: FoodId,
    pub name: FoodName,
    pub brand: BrandName,
    pub serving_unit: ServingUnit,
    pub energy: Energy,
    pub protein: Protein,
    pub fat: Fat,
    pub fat_saturated: SaturatedFat,
    pub carbs: Carbs,
    pub carbs_sugars: Sugars,
    pub fibre: Fibre,
    pub sodium: Sodium,
    pub created_at: DateTime<Utc>,
}

/// Data needed to edit an existing food.
pub struct EditFoodInput {
    pub food_id: FoodId,
    pub name: FoodName,
    pub brand: BrandName,
    pub serving_unit: ServingUnit,
    pub energy: Energy,
    pub protein: Protein,
    pub fat: Fat,
    pub fat_saturated: SaturatedFat,
    pub carbs: Carbs,
    pub carbs_sugars: Sugars,
    pub fibre: Fibre,
    pub sodium: Sodium,
}

pub type ServingId = i64;
pub type ServingName = String;

pub struct ServingInput {
    pub food_id: FoodId,
    pub serving_name: ServingName,
    pub serving_amount: f64,
    pub created_at: DateTime<Utc>,
}

pub struct Serving {
    pub serving_id: ServingId,
    pub food_id: FoodId,
    pub serving_name: ServingName,
    pub serving_amount: f64,
    pub created_at: DateTime<Utc>,
}

pub type EntryId = i64;

pub struct CreateEntryInput {
    pub date: NaiveDate,
    pub food_id: FoodId,
    pub serving_id: Option<ServingId>,
    pub amount: f64,
    pub created_at: DateTime<Utc>,
}

pub struct Entry {
    pub entry_id: EntryId,
    pub date: NaiveDate,
    pub food_id: FoodId,
    pub serving_id: Option<ServingId>,
    pub amount: f64,
    pub created_at: DateTime<Utc>,
}

impl Db {
    pub fn new() -> Fallible<Self> {
        let mut conn = Connection::open_in_memory()?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;
        let tx = conn.transaction()?;
        tx.execute_batch(include_str!("schema.sql"))?;
        tx.commit()?;
        Ok(Self { conn })
    }

    /// Return the total number of foods in the library.
    pub fn count_foods(&self) -> Fallible<usize> {
        let sql = "select count(*) from foods;";
        let count: i64 = self.conn.query_row(sql, [], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// Create a new food.
    pub fn create_food(&self, input: CreateFoodInput) -> Fallible<FoodId> {
        let sql = "
            insert into foods
                (name, brand, serving_unit, energy, protein, fat, fat_saturated, carbs, carbs_sugars, fibre, sodium, created_at)
            values
                (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            returning food_id;
        ";
        let food_id: i64 = self.conn.query_row(
            sql,
            params![
                input.name,
                input.brand,
                input.serving_unit.as_str(),
                input.energy,
                input.protein,
                input.fat,
                input.fat_saturated,
                input.carbs,
                input.carbs_sugars,
                input.fibre,
                input.sodium,
                input.created_at,
            ],
            |row| row.get(0),
        )?;
        Ok(food_id)
    }

    /// Return summary information for all foods in the database.
    pub fn list_foods(&self) -> Fallible<Vec<FoodListEntry>> {
        let sql = "select food_id, name, brand from foods order by name;";
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            Ok(FoodListEntry {
                food_id: row.get(0)?,
                name: row.get(1)?,
                brand: row.get(2)?,
            })
        })?;
        let mut foods = Vec::new();
        for food in rows {
            foods.push(food?);
        }
        Ok(foods)
    }

    /// Return data for a food.
    pub fn get_food(&self, food_id: FoodId) -> Fallible<FoodEntry> {
        let sql = "
            select
                food_id, name, brand, serving_unit, energy, protein, fat, fat_saturated, carbs, carbs_sugars, fibre, sodium, created_at
            from
                foods
            where
                food_id = ?1;
        ";
        let entry = self.conn.query_row(sql, params![food_id], |row| {
            let serving_unit_str: String = row.get(3)?;
            let serving_unit = ServingUnit::try_from(serving_unit_str.as_str())
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            Ok(FoodEntry {
                food_id: row.get(0)?,
                name: row.get(1)?,
                brand: row.get(2)?,
                serving_unit,
                energy: row.get(4)?,
                protein: row.get(5)?,
                fat: row.get(6)?,
                fat_saturated: row.get(7)?,
                carbs: row.get(8)?,
                carbs_sugars: row.get(9)?,
                fibre: row.get(10)?,
                sodium: row.get(11)?,
                created_at: row.get(12)?,
            })
        })?;
        Ok(entry)
    }

    pub fn edit_food(&self, input: EditFoodInput) -> Fallible<()> {
        let sql = "
            update foods
            set
                name = ?1,
                brand = ?2,
                serving_unit = ?3,
                energy = ?4,
                protein = ?5,
                fat = ?6,
                fat_saturated = ?7,
                carbs = ?8,
                carbs_sugars = ?9,
                fibre = ?10,
                sodium = ?11
            where
                food_id = ?12;
        ";
        self.conn.execute(
            sql,
            params![
                input.name,
                input.brand,
                input.serving_unit.as_str(),
                input.energy,
                input.protein,
                input.fat,
                input.fat_saturated,
                input.carbs,
                input.carbs_sugars,
                input.fibre,
                input.sodium,
                input.food_id,
            ],
        )?;
        Ok(())
    }

    pub fn create_serving(&self, input: ServingInput) -> Fallible<ServingId> {
        let sql = "
            insert into serving_sizes
                (food_id, serving_name, serving_amount, created_at)
            values
                (?1, ?2, ?3, ?4)
            returning serving_id;
        ";
        let serving_id: i64 = self.conn.query_row(
            sql,
            params![
                input.food_id,
                input.serving_name,
                input.serving_amount,
                input.created_at,
            ],
            |row| row.get(0),
        )?;
        Ok(serving_id)
    }

    pub fn delete_serving(&self, serving_id: ServingId) -> Fallible<()> {
        let sql = "delete from serving_sizes where serving_id = ?1;";
        self.conn.execute(sql, params![serving_id])?;
        Ok(())
    }

    pub fn list_servings(&self, food_id: FoodId) -> Fallible<Vec<Serving>> {
        let sql = "
            select
                serving_id, food_id, serving_name, serving_amount, created_at
            from
                serving_sizes
            where
                food_id = ?1
            order by
                serving_name;
        ";
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(params![food_id], |row| {
            Ok(Serving {
                serving_id: row.get(0)?,
                food_id: row.get(1)?,
                serving_name: row.get(2)?,
                serving_amount: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        let mut servings = Vec::new();
        for serving in rows {
            servings.push(serving?);
        }
        Ok(servings)
    }

    pub fn create_entry(&self, input: CreateEntryInput) -> Fallible<EntryId> {
        let sql = "
            insert into entries
                (date, food_id, serving_id, amount, created_at)
            values
                (?1, ?2, ?3, ?4, ?5)
            returning entry_id;
        ";
        let entry_id: i64 = self.conn.query_row(
            sql,
            params![
                input.date.format("%Y-%m-%d").to_string(),
                input.food_id,
                input.serving_id,
                input.amount,
                input.created_at,
            ],
            |row| row.get(0),
        )?;
        Ok(entry_id)
    }

    pub fn delete_entry(&self, entry_id: EntryId) -> Fallible<()> {
        let sql = "delete from entries where entry_id = ?1;";
        self.conn.execute(sql, params![entry_id])?;
        Ok(())
    }

    pub fn list_entries(&self, date: NaiveDate) -> Fallible<Vec<Entry>> {
        let sql = "
            select
                entry_id, date, food_id, serving_id, amount, created_at
            from
                entries
            where
                date = ?1
            order by
                created_at;
        ";
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(params![date.format("%Y-%m-%d").to_string()], |row| {
            Ok(Entry {
                entry_id: row.get(0)?,
                date: NaiveDate::parse_from_str(&row.get::<_, String>(1)?, "%Y-%m-%d")
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                food_id: row.get(2)?,
                serving_id: row.get(3)?,
                amount: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        let mut entries = Vec::new();
        for entry in rows {
            entries.push(entry?);
        }
        Ok(entries)
    }
}
