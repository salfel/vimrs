use ratatui::prelude::Rect;

use crate::{
    cursor::{Cursor, Position},
    state::State,
};

use super::TextObject;

pub struct Word {
    row: usize,
    start: usize,
    end: usize,
}

impl Word {
    pub fn new(buffer_state: State) -> Self {
        let state = (*buffer_state).borrow();
        let curr_row = &state.content.data[state.cursor.row];

        let chars: Vec<char> = curr_row.chars().collect();
        let start = Self::_get_start(&state.cursor, &chars);
        let end = Self::_get_end(&state.cursor, &chars);
        let row = state.cursor.row;

        Word { start, end, row }
    }

    fn _get_start(cursor: &Cursor, chars: &[char]) -> usize {
        let mut idx = 0;
        for (i, char) in chars.iter().enumerate().take(cursor.col) {
            if *char == ' ' {
                idx = i + 1;
            }
        }

        idx
    }

    fn _get_end(cursor: &Cursor, chars: &[char]) -> usize {
        for (idx, char) in chars.iter().enumerate().skip(cursor.col) {
            if *char == ' ' {
                return idx - 1;
            }
        }

        chars.len() - 1
    }
}

impl TextObject for Word {
    fn start(&self) -> Position {
        Position {
            x: self.start,
            y: self.row,
        }
    }

    fn end(&self) -> Position {
        Position {
            x: self.end,
            y: self.row,
        }
    }

    fn area(&self) -> Rect {
        Rect {
            x: self.start as u16,
            y: self.row as u16,
            width: (self.end - self.start) as u16,
            height: 1,
        }
    }
}
