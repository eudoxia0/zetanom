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

use std::ops::Add;

use crate::db::*;

/// The nutritional content of some amount of food.
#[derive(Clone, Copy)]
pub struct Nutrition {
    pub energy: Energy,
    pub protein: Protein,
    pub fat: Fat,
    pub fat_saturated: SaturatedFat,
    pub carbs: Carbs,
    pub carbs_sugars: Sugars,
    pub fibre: Fibre,
    pub sodium: Sodium,
}

impl Nutrition {
    pub fn scale(self, factor: f64) -> Self {
        Self {
            energy: self.energy * factor,
            protein: self.protein * factor,
            fat: self.fat * factor,
            fat_saturated: self.fat_saturated * factor,
            carbs: self.carbs * factor,
            carbs_sugars: self.carbs_sugars * factor,
            fibre: self.fibre * factor,
            sodium: self.sodium * factor,
        }
    }
}

impl Add<Nutrition> for Nutrition {
    type Output = Nutrition;

    fn add(self, rhs: Nutrition) -> Nutrition {
        Self {
            energy: self.energy + rhs.energy,
            protein: self.protein + rhs.protein,
            fat: self.fat + rhs.fat,
            fat_saturated: self.fat_saturated + rhs.fat_saturated,
            carbs: self.carbs + rhs.carbs,
            carbs_sugars: self.carbs_sugars + rhs.carbs_sugars,
            fibre: self.fibre + rhs.fibre,
            sodium: self.sodium + rhs.sodium,
        }
    }
}
