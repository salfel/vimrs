use ratatui::{style::Color, Frame};

#[derive(Default, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    pub fn render(&self, frame: &mut Frame) {
        let cell = frame.buffer_mut().get_mut(self.col as u16, self.row as u16);
        cell.set_bg(Color::Black);
    }

    pub fn move_to(&mut self, position: Position) {
        self.col = position.x;
        self.row = position.y;
    }
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}
