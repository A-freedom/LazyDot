use crate::config::Config;
use crate::utils::{copy_all, delete, expand_path, get_home_dir};
use dialoguer::MultiSelect;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
pub struct DotManger {
    config: Config,
    home_dir: PathBuf,
    dotfolder_path: PathBuf,
}
impl DotManger {
    pub fn new() -> DotManger {
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
            home_dir: get_home_dir().unwrap(),
            config,
            dotfolder_path,
        }
    }

    pub fn sync(&self) {
        let mut duplicated_paths: Vec<(PathBuf, PathBuf)> = Vec::new();
        for path in &self.config.paths {
            let path_in_home = expand_path(path).expect("Failed to expand path");

            if !path_in_home.starts_with(&self.home_dir) {
                panic!("{} is not inside the HOME directory", path);
            }

            let relative = path_in_home.strip_prefix(&self.home_dir).unwrap();
            let path_in_dotfolder = self.dotfolder_path.join(&relative);

            match (path_in_home.exists(), path_in_dotfolder.exists()) {
                (true, false) => {
                    // Init case: copy from home to dotfolder, delete original, create symlink
                    fs::create_dir_all(path_in_dotfolder.parent().unwrap()).unwrap();
                    copy_all(&path_in_home, &path_in_dotfolder).unwrap();

                    delete(&path_in_home).expect("Failed to delete original");
                    symlink(&path_in_dotfolder, &path_in_home).expect("Failed to create symlink");
                }
                (false, true) => {
                    // Restore symlink: original missing but dotfolder has it
                    symlink(&path_in_dotfolder, &path_in_home)
                        .expect("Failed to re-create symlink");
                }
                (true, true) => {
                    // if paths already exist in dotfolder there is not need to do anything
                    if path_in_home.canonicalize().unwrap().eq(&path_in_dotfolder) {
                        continue;
                    }
                    // if the paths is duplicated store them in a list for later processing
                    duplicated_paths.push((path_in_home, path_in_dotfolder));
                }
                (false, false) => {
                    println!(
                        "Warning: {} doesn't exist in home or dotfolder, skipping.",
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
            delete(&path.1).expect("Failed to delete the path in the dotfile");
            // copy the new path
            copy_all(&path.0, &path.1).unwrap();
            // deleting the path form the home
            delete(&path.0).expect("Failed to delete the path in the home");
            // create a symlink
            symlink(&path.1, &path.0).expect("Failed to create symlink");
        }

        // processing the unselected paths
        for path in doulicted_paths {
            delete(&path.0).expect("Failed to delete the path in the home");
            symlink(&path.1, &path.0).expect("Failed to create symlink");
        }
    }

    pub fn delink_all(&self) {
        for path in &self.config.paths {
            // Expand ~ or $HOME to absolute path
            let path_in_home = expand_path(path).expect("Failed to expand path");

            // dbg!(&path_in_home,!path_in_home.is_symlink());
            // Skip if not a symlink
            if !path_in_home.is_symlink() {
                println!("continue isn't symlink {:?}", &path_in_home);
                continue;
            }
            // Get relative path inside home directory
            let relative = path_in_home
                .strip_prefix(&self.home_dir)
                .expect("Failed to strip prefix from home dir");

            // Build the full path in the dotfolder
            let path_in_dotfolder = self.dotfolder_path.join(relative);

            // dbg!(&path_in_dotfolder);
            // Remove the symlink in home
            delete(&path_in_home).expect("Failed to delete symlink in home");

            // Copy original file/dir from dotfolder back to home
            copy_all(&path_in_dotfolder, &path_in_home)
                .expect("Failed to copy from dotfolder to home");
        }
    }
}
