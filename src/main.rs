use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use commands::{search::SearchCommand, Command};
use utils::{input};
use state::State;

mod utils;
mod cli;
mod state;
mod commands;

const VERSION: f32 = 2.0;

fn main() {
    let mut state = State::new();
    let mut looping = true;

    while looping {
        print!("RWS (v{:?})> ", VERSION);
        
        let parsed_input = cli::InputParser::new(input());
        match parsed_input.command.as_str() {
            "search" => {
                match SearchCommand::new(parsed_input).run() {
                    Err(e) => println!("{}", e),
                    Ok(res) => state.add_ids(res.0, res.1)
                }
            },
            "exit" => {
                looping = false;
                return ()
            },
            _ => println!("Command <{}> not found.", parsed_input.command)
        };
    }
}
