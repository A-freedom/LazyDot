use crate::utils::{check_path, get_home_dir};
use serde::Deserialize;
use std::fs;
use std::io::ErrorKind;

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct Config {
    pub dotfolder_path: String,
    pub paths: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        let config_file = get_home_dir().unwrap().join(".config/lazydot.toml");
        if !config_file.exists() {
            return Config {
                dotfolder_path: "~/mydotfolder".to_owned(),
                paths: vec![],
            };
        };
        let content = fs::read_to_string(&config_file).unwrap();
        let config: Config = toml::from_str(&content).expect("Failed to parse lazydot.toml");

        config
    }
    fn save(&self) {
        let config_file = get_home_dir().unwrap().join(".config/lazydot.toml");

        let toml_string = toml::to_string(self).expect("Failed to serialize config");
        if let Err(e) = fs::write(&config_file, &toml_string) {
            if e.kind() == ErrorKind::NotFound {
                fs::File::create(&config_file).expect("Couldn't create config file");
                fs::write(config_file, &toml_string)
                    .expect("Failed to write after creating config");
            } else {
                panic!("Failed to write config: {}", e);
            }
        }
    }

    pub fn add_path(&mut self, path: String) -> Result<(), String> {
        let path = check_path(&path)?;
        if self.paths.contains(&path) {
            println!("{} is already exist", &path);
            return Ok(());
        }
        println!("adding {}", &path);
        self.paths.push(path);
        self.save();
        Ok(())
    }

    pub fn remove_path(&mut self, path: String) {
        // TODO fix this bug. the bug is that you can't remove a path that have been deleted
        // and find a way to test for the output print of these functions
        let path = check_path(&path).expect("Path does not exist");
        for (i, v) in self.paths.iter().enumerate() {
            if *v == path {
                self.paths.remove(i);
                self.save();
                return println!("path is deleted");
            }
        }
        println!("path is already not exist");
    }
}
