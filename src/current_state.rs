use crate::config::Config;
use crate::utils::expand_path;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CurrentState {
    pub paths: Vec<String>,
}

impl CurrentState {
    pub fn save(&self, config: &Config) {
        let new_current_state = CurrentState{
            paths: config.paths.clone(),
        };
        let dotfolder = expand_path(&config.dotfolder_path)
            .expect("Failed to expand dotfolder path");
        let path = dotfolder.join("current_state.toml");

        let toml_string = toml::to_string_pretty(&new_current_state)
            .expect("Failed to serialize current state");
        fs::write(path, toml_string)
            .expect("Failed to write current state file");
    }

    pub fn new(config: &Config) -> Self {
        let dotfolder = expand_path(&config.dotfolder_path)
            .expect("Failed to expand dotfolder path");
        let path = dotfolder.join("current_state.toml");

        if !path.exists() {
            eprintln!(
                "Warning: current state file does not exist at '{}', assuming empty state.",
                path.display()
            );
            return Self::default();
        }

        let contents = fs::read_to_string(&path)
            .expect("Failed to read current state file");
        toml::from_str(&contents)
            .expect("Failed to parse current state file")
    }
}
