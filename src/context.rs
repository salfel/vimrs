use std::borrow::BorrowMut;

use ratatui::crossterm::event::KeyEvent;

use crate::mode::Mode;

pub struct Context {
    pub content: Vec<String>,
    pub cursor: Position,
    pub keys: String,
    pub mode: Mode,
    pub exit: bool,
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
            exit: false,
        }
    }

    pub fn handle_keys(&mut self, event: KeyEvent) {
        let mode = self.mode;
        mode.handle_keys(self, event);
    }

    pub fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.keys = String::new();
    }

    pub fn row(&self, row: usize) -> &String {
        &self
            .content
            .get(row)
            .unwrap_or_else(|| panic!("row: {} doesn't exist", row))
    }

    pub fn row_mut(&mut self, row: usize) -> &mut String {
        self.content
            .get_mut(row)
            .unwrap_or_else(|| panic!("row: {} doesn't exist", row))
            .borrow_mut()
    }
}

#[derive(Default, Clone, Copy)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}
