extern crate fs_utils;
extern crate tempfile;

mod utils {
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    pub fn tmp_write(content: &[u8]) -> NamedTempFile {
        let tmp_file = NamedTempFile::new().unwrap();
        fs::File::create(tmp_file.path())
            .unwrap()
            .write_all(content)
            .unwrap();
        tmp_file
    }
}

mod read {
    use super::utils::tmp_write;
    use fs_utils::read;

    #[test]
    fn it_heads_bytes_from_a_file_smaller_than_the_limit() {
        let content: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255];
        let tmp_file = tmp_write(&content);
        assert_eq!(read::head(tmp_file.path(), 100).unwrap(), content);
    }

    #[test]
    fn it_heads_bytes_from_a_file_greater_than_the_limit() {
        let content: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255];
        let tmp_file = tmp_write(&content);
        assert_eq!(read::head(tmp_file.path(), 10).unwrap(), &content[..10]);
    }

    #[test]
    fn it_heads_bytes_from_a_file_of_exact_same_size_as_the_limit() {
        let content: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255];
        let tmp_file = tmp_write(&content);
        assert_eq!(read::head(tmp_file.path(), 11).unwrap(), content);
    }

    #[test]
    fn it_heads_to_string() {
        let content = "Hello World! Привіт Світ! こんにちは世界";
        let tmp_file = tmp_write(content.as_bytes());
        assert_eq!(read::head_to_string(tmp_file.path(), 100).unwrap(), content);
    }

    #[test]
    fn it_heads_and_lossy_handles_invalid_to_string() {
        let content: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255];
        let tmp_file = tmp_write(&content);
        assert_eq!(
            read::head_to_string(tmp_file.path(), 100).unwrap(),
            "\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t�"
        );
    }

    #[test]
    fn it_heads_to_string_and_puts_truncate_message() {
        let content = "Hello World! Привіт Світ! こんにちは世界";
        let tmp_file = tmp_write(content.as_bytes());
        assert_eq!(
            read::head_to_string_with_message(tmp_file.path(), 28, "...").unwrap(),
            "Hello World! Привіт..."
        );
    }
}
