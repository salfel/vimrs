use std::{fs, io};

pub fn read_file(filename: &str) -> io::Result<String> {
    if filename.is_empty() {
        Ok(String::new())
    } else {
        fs::read_to_string(filename)
    }
}

pub fn write_file(filename: &str, contents: &str) -> io::Result<()> {
    fs::write(filename, contents)
}
