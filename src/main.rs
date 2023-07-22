use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use commands::{search::SearchCommand, Command};

mod utils;
mod cli;
mod commands;

const VERSION: f32 = 2.0;

fn main() {
    let mut looping = true;

    while looping {
        let mut input = String::new();
        
        print!("RSWC (v{:?})> ", VERSION);
        stdout().flush().unwrap();
        
        let _ = stdin().read_line(&mut input);

        let parsed_input = cli::InputParser::new(input);
        let res = match parsed_input.command.as_str() {
            "search" => SearchCommand::new(parsed_input).run(),
            "exit" => {
                looping = false;
                return ()
            },
            _ => unimplemented!("Command <{}> not found.", parsed_input.command)
        };

        if res.is_err() {
            println!("{}", res.unwrap_err());
        }
    }
}
