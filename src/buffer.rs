use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ratatui::{
    buffer::Buffer as TBuffer,
    crossterm::event::KeyEvent,
    layout::Position as TuiPosition,
    style::{Color, Style},
    text::Span,
};

use crate::{
    filesystem::{read_file, write_file},
    mode::Mode,
};

pub struct Buffer {
    pub filename: String,
    pub content: Vec<String>,
    pub cursor: Position,
    pub keys: String,
    pub mode: Mode,
    pub exit: bool,
    pub register: Register,
    message: Output,
}

impl Buffer {
    pub fn new(filename: String, register: &Register) -> Self {
        let (content, error) = Self::get_content(&filename);

        let message = match error {
            Some(message) => Output {
                message,
                error: true,
            },
            None => Output::default(),
        };

        Buffer {
            filename,
            content,
            cursor: Position::default(),
            keys: String::new(),
            mode: Mode::Normal,
            exit: false,
            register: Register::clone(register),
            message,
        }
    }

    #[cfg(test)]
    pub fn test(filename: String) -> Self {
        let content = Self::get_content(&filename).0;

        Buffer {
            filename,
            content,
            cursor: Position::default(),
            keys: String::new(),
            mode: Mode::Normal,
            exit: false,
            register: Register::new(),
            message: Output::default(),
        }
    }

    fn get_content(filename: &str) -> (Vec<String>, Option<String>) {
        match read_file(filename) {
            Ok(content) => {
                let content = if content.is_empty() {
                    vec![String::new()]
                } else {
                    content.lines().map(String::from).collect()
                };

                (content, None)
            }
            Err(msg) => (vec![String::new()], Some(msg.to_string())),
        }
    }

    pub fn render_cursor(&self, buf: &mut TBuffer) {
        let cell = buf.cell_mut(self.cursor).unwrap();
        cell.set_bg(Color::White).set_fg(Color::Black);
    }

    pub fn handle_keys(&mut self, event: KeyEvent) {
        self.mode.clone().handle_keys(self, event);
    }

    pub fn write(&mut self) {
        if let Err(msg) = write_file(&self.filename, &self.content.join("\n")) {
            self.message.message = msg.to_string();
            self.message.error = true;
        }
    }

    pub fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.keys = String::new();
    }

    pub fn row(&self, row: usize) -> &String {
        self.content
            .get(row)
            .unwrap_or_else(|| panic!("row: {} doesn't exist", row))
    }

    pub fn message(&self) -> Span {
        if self.message.error {
            Span::styled(&self.message.message, Style::default().fg(Color::Red))
        } else {
            Span::raw(&self.message.message)
        }
    }

    #[cfg(test)]
    pub fn input_keys(&mut self, text: &str) {
        use ratatui::crossterm::event::{KeyCode, KeyModifiers};

        for char in text.chars() {
            self.handle_keys(KeyEvent::new(KeyCode::Char(char), KeyModifiers::NONE));
        }
    }
}

#[derive(Default)]
struct Output {
    message: String,
    error: bool,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl From<Position> for TuiPosition {
    fn from(value: Position) -> Self {
        TuiPosition {
            x: value.col as u16,
            y: value.row as u16,
        }
    }
}

pub struct Register(Rc<RefCell<HashMap<char, String>>>);

#[allow(dead_code)]
impl Register {
    pub fn new() -> Self {
        Register(Rc::new(RefCell::new(HashMap::new())))
    }

    pub fn clone(&self) -> Register {
        Register(Rc::clone(&self.0))
    }

    pub fn get(&self, char: char) -> String {
        match (*self.0).borrow().get(&char) {
            Some(value) => value.to_string(),
            None => String::new(),
        }
    }

    pub fn set(&self, char: char, value: String) -> Option<String> {
        (*self.0).borrow_mut().insert(char, value)
    }

    pub fn get_default(&self) -> String {
        self.get('*')
    }

    pub fn set_default(&self, value: String) -> Option<String> {
        self.set('*', value)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn read_file_test() {
        let filename = String::from("Cargo.toml");
        let buffer = Buffer::test(filename);

        assert!(!buffer.content.is_empty());
    }

    #[test]
    fn write_file_test() {
        let filename = String::from("test");
        let mut buf = Buffer::test(filename.clone());
        buf.content[0] = String::from("test");
        buf.write();

        assert_eq!(read_file(&filename).unwrap(), "test");
        fs::remove_file(filename).unwrap();
    }
}
