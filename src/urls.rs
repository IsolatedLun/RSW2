use std::collections::HashMap;

use crate::_STATE;

pub fn create_search_url(args: &Vec<String>, kwargs: &HashMap<String, String>) -> Result<String, String> {
    let app_id_or_name = args[0].clone();
    let search_text = args[1].clone();
    let days = kwargs.get("--days").unwrap_or(&String::from("-1")).clone();
    let page = kwargs.get("--page").unwrap_or(&String::from("1")).clone();

    let mut _state = _STATE.lock().unwrap();
    let app_id: Option<String> = _state.try_get_app_id(app_id_or_name.clone());

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

pub fn create_collection_url(args: &Vec<String>, kwargs: &HashMap<String, String>) -> String {
    let id = args[1].clone();
    let url = format!("https://steamcommunity.com/sharedfiles/filedetails/?id={}", id);
    
    println!("{}", url);
    url
}