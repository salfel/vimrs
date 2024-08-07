use std::{cell::RefCell, rc::Rc};

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::buffer::Dirty;

use super::{
    EditorMode,
    ModeType::{self, Exit, Normal},
};

pub struct CommandMode {
    mode: ModeType,
    keys: String,
    content: Rc<RefCell<Dirty<String>>>,
}

impl CommandMode {
    pub fn new(content: Rc<RefCell<Dirty<String>>>) -> Self {
        Self {
            mode: ModeType::Command,
            keys: String::new(),
            content,
        }
    }

    fn execute_command(&mut self) {
        match self.keys.as_str() {
            "q" => self.mode = Exit,
            _ => {}
        }

        self.keys = String::new();
    }
}

impl EditorMode for CommandMode {
    fn label(&self) -> String {
        String::from("Command")
    }

    fn new_type(&self) -> ModeType {
        self.mode
    }

    fn handle_events(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(char) => self.keys.push(char),
            KeyCode::Esc => self.mode = Normal,
            KeyCode::Enter => self.execute_command(),
            _ => {}
        }
    }
}
