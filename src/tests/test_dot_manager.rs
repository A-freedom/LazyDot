#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_imports)]
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
            let (home, _) = get_home_and_dot_path(path);
            let dot =
                get_path_in_dotfolder(&home).expect("failed to get path inside the dotfolder");
            assert!(
                home.canonicalize()
                    .expect("failed to canonicalize")
                    .eq(&dot)
            );
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_resync_with_overwrite_home() {
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
            assert!(
                home.canonicalize()
                    .expect("failed to canonicalize")
                    .eq(&dot)
            );
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_resync_with_overwrite_dotfolder() {
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
            assert!(
                home.canonicalize()
                    .expect("failed to canonicalize")
                    .eq(&dot)
            );
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
    fn test_resync_with_deleted_symlinks() {
        reset_test_environment();
        let (config, manager) = sync_config_with_manager(DuplicateBehavior::Ask);
        for path in &config.paths {
            let (home, dot) = get_home_and_dot_path(path);
            delete(&home);
            assert!(!home.exists());
            assert!(dot.exists());
        }
        manager.delink_all();

        for path in &config.paths {
            let (home, dot) = get_home_and_dot_path(path);
            assert!(!home.exists());
            assert!(dot.exists());
        }
        manager.sync();
        for path in &config.paths {
            let (home, dot) = get_home_and_dot_path(path);
            assert!(
                home.canonicalize()
                    .expect("failed to canonicalize")
                    .eq(&dot)
            );
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_resync_with_existing_broken_symlinks() {
        reset_test_environment();
        let (mut config, _) = sync_config_with_manager(DuplicateBehavior::Ask);
        let dotfolder_path =
            PathBuf::from(expand_path(&config.dotfolder_path).expect("failed to expand dotfolder"));
        let secondary_dotfolder_path =
            dotfolder_path.join(expand_path("~/secondary").expect("failed to expand secondry"));
        copy_all(&dotfolder_path, &secondary_dotfolder_path).expect("failed to copy secondary");
        delete(&dotfolder_path);
        assert!(!dotfolder_path.exists());
        assert!(secondary_dotfolder_path.exists());

        config.dotfolder_path = String::from("~/secondary");
        config.save();

        let manager = DotManager::new();
        manager.sync();

        for path in &config.paths {
            let (home, dot) = get_home_and_dot_path(path);
            assert!(
                home.canonicalize()
                    .expect("failed to canonicalize")
                    .eq(&dot)
            );
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_resync_with_existing_symlinks() {
        reset_test_environment();
        let (mut config, _) = sync_config_with_manager(DuplicateBehavior::OverwriteHome);
        let dotfolder_path =
            PathBuf::from(expand_path(&config.dotfolder_path).expect("failed to expand dotfolder"));
        let secondary_dotfolder_path =
            dotfolder_path.join(expand_path("~/secondary").expect("failed to expand secondry"));
        copy_all(&dotfolder_path, &secondary_dotfolder_path).expect("failed to copy secondary");

        assert!(dotfolder_path.exists());
        assert!(secondary_dotfolder_path.exists());

        config.dotfolder_path = String::from("~/secondary");
        config.save();

        let manager = DotManager::new();
        manager.sync();

        for path in &config.paths {
            let (home, dot) = get_home_and_dot_path(path);
            assert!(
                home.canonicalize()
                    .expect("failed to canonicalize")
                    .eq(&dot)
            );
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_multiple_sync_and_delink_cycles() {
        reset_test_environment();
        let (config, manager) = sync_config_with_manager(DuplicateBehavior::OverwriteHome);
        manager.delink_all();
        for _ in 0..4 {
            manager.sync();
            for path in &config.paths {
                // duplicating the paths
                let (home, dot) = get_home_and_dot_path(path);
                assert!(
                    home.canonicalize()
                        .expect("failed to canonicalize")
                        .eq(&dot)
                );
            }
            manager.delink_all();
            for path in &config.paths {
                let (home, dot) = get_home_and_dot_path(path);
                assert!(home.exists() && !home.is_symlink());
                assert!(dot.exists() && !dot.is_symlink());
            }
        }
    }
}
