extern crate fs_utils;
extern crate tempdir;

mod utils {
    use std::path::{Path, PathBuf};

    pub fn fixture_at(name: &str) -> PathBuf {
        Path::new(file!())
            .parent()
            .map(|p| p.join("fixtures").join(name))
            .unwrap()
    }
}

mod copy_directory {
    use tempdir::TempDir;
    use fs_utils::copy::{copy_directory, destination_directory};
    use std::path::{Path, PathBuf};
    use super::utils::fixture_at;
    use std::os::unix::fs::PermissionsExt;
    use std::fs;

    #[test]
    fn it_does_not_overwrite_existing_destination_directories() {
        let (dest, source) = (TempDir::new("dest").unwrap(), Path::new("."));
        let existing_dest = destination_directory(&source, &dest.path());
        fs::create_dir(&existing_dest).unwrap();

        assert!(copy_directory(&source, &dest.path()).is_err());
    }

    #[test]
    fn it_copies_the_content_of_the_entire_directory_recursively_and_with_permissions() {
        let (dest, source) = (TempDir::new("dest").unwrap(), fixture_at("source-1"));
        let (dest_path, copy_dest) = (dest.path(), destination_directory(&source, dest.path()));

        let copy_result = copy_directory(&source, dest_path).unwrap();
        assert_eq!(copy_result, copy_dest);
        assert!(copy_result.join("a").is_file());
        assert!(copy_result.join("b").is_file());
        assert!(copy_result.join("c").is_dir());
        assert!(copy_result.join("c").join("a").is_file());
        assert!(copy_result.join("c").join("b").is_file());
        #[cfg(not(windows))]
        fn os_specific(copy_result: &PathBuf) {
            assert_eq!(
                copy_result
                    .join("c")
                    .join("c")
                    .metadata()
                    .unwrap()
                    .permissions()
                    .mode() & 0o111,
                0o111
            );
        }
        #[cfg(windows)]
        fn os_specific(copy_result: &PathBuf) {}

        os_specific(&copy_result)
    }
}

mod destination_directory {
    use fs_utils::copy::destination_directory;
    use std::path::PathBuf;
    use std::env::current_dir;

    #[test]
    fn it_always_appends_the_filename_to_destination() {
        assert_eq!(
            destination_directory("source/subdir", "dest"),
            PathBuf::from("dest/subdir")
        );
    }

    #[test]
    fn it_can_deal_with_the_root_directory() {
        assert_eq!(
            destination_directory("/", "dest"),
            PathBuf::from("dest/ROOT")
        )
    }

    #[test]
    fn it_can_work_with_absolute_source_paths() {
        assert_eq!(
            destination_directory("/hello/there", "dest"),
            PathBuf::from("dest/there")
        );
    }

    #[test]
    fn it_can_work_with_absolute_destination_paths() {
        assert_eq!(
            destination_directory(".", "/hello/dest"),
            PathBuf::from("/hello/dest").join(current_dir().unwrap().file_name().unwrap())
        );
    }

    #[test]
    fn it_can_work_with_relative_paths_too() {
        assert_eq!(
            destination_directory("../", "dest"),
            PathBuf::from("dest").join(
                current_dir()
                    .unwrap()
                    .join("..")
                    .canonicalize()
                    .unwrap()
                    .file_name()
                    .unwrap()
            )
        );
    }

    #[test]
    fn it_can_work_with_relative_paths() {
        assert_eq!(
            destination_directory(".", "dest"),
            PathBuf::from("dest").join(current_dir().unwrap().file_name().unwrap())
        );
    }
}
