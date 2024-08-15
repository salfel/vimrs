use crate::{
    buffer::{Buffer, Position},
    navigation::{down, end_line, end_word, left, right, start_line, start_word, up},
};

#[cfg(test)]
use crate::test::Event;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Motion {
    Left,
    Right,
    Up,
    Down,
    Start,
    End,
    StartWord,
    EndWord,
}

impl Motion {
    pub fn new(key: &str) -> Option<Self> {
        match key {
            "h" => Some(Motion::Left),
            "j" => Some(Motion::Down),
            "k" => Some(Motion::Up),
            "l" => Some(Motion::Right),
            "^" => Some(Motion::Start),
            "$" => Some(Motion::End),
            "b" => Some(Motion::StartWord),
            "e" => Some(Motion::EndWord),
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
            Motion::Start => start_line(buf),
            Motion::End => end_line(buf),
            Motion::StartWord => start_word(buf),
            Motion::EndWord => end_word(buf),
        }
    }
}
