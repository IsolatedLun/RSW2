use crate::_STATE;
use crate::cli::InputParser;
use crate::commands::Command as RCommand;
use crate::urls::create_collection_url;
use crate::utils::{fetch_then_parse_html, create_selector};

pub struct CollectionCommand {
    parsed_input: InputParser
}

impl RCommand<(String, Vec<String>), String> for CollectionCommand {
    fn new(parsed_input: InputParser) -> Self {
        CollectionCommand {
            parsed_input
        }
    }

    fn assert(&self) -> Result<(), String> {
        if self.parsed_input.args.len() < 1 {
            return Err(String::from("Insufficient arguments"))
        }

        Ok(())
    }

    fn run(&self) -> Result<(String, Vec<String>), String> {
        let assertion = self.assert();
        if assertion.is_err() {
            return Err(assertion.unwrap_err());
        }

        let mut _state = _STATE.lock().unwrap();
        let url = create_collection_url(&self.parsed_input.args.clone(), &self.parsed_input.options);
        let html: Result<scraper::Html, String> = fetch_then_parse_html(url);
        if html.is_err() {
            return Err(html.unwrap_err());
        }
        
        let app_id = _state.try_get_app_id(self.parsed_input.args[0].clone());
        if app_id.is_none() {
            return Err(format!(">> Alias <{}> does not exist.", self.parsed_input.args[0]));
        }

        let collection_item_selector = create_selector(".collectionItem");
        let mut ids: Vec<String> = vec![];
        for collection_item in html.unwrap().select(&collection_item_selector) {
            let (_, id) = collection_item.value().id().unwrap().split_once("_").unwrap();
            ids.push(id.to_string());
        }

        Ok((app_id.unwrap(), ids))
    }
}