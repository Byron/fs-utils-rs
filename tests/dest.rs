extern crate fs_utils;

mod destination_directory {
    use fs_utils::copy::destination_directory;
    use std::path::PathBuf;
    use std::env::set_current_dir;

    #[test]
    #[should_panic]
    /// Changes CWD, and thus needs its own file to be sandboxed
    fn it_can_work_with_relative_paths_but_not_if_that_is_root_as_well() {
        set_current_dir("/").unwrap();
        assert_eq!(destination_directory("", "dest"),
                   PathBuf::from("dest/fs-utils"));
    }

}
