use crate::utils::{check_path, get_home_dir};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct Config {
    pub dotfolder_path: String,
    pub paths: Vec<String>,
    pub defaults: Defaults,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Defaults {
    #[serde(default = "default_duplicate_behavior")]
    pub on_duplicate: DuplicateBehavior,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DuplicateBehavior {
    Ask,
    OverwriteHome,
    OverwriteDotfile,
    BackupHome,
    Skip,
}
fn default_duplicate_behavior() -> DuplicateBehavior {
    DuplicateBehavior::Ask
}

// TODO implement returning Result
impl Config {
    pub fn new() -> Config {
        let config_file = get_home_dir().unwrap().join(".config/lazydot.toml");
        if !config_file.exists() {
            return Config {
                dotfolder_path: "~/mydotfolder".to_owned(),
                paths: vec![],
                defaults: Defaults {
                    on_duplicate: DuplicateBehavior::Ask,
                },
            };
        };
        let content = fs::read_to_string(&config_file).unwrap();
        let config: Config = toml::from_str(&content).expect("Failed to parse lazydot.toml");

        config
    }
    pub fn save(&self) {
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
            return Ok(());
        }
        self.paths.push(path);
        self.save();
        Ok(())
    }

    pub fn remove_path(&mut self, path: String) {
        let path = check_path(&path).expect("Path does not exist");
        for (i, v) in self.paths.iter().enumerate() {
            if *v == path {
                self.paths.remove(i);
                self.save();
                return;
            }
        }
    }
}
