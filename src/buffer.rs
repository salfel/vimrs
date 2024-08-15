use ratatui::{buffer::Buffer as TBuffer, crossterm::event::KeyEvent, style::Color};

use crate::{
    filesystem::{read_file, write_file},
    mode::Mode,
};

#[cfg(test)]
use crate::test::Event;

#[allow(dead_code)]
pub struct Buffer {
    pub filename: String,
    pub content: Vec<String>,
    pub cursor: Position,
    pub keys: String,
    pub mode: Mode,
    pub exit: bool,
    pub print: String,
    #[cfg(test)]
    pub events: Vec<Event>,
}

impl Buffer {
    pub fn new(filename: String) -> Self {
        let file_content = read_file(&filename);
        let content = if file_content.is_empty() {
            vec![String::new()]
        } else {
            file_content
                .lines()
                .map(String::from)
                .collect::<Vec<String>>()
        };

        Buffer {
            filename,
            content,
            cursor: Position::default(),
            keys: String::new(),
            mode: Mode::Normal,
            exit: false,
            print: String::new(),
            #[cfg(test)]
            events: Vec::new(),
        }
    }

    pub fn render_cursor(&self, buf: &mut TBuffer) {
        let cell = buf.get_mut(self.cursor.col as u16, self.cursor.row as u16);
        cell.set_bg(Color::White).set_fg(Color::Black);
    }

    pub fn handle_keys(&mut self, event: KeyEvent) {
        self.mode.clone().handle_keys(self, event);
    }

    pub fn write(&self) {
        write_file(&self.filename, &self.content.join("\n"));
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

    #[cfg(test)]
    pub fn input_text(&mut self, text: &str) {
        use ratatui::crossterm::event::{KeyCode, KeyModifiers};

        for char in text.chars() {
            self.handle_keys(KeyEvent::new(KeyCode::Char(char), KeyModifiers::NONE));
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_file() {
        let filename = String::from("Cargo.toml");
        let buffer = Buffer::new(filename);

        assert!(!buffer.content.is_empty());
    }
}
