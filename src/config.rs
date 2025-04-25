use crate::create_toml_temp::create_default_config_if_missing;
use crate::utils::{check_path, get_home_dir};
use serde::{Deserialize, Serialize};
use std::fs;
use toml_edit::{Array, DocumentMut, Item, Value};

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

    #[serde(default = "default_on_delink_behavior")]
    pub on_delink: OnDelinkBehavior,
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
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OnDelinkBehavior {
    Remove,
    Keep,
}
fn default_on_delink_behavior() -> OnDelinkBehavior {
    OnDelinkBehavior::Remove
}

impl Config {
    pub fn new() -> Config {
        let config_file = get_home_dir().join(".config/lazydot.toml");
        create_default_config_if_missing();
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

        let content =
            fs::read_to_string(&config_file).expect("Failed to read existing config for update");

        let mut doc = content
            .parse::<DocumentMut>()
            .expect("Failed to parse config as TOML document");

        doc["dotfolder_path"] = toml_edit::value(&self.dotfolder_path);

        // Manually construct the array for paths
        let mut paths_array = Array::default();
        for path in &self.paths {
            paths_array.push(path.as_str());
        }
        doc["paths"] = Item::Value(Value::Array(paths_array));

        doc["defaults"]["on_duplicate"] =
            toml_edit::value(format!("{:?}", self.defaults.on_duplicate).to_lowercase());

        doc["defaults"]["on_delink"] =
            toml_edit::value(format!("{:?}", self.defaults.on_delink).to_lowercase());
        
        fs::write(config_file, doc.to_string())
            .expect("Failed to write updated config with preserved comments");
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
