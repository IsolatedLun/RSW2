use std::{collections::HashMap, fs::File, io::{Read, Write}};

use crate::utils::clean_text;

pub struct State {
    ids_dict: HashMap<String, Vec<String>>,
    aliases: HashMap<String, String>
}

impl State {
    pub fn new() -> Self {
        State {
            ids_dict: HashMap::new(),
            aliases: State::load_aliases()
        }
    }

    pub fn set_alias(&mut self, app_id: String, name: String) {
        let added = self.aliases.insert(clean_text(&name), app_id);
        if added.is_none() {
            println!(">> Added alias <{}>", clean_text(name));

            let mut file = File::options()
                .write(true)
                .open("data/aliases.json")
                .unwrap();

            let json_content = serde_json::to_string(&self.aliases).unwrap();

            file.write_all(json_content.as_bytes()).unwrap();
        }
    }

    pub fn load_aliases() -> HashMap<String, String> {
        let mut file: File = match File::open("data/aliases.json") {
            Ok(f) => f,
            Err(_) => File::create("data/aliases.json").unwrap()
        };
        let mut buf: String = String::new();
        file.read_to_string(&mut buf).unwrap();

        serde_json::from_str(&buf).unwrap()
    }

    pub fn get_alias(&mut self, name: String) -> Option<&String> {
        self.aliases.get(&name) 
    }

    pub fn add_ids(&mut self, app_id: String, ids: Vec<String>) {
        let selected_ids_len = ids.len();
        self.ids_dict.entry(app_id.clone()).or_insert(Vec::new()).extend(ids);


        let current_id_vec = self.ids_dict.get_mut(&app_id).unwrap();
        let original_ids_len = current_id_vec.len();
        println!(">> Added {} item(s)", selected_ids_len);

        current_id_vec.sort_unstable();
        current_id_vec.dedup();

        for x in current_id_vec.iter() {
            println!("{}", x);
        }
        
        let filtered_ids_len = current_id_vec.len();
        if original_ids_len > filtered_ids_len {
            println!(">> Removed {} duplicate(s)", original_ids_len - filtered_ids_len);
        }
    }
}