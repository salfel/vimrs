use crate::{
    buffer::{Buffer, Position},
    navigation::{
        down, end_line, find_char, find_prev_char, left, prev_word_start, right, start_line, up,
        word_end, word_start,
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
    Find { char: char },
    FindPrev { char: char },
}

impl Motion {
    pub fn new(keys: &str) -> Option<Self> {
        match keys {
            "h" => Some(Motion::Left),
            "j" => Some(Motion::Down),
            "k" => Some(Motion::Up),
            "l" => Some(Motion::Right),
            "^" => Some(Motion::LineStart),
            "$" => Some(Motion::LineEnd),
            "b" => Some(Motion::PrevWordStart),
            "e" => Some(Motion::WordEnd),
            "w" => Some(Motion::StartWord),
            keys if keys.len() >= 2 => Self::two_char_motions(keys),
            _ => None,
        }
    }

    fn two_char_motions(keys: &str) -> Option<Self> {
        let chars: Vec<char> = keys.chars().collect();
        match (chars[0], chars[1]) {
            ('f', char) => Some(Motion::Find { char }),
            ('F', char) => Some(Motion::FindPrev { char }),
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
            Motion::Find { char } => find_char(buf, char),
            Motion::FindPrev { char } => find_prev_char(buf, char),
        }
    }

    pub fn inclusive(&self) -> bool {
        matches!(self, Motion::PrevWordStart | Motion::StartWord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_multi_char_motions() {
        let motion = Motion::new("fa");
        assert_eq!(motion, Some(Motion::Find { char: 'a' }))
    }
}
