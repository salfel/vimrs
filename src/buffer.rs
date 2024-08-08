use std::{cell::RefCell, fs, io, rc::Rc};

use ratatui::{layout::Rect, Frame};

use crate::{
    mode::{
        EditorMode, Mode,
        ModeType::{Exit, Normal},
    },
    state::{BufferState, State},
};

pub struct Buffer {
    name: String,
    pub state: State,
    pub mode: Mode,
}

impl Buffer {
    pub fn new(name: String, errors: &mut Vec<String>) -> Self {
        match Self::get_file_contents(&name) {
            Ok(content) => {
                let content = Rc::new(RefCell::new(BufferState::new(content)));
                Buffer {
                    name,
                    mode: Mode::new(Normal, Rc::clone(&content)),
                    state: content,
                }
            }
            Err(msg) => {
                errors.push(msg.to_string());
                let content = Rc::new(RefCell::new(BufferState::new(String::new())));

                Buffer {
                    name: String::new(),
                    mode: Mode::new(Normal, Rc::clone(&content)),
                    state: content,
                }
            }
        }
    }

    pub fn change_mode(&mut self) {
        if self.mode.should_change() && !self.should_exit() {
            self.mode = Mode::new(self.mode.new_type(), Rc::clone(&self.state));
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.mode.render(frame, area);
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
