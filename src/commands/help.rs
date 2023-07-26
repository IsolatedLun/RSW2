use crate::cli::InputParser;
use crate::commands::Command as RCommand;

pub struct HelpCommand {
    parsed_input: InputParser
}

impl RCommand<(), ()> for HelpCommand {
    fn new(parsed_input: InputParser) -> Self {
        HelpCommand {
            parsed_input
        }
    }

    fn assert(&self) -> Result<(), String> {
        Ok(())
    }

    fn run(&self) -> Result<(), ()> {
        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "Command", "Args", "Options", "Description"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "help", "-", "-", "-"
        );
        
        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "search", "[appAlias] [query]", "--pages [number]", "Search workshop items (sorted by top & most popular)"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "collection", "[appAlias] [collectionId]", "-", "Adds all items of the collection"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "clear", "-", "-", "Clears all items"
        );

        Ok(())
    }
}