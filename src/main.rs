use std::sync::Mutex;

use commands::{Command, search::SearchCommand, download::DownloadCommand, help::HelpCommand};
use once_cell::sync::Lazy;
use utils::input;
use state::State;

use crate::commands::collection::CollectionCommand;

mod utils;
mod cli;
mod state;
mod commands;
mod urls;

static _STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::new()));

const VERSION: f32 = 2.0;
const STEAMCMD_DIR: &str = r#"C:/Users/AJC/Desktop/steamcmd"#;

fn main() {
    let mut looping = true;

    while looping {
        print!("RWS (v{:?})> ", VERSION);
        
        let parsed_input = cli::InputParser::new(input());
        match parsed_input.command.as_str() {
            "search" => {
                match SearchCommand::new(parsed_input).run() {
                    Err(e) => println!("{}", e),
                    Ok(res) => _STATE.lock().unwrap().add_ids(res.0, res.1)
                }
            },
            "collection" => {
                match CollectionCommand::new(parsed_input).run() {
                    Err(e) => println!("{}", e),
                    Ok(res) => _STATE.lock().unwrap().add_ids(res.0, res.1)
                }
            },
            "download" => {
                let res = DownloadCommand::new(parsed_input).run();
                println!("{}", res.unwrap());
            },
            "help" => {
                HelpCommand::new(parsed_input).run().unwrap();
            },
            "clear" => {
                _STATE.lock().unwrap().clear();
            },
            "exit" => looping = false,
            _ => println!(">> Command <{}> not found.", parsed_input.command)
        };
    }

    println!("Exitting...")
}
