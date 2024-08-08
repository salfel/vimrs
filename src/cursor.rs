use ratatui::{style::Color, Frame};

#[derive(Default, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    pub fn render(&self, frame: &mut Frame, offset: i16) {
        let cell = frame
            .buffer_mut()
            .get_mut((self.col as i16 + offset) as u16, self.row as u16);
        cell.set_bg(Color::Black);
    }
}
