use std::{collections::HashMap, io::{stdout, stdin, Write}, path::PathBuf, fs};

use scraper::Selector;

use crate::_STATE;

pub fn fetch_page(url: String) -> Result<String, String> {
    match reqwest::blocking::get(url) {
        Ok(res) => Ok(res.text().unwrap()),
        Err(e) => Err(format!("[{}]: {}", e.status().unwrap(), e.to_string()))
    }
}

pub fn create_search_url(args: &Vec<String>, kwargs: &HashMap<String, String>) -> Result<String, String> {
    let app_id_or_name = args[0].clone();
    let search_text = args[1].clone();
    let days = kwargs.get("--days").unwrap_or(&String::from("-1")).clone();
    let page = kwargs.get("--page").unwrap_or(&String::from("1")).clone();

    let mut _state = _STATE.lock().unwrap();
    let app_id: Option<&String> = match app_id_or_name.chars().all(char::is_numeric) {
        true => Some(&app_id_or_name),
        false => _state.get_alias(app_id_or_name.clone()),
    };

    if app_id.is_none() {
        return Err(format!(">> Alias <{}> does not exist.", app_id_or_name));
    }

    let url = format!(
        "https://steamcommunity.com/workshop/browse/?appid={}&searchtext={}&days={}&p={}",
        app_id.unwrap(), search_text, days, page
    );

    println!("{}", url);
    Ok(url)
}

pub fn create_selector(selector: &str) -> Selector {
    scraper::Selector::parse(selector).unwrap()
}

pub fn print_border() {
    println!("{}", "=".repeat(16));
}

pub fn input() -> String {
    let mut input: String = String::new();
    stdout().flush().unwrap();
    let _ = stdin().read_line(&mut input);

    input
}

pub fn clean_text(text: impl Into<String>) -> String {
    let string: String = text.into();

    string.to_lowercase().replace(" ", "_")
}

pub fn split_text_to_numbers(text: String, split_by: String) -> Result<Vec<usize>, String> {
    let mut num_vec: Vec<usize> = vec![];
    for x in text.split(&split_by).into_iter() {
        let res = x.trim().parse::<usize>();
        if res.is_err() {
            return Err(res.unwrap_err().to_string())
        }
        
        num_vec.push(res.unwrap());
    }

    Ok(num_vec)
}

pub fn get_absolute_path(relative_path: String) -> String {
    fs::canonicalize(PathBuf::from(relative_path)).unwrap().to_str().unwrap()[4..].to_string()
}