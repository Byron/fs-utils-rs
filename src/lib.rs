//! Utilities to help working with the file-system.
use std::path::{Path, PathBuf};
use std::io;
use std::env::current_dir;

/// Return the computed destination directory, given a source directory.
pub fn destination_dir<P: AsRef<Path>>(source_dir: P, destination_dir: P) -> PathBuf {
    let (cwd, source_dir) = (current_dir().unwrap(), source_dir.as_ref());
    destination_dir.as_ref()
                   .join(if source_dir.is_relative() {
                       source_dir.file_name()
                                 .unwrap_or(&cwd.file_name().expect("to not be at the root"))
                   } else {
                       source_dir.file_name().unwrap_or("ROOT".as_ref())
                   })
}

pub fn copy_directory(source_dir: &Path, destination_dir: &Path) -> Result<PathBuf, io::Error> {
    let dest = ::destination_dir(source_dir, destination_dir);
    Ok(dest)
}
