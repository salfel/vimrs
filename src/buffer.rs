use ratatui::crossterm::event::KeyEvent;

use crate::{context::Context, mode::Mode};

pub struct Buffer {
    pub filename: String,
    pub context: Context,
}

impl Buffer {
    pub fn new(filename: String, content: String) -> Self {
        Buffer {
            filename,
            context: Context::new(content),
        }
    }

    pub fn get_content(&self) -> String {
        self.context.content.join("\n")
    }

    pub fn handle_keys(&mut self, event: KeyEvent) {
        self.context.handle_keys(event);
    }
}
