use crate::config::{Config, DuplicateBehavior};
use crate::utils::{
    copy_all, delete, expand_path, get_home_and_dot_path, get_home_dir, get_path_in_dotfolder,
};
use dialoguer::MultiSelect;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

pub struct DotManager {
    config: Config,
    home_dir: PathBuf,
}
impl DotManager {
    pub fn new() -> DotManager {
        // init Config
        let config = Config::new();

        // init dotfolder
        let dotfolder_path = expand_path(&config.dotfolder_path).unwrap();
        if !dotfolder_path.exists() {
            fs::create_dir_all(&dotfolder_path).unwrap();
        }
        if !dotfolder_path.is_dir() {
            panic!("{} is not a directory", dotfolder_path.display());
        }

        Self {
            home_dir: get_home_dir(),
            config,
        }
    }

    pub fn sync(&self) {
        let mut duplicated_paths: Vec<(PathBuf, PathBuf)> = Vec::new();
        for path in &self.config.paths {
            let (path_in_home, path_in_dotfolder) = get_home_and_dot_path(path);

            if !path_in_home.starts_with(&self.home_dir) {
                panic!("{} is not inside the HOME directory", path);
            }
            if path_in_home.is_symlink() && !path_in_home.exists() {
                delete(&path_in_home);
            }
            match (path_in_home.exists(), path_in_dotfolder.exists()) {
                (true, false) => {
                    // Init case: copy from home to dotfolder, delete original, create symlink
                    copy_all(&path_in_home, &path_in_dotfolder).unwrap();
                    delete(&path_in_home);
                    symlink(&path_in_dotfolder, &path_in_home).expect("Failed to create symlink");
                }
                (false, true) => {
                    // Restore symlink: original missing but dotfolder has it
                    symlink(&path_in_dotfolder, &path_in_home)
                        .expect("Failed to re-create symlink");
                }
                (true, true) => {
                    match self.config.defaults.on_duplicate {
                        DuplicateBehavior::Ask => {
                            duplicated_paths.push((path_in_home, path_in_dotfolder));
                        }
                        DuplicateBehavior::OverwriteHome => {
                            delete(&path_in_home);
                            symlink(&path_in_dotfolder, &path_in_home)
                                .expect("Failed to create symlink");
                        }
                        DuplicateBehavior::OverwriteDotfile => {
                            delete(&path_in_dotfolder);
                            copy_all(&path_in_home, &path_in_dotfolder).unwrap();
                            delete(&path_in_home);
                            symlink(&path_in_dotfolder, &path_in_home)
                                .expect("Failed to create symlink");
                        }
                        DuplicateBehavior::BackupHome => {
                            let backup = path_in_home.with_extension("bak");
                            fs::rename(&path_in_home, &backup)
                                .expect("Failed to create backup of Home path");
                            symlink(&path_in_dotfolder, &path_in_home)
                                .expect("Failed to create symlink");
                        }
                        DuplicateBehavior::Skip => {
                            // println!("Skipping duplicated path: {}", path_in_home.display());
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
        }
        if !duplicated_paths.is_empty() {
            self.process_duplicated(duplicated_paths);
        }
    }

    fn process_duplicated(&self, doulicted_paths: Vec<(PathBuf, PathBuf)>) {
        // TODO add preset behave by a config or passed parameter
        println!(
            "\nSome files exist in both your home and dotfolder.\n\
             Select the ones to KEEP from home.\n\
             - 'Select All' = keep all home versions\n\
             - No selection = use dotfolder versions\n"
        );
        let options = [
            vec!["Select All"],
            doulicted_paths
                .iter()
                .map(|it| it.0.to_str().unwrap())
                .collect::<Vec<_>>(),
        ]
        .concat();

        let selected = MultiSelect::new().items(&options).interact().unwrap();

        let selected_indices = if !selected.is_empty() && selected[0] == 0 {
            // "Select All" was picked
            (0..doulicted_paths.len()).collect::<Vec<_>>()
        } else {
            // Adjust indices (skip the "Select All" at 0)
            selected.iter().map(|i| i - 1).collect::<Vec<_>>()
        };
        // processing the selected paths
        for index in &selected_indices {
            // removing the selected path from the list
            let path = doulicted_paths.get(*index).expect("index out of range");
            // deleting the unwanted path in the dotfolder
            delete(&path.1);
            // copy the new path
            copy_all(&path.0, &path.1).unwrap();
            // deleting the path form the home
            delete(&path.0);
            // create a symlink
            symlink(&path.1, &path.0).expect("Failed to create symlink");
        }

        // processing the unselected paths
        for path in doulicted_paths {
            delete(&path.0);
            symlink(&path.1, &path.0).expect("Failed to create symlink");
        }
    }

    pub fn delink_all(&self) {
        self.delink(&self.config.paths);
    }

    pub fn delink(&self, paths: &Vec<String>) {
        for path in paths {
            // Expand ~ or $HOME to absolute path
            let path_in_home = expand_path(path).expect("Failed to expand path");

            // Skip if not a symlink
            if !path_in_home.is_symlink() {
                continue;
            }

            // Build the full path in the dotfolder
            let path_in_dotfolder = get_path_in_dotfolder(&path_in_home).unwrap();

            // dbg!(&path_in_dotfolder);
            // Remove the symlink in home
            delete(&path_in_home);

            // Copy original file/dir from dotfolder back to home
            copy_all(&path_in_dotfolder, &path_in_home)
                .expect("Failed to copy from dotfolder to home");

            // TODO make this behavior optional ether by a flag or setting in the config file
            // delete(&path_in_dotfolder).expect("Failed to delete the path in the dotfolder");
        }
    }
}
