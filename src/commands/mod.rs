use crate::cli::InputParser;

pub trait Command<T, E> {
    fn new(parsed_input: InputParser) -> Self;

    fn assert(&self) -> Result<(), String>;
    fn run(&self) -> Result<T, E>;
}

pub mod search;
pub mod download;
pub mod help;