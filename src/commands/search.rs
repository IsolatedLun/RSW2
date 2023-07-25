use std::vec;

use scraper::{Html, ElementRef};

use crate::_STATE;
use crate::cli::InputParser;
use crate::commands::Command;
use crate::utils::{fetch_page, create_search_url, create_selector, print_border, input, split_text_to_numbers};

pub struct SearchCommand {
    parsed_input: InputParser
}

impl Command<(String, Vec<String>), String> for SearchCommand {
    fn new(parsed_input: InputParser) -> Self {
        SearchCommand {
            parsed_input
        }
    }

    fn assert(&self) -> Result<(), String> {
        if self.parsed_input.args.len() < 2 {
            return Err(String::from(">> Insufficient arguments"));
        }

        Ok(())
    }

    fn run(&self) -> Result<(String, Vec<String>), String> {
        let assertion = self.assert();
        if assertion.is_err() {
            return Err(assertion.unwrap_err());
        }

        let url = create_search_url(&self.parsed_input.args, 
            &self.parsed_input.options);
        if url.is_err() {
            return Err(url.unwrap_err());
        }
        
        let html: Result<Html, String> = match fetch_page(url.unwrap()) {
            Ok(content) => Ok(scraper::Html::parse_document(&content)),
            Err(e) => Err(e) 
        };

        if html.is_err() {
            return Err(html.unwrap_err());
        }
        let html = html.unwrap();

        let app_name_selector = create_selector(".apphub_AppName");
        let app_name: String = html.select(&app_name_selector).next().unwrap().text().collect();
        match self.parsed_input.args[0].chars().all(char::is_numeric) {
            true => _STATE.lock().unwrap().set_alias(self.parsed_input.args[0].clone(), app_name),
            false => ()
        };
        
        let workshop_item_selector = create_selector(".workshopItem");
        let workshop_items: Vec<ElementRef> = html.select(&workshop_item_selector).collect();
        let ids: Vec<String> = SearchCommand::display_items(workshop_items);

        return match SearchCommand::select_ids(ids) {
            Err(e) => Err(e),
            Ok(res) => Ok((self.parsed_input.args[0].clone(), res))
        }
    }
}

impl SearchCommand {
    fn select_ids(ids: Vec<String>) -> Result<Vec<String>, String> {
        print!("Select by indexes > ");

        let _input = input();
        if _input.trim().len() == 0 {
            return Ok(vec![]);
        }

        let selected_indexes: Result<Vec<usize>, String> = match _input.contains(",") {
            true => split_text_to_numbers(_input, String::from(",")),
            false => split_text_to_numbers(_input, String::from(" "))
        };
        if selected_indexes.is_err() {
            return Err(format!(">> {}", selected_indexes.unwrap_err()));
        }

        let max_selected_idx = *selected_indexes.clone().unwrap().iter().max().unwrap();
        if max_selected_idx > ids.len() {
            return Err(format!(">> Index <{}> out of range", max_selected_idx));
        }

        let mut selected_ids: Vec<String> = vec![];
        selected_indexes.unwrap().into_iter().for_each(|idx| selected_ids.push(ids[idx].clone()));
    
        Ok(selected_ids)
    }

    fn display_items(items: Vec<ElementRef>) -> Vec<String> {
        print_border();

        let mut ids: Vec<String> = vec![];
        for (i, workshop_item) in items.into_iter().enumerate() {
            let item_name_selector = create_selector(".workshopItemTitle");
            let item_name: String = workshop_item.select(&item_name_selector).next().unwrap().text().collect();

            let item_link_selector = create_selector(".ugc");
            let item_link = workshop_item.select(&item_link_selector).next().unwrap();
            ids.push(item_link.value().attr("data-publishedfileid").unwrap().to_string());

            println!("{}) {}", i, item_name);
        }

        print_border(); 

        ids
    }
}