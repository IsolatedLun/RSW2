use crate::cli::InputParser;

pub trait Command {
    fn new(parsed_input: InputParser) -> Self;

    fn assert(&self) -> Result<(), String>;
    fn run(&self) -> Result<(), String>;
}

pub mod search;