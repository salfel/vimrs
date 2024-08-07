use std::{fs, io};

pub struct Buffer {
    name: String,
    pub content: String,
    dirty: bool,
}

impl Buffer {
    pub fn new(name: String, errors: &mut Vec<String>) -> Self {
        match Self::get_file_contents(&name) {
            Ok(content) => Buffer {
                name,
                content,
                dirty: false,
            },
            Err(msg) => {
                errors.push(msg.to_string());

                Buffer {
                    name: String::new(),
                    content: String::new(),
                    dirty: false,
                }
            }
        }
    }

    fn get_file_contents(path: &str) -> io::Result<String> {
        if !path.is_empty() {
            fs::read_to_string(path)
        } else {
            Ok(String::new())
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
