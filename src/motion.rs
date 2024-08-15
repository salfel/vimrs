use crate::{
    buffer::{Buffer, Position},
    navigation::{
        down, end_line, left, prev_word_start, right, start_line, up, word_end, word_start,
    },
};

#[cfg(test)]
use crate::test::Event;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Motion {
    Left,
    Right,
    Up,
    Down,
    LineStart,
    LineEnd,
    PrevWordStart,
    StartWord,
    WordEnd,
}

impl Motion {
    pub fn new(key: &str) -> Option<Self> {
        match key {
            "h" => Some(Motion::Left),
            "j" => Some(Motion::Down),
            "k" => Some(Motion::Up),
            "l" => Some(Motion::Right),
            "^" => Some(Motion::LineStart),
            "$" => Some(Motion::LineEnd),
            "b" => Some(Motion::PrevWordStart),
            "e" => Some(Motion::WordEnd),
            "w" => Some(Motion::StartWord),
            _ => None,
        }
    }

    pub fn execute(self, buf: &mut Buffer) -> Position {
        #[cfg(test)]
        buf.events.push(Event::Motion(self));

        match self {
            Motion::Left => left(buf),
            Motion::Down => down(buf),
            Motion::Up => up(buf),
            Motion::Right => right(buf),
            Motion::LineStart => start_line(buf),
            Motion::LineEnd => end_line(buf),
            Motion::PrevWordStart => prev_word_start(buf),
            Motion::WordEnd => word_end(buf),
            Motion::StartWord => word_start(buf),
        }
    }
}
