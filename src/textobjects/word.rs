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
        let start = Self::get_start(state.cursor.col, &chars);
        let end = Self::get_end(state.cursor.col, &chars);
        let row = state.cursor.row;

        Word { start, end, row }
    }

    fn get_start(col: usize, chars: &[char]) -> usize {
        let mut idx = col;

        if idx != 0 && Self::is_start(idx, chars) {
            idx -= 1;
            while Self::is_empty(idx, chars) {
                idx -= 1;
            }
        }

        let mut prev = ' ';
        while let Some(char) = chars.get(idx) {
            if *char == ' ' && prev != ' ' {
                return idx + 1;
            } else if idx == 0 {
                return 0;
            }

            idx -= 1;
            prev = *char;
        }

        0
    }

    fn get_end(col: usize, chars: &[char]) -> usize {
        let mut idx = col;

        if Self::is_end(idx, chars) {
            idx += 1;
            while Self::is_empty(idx, chars) {
                idx += 1;
            }
        }

        let mut prev = ' ';
        while let Some(char) = chars.get(idx) {
            if *char == ' ' && prev != ' ' {
                return idx - 1;
            }
            idx += 1;
            prev = *char;
        }

        chars.len() - 1
    }

    fn is_start(col: usize, chars: &[char]) -> bool {
        col == 0 || (Self::is_empty(col - 1, chars) && !Self::is_empty(col, chars))
    }

    fn is_end(col: usize, chars: &[char]) -> bool {
        Self::is_empty(col + 1, chars) && !Self::is_empty(col, chars)
    }

    fn is_empty(idx: usize, chars: &[char]) -> bool {
        match chars.get(idx) {
            Some(char) => *char == ' ',
            None => false,
        }
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
