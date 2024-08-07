use std::{fs, io};

use crate::mode::{
    EditorMode, Mode,
    ModeType::{Exit, Normal},
};

pub struct Buffer {
    name: String,
    pub content: String,
    dirty: bool,
    pub mode: Mode,
}

impl Buffer {
    pub fn new(name: String, errors: &mut Vec<String>) -> Self {
        match Self::get_file_contents(&name) {
            Ok(content) => Buffer {
                name,
                content,
                dirty: false,
                mode: Mode::new(Normal),
            },
            Err(msg) => {
                errors.push(msg.to_string());

                Buffer {
                    name: String::new(),
                    content: String::new(),
                    dirty: false,
                    mode: Mode::new(Normal),
                }
            }
        }
    }

    pub fn change_mode(&mut self) {
        if self.mode.should_change() && !self.should_exit() {
            self.mode = Mode::new(self.mode.new_type());
        }
    }

    pub fn should_exit(&self) -> bool {
        self.mode.new_type() == Exit
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn get_file_contents(path: &str) -> io::Result<String> {
        if !path.is_empty() {
            fs::read_to_string(path)
        } else {
            Ok(String::new())
        }
    }
}
