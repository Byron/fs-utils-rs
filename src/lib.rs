use std::path::{PathBuf, Path};

fn dest(source: &Path, destination: &Path) -> PathBuf { 
    let base_name = match source.file_name() {
        Some(b) => b.to_owned(),
        None => {
            source.canonicalize().unwrap().file_name().unwrap().to_owned()
        }
    };
    destination.join(base_name)
}

fn cpr(source: &str, destination: &str) -> Result<(), ()> {
    Ok(())
}

#[test]
fn test_dest_with_relative_source_and_absolute_destination() {
    let res = dest(Path::new("."), Path::new("/tmp"));
    assert_eq!(res, Path::new("/"));
}

#[test]
fn test_dest_with_absolute_source_and_absolute_destination() {
    let res = dest(Path::new("/home"), Path::new("/tmp"));
    assert_eq!(res, Path::new("/tmp/home"));
}

// #[test]
// fn it_works() {
//     let source = "source";
//     let destination = ".";
//     let computed_destination = dest(source, destination);
//     let actual_destination = cpr(source, destination).unwrap();
//     assert_eq!(actual_destination, "./source");
// }
