use std::{io::{stdout, stdin, Write}, path::PathBuf, fs};

use scraper::Selector;

pub fn fetch_page(url: String) -> Result<String, String> {
    match reqwest::blocking::get(url) {
        Ok(res) => Ok(res.text().unwrap()),
        Err(e) => Err(format!("[{}]: {}", e.status().unwrap(), e.to_string()))
    }
}

pub fn parse_html(html: String) -> scraper::Html {
    scraper::Html::parse_document(&html)
}

pub fn fetch_then_parse_html(url: String) -> Result<scraper::Html, String> {
    return match fetch_page(url) {
        Ok(content) => Ok(parse_html(content)),
        Err(e) => Err(e)
    }
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