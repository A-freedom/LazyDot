mod test {
    use crate::config::DuplicateBehavior;
    use crate::dot_manager::DotManager;
    use crate::utils::{
        copy_all, delete, expand_path, get_home_and_dot_path, get_home_dir_string,
        get_path_in_dotfolder, init_config_with_paths, mock_dotfile_paths, reset_test_environment,
        sync_config_with_manager,
    };
    use std::fs;
    use std::path::PathBuf;

    fn read_file(path: &std::path::Path) -> String {
        fs::read_to_string(path).expect("Failed to read file")
    }

    fn is_symlink(path: &str) -> bool {
        expand_path(path).unwrap().is_symlink()
    }

    fn assert_is_symlink(path: &str) {
        assert!(is_symlink(path), "Expected symlink: {}", path);
    }

    fn assert_not_symlink(path: &str) {
        assert!(!is_symlink(path), "Expected not a symlink: {}", path);
    }

    #[test]
    #[serial_test::serial]
    fn test_add_invalid_paths() {
        reset_test_environment();
        let mut config = init_config_with_paths();

        let invalids = vec![
            "~/some_path",
            ".absolute_path",
            "~/nested/path/config.csv",
            "null",
        ];
        for path in invalids {
            let err = config.add_path(path.to_string()).unwrap_err();
            assert!(err.contains("Path does not exist"), "Error: {}", err);
        }

        let home_path = get_home_dir_string();
        for path in vec!["~/", "", &home_path] {
            let err = config.add_path(path.to_string()).unwrap_err();
            assert!(
                err.contains("home"),
                "Expected home path error, got: {}",
                err
            );
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_path_normalization() {
        reset_test_environment();
        let config = init_config_with_paths();
        assert_eq!(
            config.paths,
            vec![
                "~/.bashrc",
                "~/.config/app1",
                "~/.config/app2/app_config2.toml"
            ]
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_sync_with_default_behavior() {
        reset_test_environment();
        let (config, _) = sync_config_with_manager(DuplicateBehavior::Ask);
        for path in &config.paths {
            let home = expand_path(path).unwrap();
            let dot = get_path_in_dotfolder(&home).unwrap();
            assert!(home.is_symlink());
            assert!(dot.exists());
            let (home, _) = get_home_and_dot_path(path);
            let dot =
                get_path_in_dotfolder(&home).expect("failed to get path inside the dotfolder");
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_sync_with_overwrite_home() {
        reset_test_environment();
        let (config, manager) = sync_config_with_manager(DuplicateBehavior::OverwriteHome);

        for path in &config.paths {
            assert_is_symlink(path);
        }

        manager.delink(&config.paths);

        for path in &config.paths {
            let (home, _) = get_home_and_dot_path(path);
            if home.is_dir() {
                continue;
            }
            let dot =
                get_path_in_dotfolder(&home).expect("failed to get path inside the dotfolder");
            fs::write(&dot, "old dotfile").expect("failed to write to file");
            fs::write(&home, "old home").expect("failed to write to file");
        }

        manager.sync();

        for path in &config.paths {
            let (home, _) = get_home_and_dot_path(path);
            if home.is_dir() {
                continue;
            }
            let dot =
                get_path_in_dotfolder(&home).expect("failed to get path inside the dotfolder");
            assert_eq!(read_file(&home), "old dotfile");
            assert_eq!(read_file(&dot), "old dotfile");
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_sync_with_overwrite_dotfolder() {
        reset_test_environment();
        let (config, manager) = sync_config_with_manager(DuplicateBehavior::OverwriteDotfile);

        for path in &config.paths {
            assert_is_symlink(path);
        }

        manager.delink(&config.paths);

        for path in &config.paths {
            let (home, dot) = get_home_and_dot_path(path);
            if dot.is_dir() {
                continue;
            }
            fs::write(&home, "old home").expect("failed to write to file");
            fs::write(&dot, "old dotfile").expect("failed to write to file");
        }

        manager.sync();

        for path in &config.paths {
            let (home, dot) = get_home_and_dot_path(path);
            if home.is_dir() {
                continue;
            }
            assert_eq!(read_file(&home), "old home");
            assert_eq!(read_file(&dot), "old home");
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_sync_with_skip() {
        reset_test_environment();
        let (config, manager) = sync_config_with_manager(DuplicateBehavior::Skip);

        for path in &config.paths {
            assert_is_symlink(path);
        }

        manager.delink(&config.paths);

        for path in &config.paths {
            let (home, dot) = get_home_and_dot_path(path);
            if home.is_dir() {
                continue;
            }
            fs::write(&home, "old home").unwrap();
            fs::write(&dot, "old dotfile").unwrap();
        }

        manager.sync();

        for path in &config.paths {
            let (home, dot) = get_home_and_dot_path(path);
            if home.is_dir() {
                continue;
            }
            assert_eq!(read_file(&home), "old home");
            assert_eq!(read_file(&dot), "old dotfile");
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_delink_removes_symlinks() {
        reset_test_environment();
        let (_, manager) = sync_config_with_manager(DuplicateBehavior::Ask);
        let paths = mock_dotfile_paths();

        for path in &paths {
            assert_is_symlink(path);
            manager.delink(&[path.clone()].to_vec());
            assert_not_symlink(path);
        }
    }
    #[test]
    #[serial_test::serial]
    fn test_restoring_symlinks() {
        reset_test_environment();
        let (config, _) = sync_config_with_manager(DuplicateBehavior::Ask);
        for path in &config.paths {
            let home = expand_path(path).expect("failed to expand path");
            delete(&home).expect("failed to delete old symlink");
        }
        let manager = DotManager::new();
        manager.sync();
        for path in &config.paths {
            let home = expand_path(path).expect("failed to expand path");
            assert!(home.is_symlink());
        }
    }
}
