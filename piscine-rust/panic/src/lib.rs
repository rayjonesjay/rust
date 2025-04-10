use std::fs::File;
use std::path::Path;

pub fn open_file(s: &str) -> File {
    File::open(Path::new(s)).unwrap()
}