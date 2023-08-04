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
        match self.get_credentials() {
            Some(credentials) => writeln!(
                file , "login {} {}", credentials.0, credentials.1
            ),
            None => writeln!(file, "login anonymous")
        }.unwrap();

        for (app_id, ids) in state.ids_dict.iter() {
            for id in ids.iter() {
                writeln!(
                    file, 
                    "workshop_download_item {} {}",
                    app_id, 
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

impl DownloadCommand {
    fn get_credentials(&self) -> Option<(String, String)> {
        return match self.parsed_input.options.get("--u") {
            Some(username) => {
                let password = self.parsed_input.options.get("--p");
                if password.is_none() {
                    println!("Password for user <{}> not found", username);
                    return None;
                }

                return Some((username.clone(), password.unwrap().clone()));
            },
            None => None
        }
    }
}