use std::fs;

// TODO return result
pub fn read_file(filename: &String) -> String {
    match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => String::new(),
    }
}

// TODO return result
pub fn write_file(filename: &str, contents: &str) {
    let _ = fs::write(filename, contents);
}
