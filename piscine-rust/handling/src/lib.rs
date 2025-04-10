use std::fs::{OpenOptions};
use std::path::Path;
use std::io::{Write};

/*
Create a function open_or_create with two arguments:

1. file, which represents a file path.
2. content, which represents the content to be written to the file.

This function should try to open the file. If it does not exist, the file should be created.
The content should be appended to the file. This means it shouldn't replace whatever content is inside, but append to it.
Should anything go wrong, the function should panic.
*/
pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file = OpenOptions::new().append(true).read(true).write(true).create(true).open(path);
    let buf : Vec<u8> = content.to_owned().into_bytes();
    file.unwrap().write(&buf).unwrap();
}