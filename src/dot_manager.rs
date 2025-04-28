use crate::config::{Config, DuplicateBehavior, OnDelinkBehavior};
use crate::current_state::CurrentState;
use crate::utils::{copy_all, delete, expand_path, get_home_and_dot_path, get_path_in_dotfolder};
use dialoguer::MultiSelect;
use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

pub struct DotManager {
    // the pub(crate) is needed for testing
    pub(crate) config: Config,
    pub(crate) current_state: CurrentState,
}

impl DotManager {
    pub fn new() -> DotManager {
        let config = Config::new();
        let dotfolder_path = expand_path(&config.dotfolder_path);
        if !dotfolder_path.exists() {
            fs::create_dir_all(&dotfolder_path).expect(&format!(
                "Failed to create the dotfolder folder: {}",
                dotfolder_path.display()
            ));
        }
        if !dotfolder_path.is_dir() {
            panic!("{} is not a directory", dotfolder_path.display());
        }

        Self {
            current_state: CurrentState::new(&config),
            config,
        }
    }

    pub fn sync(&self) {
        let paths_tobe_unlinked =
            Self::find_paths_to_unlink(&self.current_state.paths, &self.config.paths);
        self.delink(&paths_tobe_unlinked);

        let mut duplicated_paths: Vec<(PathBuf, PathBuf)> = Vec::new();

        for path in &self.config.paths {
            print!("Linking: ");
            let (path_in_home, path_in_dotfolder) = get_home_and_dot_path(path);

            // check if `path_in_home` is broken link
            if path_in_home.is_symlink() && !path_in_home.exists() {
                delete(&path_in_home);
            }

            match (path_in_home.exists(), path_in_dotfolder.exists()) {
                (true, false) => {
                    copy_all(&path_in_home, &path_in_dotfolder).unwrap();
                    delete(&path_in_home);
                    if let Err(e) = symlink(&path_in_dotfolder, &path_in_home) {
                        println!("Failed to create symlink: {}", e);
                        continue;
                    }
                }
                (false, true) => {
                    if let Err(e) = symlink(&path_in_dotfolder, &path_in_home) {
                        println!("Failed to create symlink: {}", e);
                        continue;
                    }
                }
                (true, true) => {
                    match self.config.defaults.on_duplicate {
                        DuplicateBehavior::Ask => {
                            if path_in_home
                                .canonicalize()
                                .expect("Failed to canonicalize path")
                                .eq(&path_in_dotfolder)
                            {
                                continue;
                            }
                            duplicated_paths.push((path_in_home, path_in_dotfolder));
                        }
                        DuplicateBehavior::OverwriteHome => {
                            delete(&path_in_home);
                            if let Err(e) = symlink(&path_in_dotfolder, &path_in_home) {
                                println!("Failed to create symlink: {}", e);
                                continue;
                            }
                        }
                        DuplicateBehavior::OverwriteDotfile => {
                            delete(&path_in_dotfolder);
                            copy_all(&path_in_home, &path_in_dotfolder).unwrap();
                            delete(&path_in_home);
                            if let Err(e) = symlink(&path_in_dotfolder, &path_in_home) {
                                println!("Failed to create symlink: {}", e);
                                continue;
                            }
                        }
                        DuplicateBehavior::BackupHome => {
                            let backup = path_in_home.with_extension("bak");
                            fs::rename(&path_in_home, &backup)
                                .expect("Failed to create backup of Home path");
                            if let Err(e) = symlink(&path_in_dotfolder, &path_in_home) {
                                println!("Failed to create symlink: {}", e);
                                continue;
                            }
                        }
                        DuplicateBehavior::Skip => {
                            // skip
                        }
                    }
                }
                (false, false) => {
                    println!(
                        "Warning: path doesn't exist in home or dotfolder, skipping. \n {}",
                        path_in_home.display()
                    );
                }
            }
            println!("{} linked successfully", path);
        }
        if !duplicated_paths.is_empty() {
            self.process_duplicated(duplicated_paths);
        }

        self.current_state.save(&self.config);
    }

    fn process_duplicated(&self, duplicated_paths: Vec<(PathBuf, PathBuf)>) {
        println!(
            "\nSome files exist in both your home and dotfolder.\n\
             Select the ones to KEEP from home.\n\
             - 'Select All' = keep all home versions\n\
             - No selection = use dotfolder versions\n"
        );

        let options = [
            vec!["Select All"],
            duplicated_paths
                .iter()
                .map(|it| it.0.to_str().unwrap())
                .collect::<Vec<_>>(),
        ]
        .concat();

        let selected = MultiSelect::new()
            .items(&options)
            .interact()
            .expect("Failed to select");

        let selected_indices = if !selected.is_empty() && selected[0] == 0 {
            (0..duplicated_paths.len()).collect::<Vec<_>>()
        } else {
            selected.iter().map(|i| i - 1).collect::<Vec<_>>()
        };

        // Handle selected paths
        for index in &selected_indices {
            print!("Overwriting Home with Dotfile: ");
            let path = duplicated_paths.get(*index).expect("Index out of range");
            delete(&path.1);
            copy_all(&path.0, &path.1).unwrap();
            delete(&path.0);
            if let Err(e) = symlink(&path.1, &path.0) {
                println!("Failed to create symlink: {}", e);
                continue;
            }
            println!("{} successfully", path.0.display());
        }

        // Handle unselected paths
        for (i, path) in duplicated_paths.into_iter().enumerate() {
            print!("Keeping Home: ");
            if selected_indices.contains(&i) {
                continue; // already handled
            }
            delete(&path.0);
            if let Err(e) = symlink(&path.1, &path.0) {
                println!("Failed to create symlink: {}", e);
                continue;
            }
            println!("{} successfully", path.0.display());
        }
    }

    pub fn delink_all(&self) {
        self.delink(&self.config.paths);
    }

    pub fn delink(&self, paths: &[String]) {
        for path in paths {
            print!("Unlinking: ");
            let path_in_home = expand_path(path);

            if !path_in_home.is_symlink() {
                println!("{} is not a symlink", path);
                continue;
            }

            let path_in_dotfolder =
                get_path_in_dotfolder(&path_in_home).expect("Failed to get path in dotfolder");

            if !path_in_dotfolder.exists() {
                println!("{} doesn't exist in dotfolder", path);
                continue;
            }
            if !path_in_home
                .canonicalize()
                .expect("Failed to canonicalize path")
                .eq(&path_in_dotfolder)
            {
                println!("{} is not a symlink to dotfolder", path);
                continue;
            }

            delete(&path_in_home);

            copy_all(&path_in_dotfolder, &path_in_home)
                .expect("Failed to copy from dotfolder to home");
            match self.config.defaults.on_delink {
                OnDelinkBehavior::Remove => {
                    delete(&path_in_dotfolder);
                }
                OnDelinkBehavior::Keep => {}
            }
            println!("{} delinked successfully", path);
        }
    }

    fn find_paths_to_unlink(current_paths: &[String], config_paths: &[String]) -> Vec<String> {
        let current_set: HashSet<_> = current_paths.iter().collect();
        let config_set: HashSet<_> = config_paths.iter().collect();

        current_set
            .difference(&config_set)
            .map(|s| (*s).clone())
            .collect()
    }
}