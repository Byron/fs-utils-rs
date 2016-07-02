extern crate fs_utils;
extern crate tempdir;

mod copy_directory {
    use tempdir::TempDir;
    use std::path::{Path, PathBuf};
    use fs_utils::{destination_dir, copy_directory};

    fn fixture_at(name: &str) -> PathBuf {
        Path::new(file!()).parent().map(|p| p.join("fixtures").join(name)).unwrap()
    }

    #[test]
    fn it_copies_the_content_of_the_entire_directory_recursively_and_respects_basic_permissions
                                                                                                () {
        let (dest, source) = (TempDir::new("dest").unwrap(), fixture_at("source-1"));
        let (dest_path, copy_dest) = (dest.path(), destination_dir(source.as_ref(), dest.path()));

        assert_eq!(copy_directory(&source, dest_path).unwrap(), copy_dest);
    }
}

mod destination_dir {
    use fs_utils::destination_dir;
    use std::path::PathBuf;
    use std::env::current_dir;

    #[test]
    fn it_always_appends_the_filename_to_destination() {
        assert_eq!(destination_dir("source/subdir", "dest"),
                   PathBuf::from("dest/subdir"));
    }

    #[test]
    fn it_can_deal_with_the_root_directory() {
        assert_eq!(destination_dir("/", "dest"), PathBuf::from("dest/ROOT"))
    }

    #[test]
    fn it_can_work_with_absolute_source_paths() {
        assert_eq!(destination_dir("/hello/there", "dest"),
                   PathBuf::from("dest/there"));
    }

    #[test]
    fn it_can_work_with_absolute_destination_paths() {
        assert_eq!(destination_dir(".", "/hello/dest"),
                   PathBuf::from("/hello/dest/fs-utils-rs"));
    }

    #[test]
    fn it_can_work_with_relative_paths_too() {
        assert_eq!(destination_dir("../", "dest"),
                   PathBuf::from("dest").join(current_dir()
                                                  .unwrap()
                                                  .join("..")
                                                  .canonicalize()
                                                  .unwrap()
                                                  .file_name()
                                                  .unwrap()));
    }

    #[test]
    fn it_can_work_with_relative_paths() {
        assert_eq!(destination_dir(".", "dest"),
                   PathBuf::from("dest").join(current_dir()
                                                  .unwrap()
                                                  .file_name()
                                                  .unwrap()));
    }
}
