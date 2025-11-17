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

use error::Fallible;
use rusqlite::Connection;
use rusqlite::config::DbConfig;

pub struct Db {
    conn: Connection,
}

pub type FoodId = i64;

pub enum ServingUnit {
    Grams,
    Milliliters,
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
    pub sodium: Sodium,
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
        todo!()
    }
}
