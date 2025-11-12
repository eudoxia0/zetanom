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

use core::cli::Command;
use core::repl::start_repl;
use core::www::start_server;
use std::process::ExitCode;

use clap::Parser;

#[tokio::main]
async fn main() -> ExitCode {
    let c: Command = Command::parse();
    let res = match c {
        Command::Repl => start_repl(),
        Command::Serve => start_server().await,
    };
    match res {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("zetanom: {e}");
            ExitCode::FAILURE
        }
    }
}
