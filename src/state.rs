use std::collections::HashMap;

pub struct State {
    ids_dict: HashMap<String, Vec<String>>
}

impl State {
    pub fn new() -> Self {
        State {
            ids_dict: HashMap::new()
        }
    }

    pub fn add_ids(&mut self, app_id: String, ids: Vec<String>) {
        let selected_ids_len = ids.len();
        self.ids_dict.entry(app_id.clone()).or_insert(Vec::new()).extend(ids);


        let current_id_vec = self.ids_dict.get_mut(&app_id).unwrap();
        let original_ids_len = current_id_vec.len();
        println!("Added {} item(s)", selected_ids_len);

        current_id_vec.sort_unstable();
        current_id_vec.dedup();

        let filtered_ids_len = current_id_vec.len();
        if original_ids_len > filtered_ids_len {
            println!("Removed {} duplicate(s)", original_ids_len - filtered_ids_len);
        }
    }
}