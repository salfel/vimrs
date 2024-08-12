use ratatui::{crossterm::event::KeyEvent, style::Color, Frame};

use crate::{context::Context, mode::Mode};

#[allow(dead_code)]
pub struct Buffer {
    pub filename: String,
    context: Context,
}

impl Buffer {
    pub fn new(filename: String, content: String) -> Self {
        Buffer {
            filename,
            context: Context::new(content),
        }
    }

    pub fn render_cursor(&self, frame: &mut Frame) {
        let cell = frame.buffer_mut().get_mut(
            self.context.cursor.col as u16,
            self.context.cursor.row as u16,
        );
        cell.set_bg(Color::White).set_fg(Color::Black);
    }

    pub fn get_content(&self) -> String {
        self.context.content.join("\n")
    }

    pub fn get_mode(&self) -> Mode {
        self.context.mode
    }

    pub fn handle_keys(&mut self, event: KeyEvent) {
        self.context.handle_keys(event);
    }
}
