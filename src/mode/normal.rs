use ratatui::crossterm::event::{KeyCode, KeyEvent};

use super::{EditorMode, ModeType};

pub struct NormalMode {
    mode: ModeType,
    keys: String,
}

impl NormalMode {
    pub fn new() -> Self {
        Self {
            mode: ModeType::Normal,
            keys: String::new(),
        }
    }
}

impl NormalMode {
    fn handle_keybindings(&mut self) {
        match self.keys.as_str() {
            ":" => self.mode = ModeType::Command,
            _ => return,
        }

        self.keys = String::new();
    }
}

impl EditorMode for NormalMode {
    fn new_type(&self) -> ModeType {
        self.mode
    }

    fn handle_events(&mut self, event: KeyEvent) {
        if let KeyCode::Char(char) = event.code {
            self.keys.push(char);
        }

        self.handle_keybindings();
    }
}
