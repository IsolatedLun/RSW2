use scraper::{Html, ElementRef};

use crate::cli::InputParser;
use crate::commands::Command;
use crate::utils::{fetch_page, create_search_url};

pub struct SearchCommand {
    parsed_input: InputParser
}

impl Command for SearchCommand {
    fn new(parsed_input: InputParser) -> Self {
        SearchCommand {
            parsed_input
        }
    }

    fn assert(&self) -> Result<(), String> {
        if self.parsed_input.args.len() < 1 {
            return Err(String::from("Insufficient argument: No query"));
        }

        Ok(())
    }

    fn run(&self) -> Result<(), String> {
        let assertion = self.assert();
        if assertion.is_err() {
            return assertion;
        }

        let html: Result<Html, String> = match fetch_page(
            create_search_url(&self.parsed_input.args, &self.parsed_input.options)
        ) {
            Ok(content) => Ok(scraper::Html::parse_document(&content)),
            Err(e) => Err(e) 
        };

        if html.is_err() {
            return Err(html.unwrap_err());
        }
        let html = html.unwrap();
        
        let workshop_item_selector = scraper::Selector::parse(".workshopItem").unwrap();
        let workshop_items: Vec<ElementRef> = html.select(&workshop_item_selector).collect();
        SearchCommand::display_items(workshop_items);
        
        assertion
    }
}

impl SearchCommand {
    fn display_items(items: Vec<ElementRef>) {
        for (i, workshop_item) in items.into_iter().enumerate() {
            let item_name_selector = scraper::Selector::parse(".workshopItemTitle").unwrap();
            let item_name: String = workshop_item.select(&item_name_selector).flat_map(|el| el.text()).collect();

            println!("{}) {}", i, item_name);
        }
    }
}