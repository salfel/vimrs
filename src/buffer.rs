use ratatui::{crossterm::event::KeyEvent, style::Color, Frame};

use crate::{
    context::Context,
    filesystem::{read_file, write_file},
    mode::Mode,
};

#[allow(dead_code)]
pub struct Buffer {
    pub filename: String,
    context: Context,
}

impl Buffer {
    pub fn new(filename: String) -> Self {
        let content = read_file(&filename);

        Buffer {
            filename,
            context: Context::new(content),
        }
    }

    pub fn run_actions(&mut self) {
        if self.context.write {
            self.write_contents();

            self.context.write = false;
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

    pub fn should_exit(&self) -> bool {
        self.context.exit
    }

    pub fn print(&self) -> String {
        self.context.print.clone()
    }

    fn write_contents(&self) {
        write_file(&self.filename, &self.context.content.join("\n"));
    }
}
