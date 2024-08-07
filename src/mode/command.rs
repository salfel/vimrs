use ratatui::crossterm::event::{KeyCode, KeyEvent};

use super::{EditorMode, ModeType};

pub struct CommandMode {
    mode: ModeType,
    keys: String,
}

impl CommandMode {
    pub fn new() -> Self {
        Self {
            mode: ModeType::Command,
            keys: String::new(),
        }
    }
    fn handle_keybindings(&mut self) {
        match self.keys.as_str() {
            ":" => self.mode = ModeType::Command,
            _ => return,
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
        if let KeyCode::Char(char) = event.code {
            self.keys.push(char);
        }

        self.handle_keybindings();
    }
}
