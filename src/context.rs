use ratatui::crossterm::event::KeyEvent;

use crate::mode::Mode;

pub struct Context {
    pub content: Vec<String>,
    pub cursor: Position,
    pub keys: String,
    pub mode: Mode,
}

impl Context {
    pub fn new(content: String) -> Self {
        let content = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(String::from).collect::<Vec<String>>()
        };

        Context {
            content,
            cursor: Position::default(),
            keys: String::new(),
            mode: Mode::Normal,
        }
    }

    pub fn handle_keys(&mut self, event: KeyEvent) {
        let mode = self.mode;
        mode.handle_keys(self, event);
    }
}

#[derive(Default, Clone, Copy)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}
