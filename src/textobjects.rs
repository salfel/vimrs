use ratatui::layout::Rect;

use crate::cursor::Position;

pub mod word;

pub trait TextObject {
    fn start(&self) -> Position;

    fn end(&self) -> Position;

    fn area(&self) -> Rect;
}

pub enum TextObjectPosition {
    WordStart,
    WordEnd,
}
