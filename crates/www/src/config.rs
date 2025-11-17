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

use std::path::PathBuf;

use error::Fallible;

pub struct Config {
    /// Absolute, canonicalized path to the SQLite3 database.
    db_path: PathBuf,
    /// Port in which to run the server.
    port: u16,
}

impl Config {
    /// Load the configuration from `~/.config/zetanom/config.toml`.
    pub fn load() -> Fallible<Self> {
        todo!()
    }
}
