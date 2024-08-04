use ratatui::crossterm::event::KeyEvent;

pub mod insert;
pub mod normal;

pub trait Mode {
    fn mode(&mut self) -> Option<Box<dyn Mode>>;

    fn handle_key(&mut self, event: KeyEvent);

    fn label(&self) -> String;
}
