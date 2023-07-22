use std::{ops::Index, collections::HashMap};

pub fn fetch_page(url: String) -> Result<String, String> {
    match reqwest::blocking::get(url) {
        Ok(res) => Ok(res.text().unwrap()),
        Err(e) => Err(format!("[{}]: {}", e.status().unwrap(), e.to_string()))
    }
}

pub fn create_search_url(args: &Vec<String>, kwargs: &HashMap<String, String>) -> String {
    let app_id = args[0].clone();
    let search_text = args[1].clone();
    let days = kwargs.get("--days").unwrap_or(&String::from("-1")).clone();
    let page = kwargs.get("--page").unwrap_or(&String::from("1")).clone();

    let url = format!(
        "https://steamcommunity.com/workshop/browse/?appid={}&searchtext={}&days={}&p={}",
        app_id, search_text, days, page
    );

    println!("{}", url);
    url
}