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

pub fn start_repl() -> () {
    loop {
        print!("> ");
        flush();
        let l = readline();
        println!("Echo: {l}");
    }
}

fn flush() {
    stdout().flush().unwrap();
}

fn readline() -> String {
    let mut buf = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
