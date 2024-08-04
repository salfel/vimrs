use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

pub mod command;
pub mod insert;
pub mod normal;

pub trait Mode {
    fn mode(&mut self) -> Option<Box<dyn Mode>>;

    fn handle_key(&mut self, event: KeyEvent);

    fn label(&self) -> String;

    fn render(&mut self, frame: &mut Frame, rect: Rect);
}
