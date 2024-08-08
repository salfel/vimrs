use ratatui::{
    prelude::{Buffer, Rect},
    style::Color,
    widgets::Widget,
};

use crate::buffer::Cursor;

impl Widget for Cursor {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        let cell = buf.get_mut(self.col as u16, self.row as u16);
        cell.set_bg(Color::Black);
    }
}
