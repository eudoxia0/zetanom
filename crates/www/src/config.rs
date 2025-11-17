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

use std::fs;
use std::path::PathBuf;

use error::AppError;
use error::Fallible;
use serde::Deserialize;

pub struct Config {
    /// Absolute, canonicalized path to the SQLite3 database.
    db_path: PathBuf,
    /// Port in which to run the server.
    port: u16,
}

#[derive(Deserialize)]
struct ConfigFile {
    db_path: PathBuf,
    port: u16,
}

impl Config {
    /// Load the configuration from `~/.config/zetanom/config.toml`.
    pub fn load() -> Fallible<Self> {
        // Get the home directory
        let home = std::env::var("HOME")
            .map_err(|_| AppError::new("HOME environment variable not set"))?;

        // Construct the config file path
        let config_path = PathBuf::from(home)
            .join(".config")
            .join("zetanom")
            .join("config.toml");

        // Read the config file
        let contents = fs::read_to_string(&config_path).map_err(|e| {
            AppError::new(format!(
                "Failed to read config file at {}: {}",
                config_path.display(),
                e
            ))
        })?;

        // Parse the TOML
        let config_file: ConfigFile = toml::from_str(&contents)
            .map_err(|e| AppError::new(format!("Failed to parse config file: {}", e)))?;

        // Canonicalize the database path
        let db_path = fs::canonicalize(&config_file.db_path).map_err(|e| {
            AppError::new(format!(
                "Failed to canonicalize database path {}: {}",
                config_file.db_path.display(),
                e
            ))
        })?;

        Ok(Config {
            db_path,
            port: config_file.port,
        })
    }
}
