use std::collections::HashMap;
use std::{fs::File, io::Write};
use std::process::Command;
use std::vec;

use crate::utils::get_absolute_path;
use crate::{_STATE, STEAMCMD_DIR};
use crate::cli::InputParser;
use crate::commands::Command as RCommand;

pub struct DownloadCommand {
    parsed_input: InputParser
}

impl RCommand<String, String> for DownloadCommand {
    fn new(parsed_input: InputParser) -> Self {
        DownloadCommand {
            parsed_input
        }
    }

    fn assert(&self) -> Result<(), String> {
        Ok(())
    }

    fn run(&self) -> Result<String, String> {
        let assertion = self.assert();
        if assertion.is_err() {
            return Err(assertion.unwrap_err());
        }

        let state = _STATE.lock().unwrap();
        let mut file = File::options()
            .create(true)
            .write(true)
            .open("output.txt")
            .unwrap();

        let mut command = Command::new("cmd");
        let temp_aliases: HashMap<String, String> = state.aliases.clone();

        writeln!(file, "login anonymous").unwrap();
        for (app_name, ids) in state.ids_dict.iter() {
            for id in ids.iter() {
                writeln!(
                    file, 
                    "workshop_download_item {} {}",
                    temp_aliases.get(app_name).unwrap().clone(), 
                    id.clone()
                ).unwrap();
            }
        }
        writeln!(file, "quit").unwrap();
        println!(">> Downloading...");
        
        let abs_file_path = get_absolute_path(String::from("output.txt"));
        println!("{}", abs_file_path);
        
        command.args(vec!["/C", "start", "cmd", "/K", "steamcmd +runscript", abs_file_path.as_str()]);
        command.current_dir(STEAMCMD_DIR);

        return match command.output() {
            Ok(_) => Ok(String::from(">> Downloaded items")),
            Err(e) => Err(format!(">> Failed downloading items: {}", e.to_string()))
        }
    }
}