use std::fs::{self, File};
use std::io::{Result, Write};

pub struct FileSystem {}

impl FileSystem {
    pub fn read_file(path: String) -> Result<String> {
        fs::read_to_string(path)
    }

    pub fn write_to_file(path: String, content: String) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
