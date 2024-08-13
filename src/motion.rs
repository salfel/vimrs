use crate::{
    context::{Context, Position},
    navigation::{down, end_line, end_word, left, right, start_line, start_word, up},
};

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

    pub fn execute(self, cx: &mut Context) -> Position {
        match self {
            Motion::Left => left(cx),
            Motion::Down => down(cx),
            Motion::Up => up(cx),
            Motion::Right => right(cx),
            Motion::Start => start_line(cx),
            Motion::End => end_line(cx),
            Motion::StartWord => start_word(cx),
            Motion::EndWord => end_word(cx),
        }
    }
}
