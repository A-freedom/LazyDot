use crate::utils::{check_path, get_home_dir};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct Config {
    pub defaults: Defaults,

    // Always treat these paths as unexpanded. Use expand_path() before any real use.
    pub dotfolder_path: String,
    pub paths: Vec<String>,
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

impl Config {
    pub fn new() -> Config {
        let config_file = get_home_dir().join(".config/lazydot.toml");
        if !config_file.exists() {
            return Config {
                dotfolder_path: "~/mydotfolder".to_owned(),
                paths: vec![],
                defaults: Defaults {
                    on_duplicate: DuplicateBehavior::Ask,
                },
            };
        };
        let content = fs::read_to_string(&config_file).expect(
            format!(
                "Unable to read config file: {}",
                config_file.to_str().unwrap()
            )
            .as_str(),
        );
        let config: Config = toml::from_str(&content).expect("Failed to parse lazydot.toml");
        config.validate_config();
        config
    }
    pub fn save(&self) {
        self.validate_config();

        let config_file = get_home_dir().join(".config/lazydot.toml");

        let toml_string = toml::to_string_pretty(self).expect("Failed to serialize config");
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
    fn validate_config(&self) {
        for path in &self.paths {
            if !(path.starts_with("~/") || path.starts_with("/")) {
                panic!(
                    "Invalid path: \"{}\" every path must start with `~/` or `/`",
                    path
                );
            }
        }
        if !self.dotfolder_path.starts_with("~/") {
            panic!(
                "Invalid path: \"{}\" every path must start with `~/`",
                self.dotfolder_path
            );
        }
    }
}
