use crate::cli::InputParser;

pub trait Command<T> {
    fn new(parsed_input: InputParser) -> Self;

    fn assert(&self) -> Result<(), String>;
    fn run(&self) -> Result<T, String>;
}

pub mod search;