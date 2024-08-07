use std::{cell::RefCell, fs, io, rc::Rc};

use crate::mode::{
    EditorMode, Mode,
    ModeType::{Exit, Normal},
};

pub struct Dirty<T> {
    pub data: T,
    pub dirty: bool,
}

impl<T> Dirty<T> {
    pub fn new(data: T) -> Self {
        Dirty { data, dirty: false }
    }
}

pub struct Buffer {
    name: String,
    pub content: Rc<RefCell<Dirty<String>>>,
    pub mode: Mode,
}

impl Buffer {
    pub fn new(name: String, errors: &mut Vec<String>) -> Self {
        match Self::get_file_contents(&name) {
            Ok(content) => {
                let mut content = Rc::new(RefCell::new(Dirty::new(content)));
                Buffer {
                    name,
                    mode: Mode::new(Normal, Rc::clone(&content)),
                    content,
                }
            }
            Err(msg) => {
                errors.push(msg.to_string());
                let mut content = Rc::new(RefCell::new(Dirty::new(String::new())));

                Buffer {
                    name: String::new(),
                    mode: Mode::new(Normal, Rc::clone(&content)),
                    content,
                }
            }
        }
    }

    pub fn change_mode(&mut self) {
        if self.mode.should_change() && !self.should_exit() {
            self.mode = Mode::new(self.mode.new_type(), Rc::clone(&self.content));
        }
    }

    pub fn should_exit(&self) -> bool {
        self.mode.new_type() == Exit
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    fn get_file_contents(path: &str) -> io::Result<String> {
        if !path.is_empty() {
            fs::read_to_string(path)
        } else {
            Ok(String::new())
        }
    }
}
