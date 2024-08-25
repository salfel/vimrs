use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ratatui::{
    buffer::Buffer as TBuffer,
    crossterm::event::KeyEvent,
    style::{Color, Style},
    text::Span,
};

use crate::{
    filesystem::{read_file, write_file},
    mode::Mode,
};

#[cfg(test)]
use crate::test::Event;

pub type Registers = Rc<RefCell<HashMap<char, String>>>;

#[allow(dead_code)]
pub struct Buffer {
    pub filename: String,
    pub content: Vec<String>,
    pub cursor: Position,
    pub keys: String,
    pub mode: Mode,
    pub exit: bool,
    pub registers: Registers,
    message: Output,
    #[cfg(test)]
    pub events: Vec<Event>,
    #[cfg(test)]
    pub write: bool,
}

impl Buffer {
    pub fn new(filename: String, registers: &Registers) -> Self {
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
            registers: Rc::clone(registers),
            message,
            #[cfg(test)]
            events: Vec::new(),
            #[cfg(test)]
            write: false,
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
            registers: Rc::new(RefCell::new(HashMap::new())),
            message: Output::default(),
            #[cfg(test)]
            events: Vec::new(),
            #[cfg(test)]
            write: false,
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
        let cell = buf.get_mut(self.cursor.col as u16, self.cursor.row as u16);
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
