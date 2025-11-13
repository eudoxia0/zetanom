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

use std::io::Write;
use std::io::stdin;
use std::io::stdout;

use error::Fallible;

use crate::db::Db;

pub fn start_repl() -> Fallible<()> {
    let db = Db::new()?;
    loop {
        print!("> ");
        flush()?;
        let l = readline()?;
        match l.as_ref() {
            "count" => {
                let c = db.count_foods()?;
                println!("The library has {c} foods.");
            }
            "q" => {
                println!("Bye!");
                break;
            }
            _ => {
                println!("Unknown command.");
            }
        }
    }
    Ok(())
}

fn flush() -> Fallible<()> {
    stdout().flush()?;
    Ok(())
}

fn readline() -> Fallible<String> {
    let mut buf = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}
