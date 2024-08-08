use std::{cell::RefCell, rc::Rc};

use ratatui::text::Line;

pub type State = Rc<RefCell<BufferState>>;

pub struct BufferState {
    pub content: Dirty<Vec<String>>,
    pub cursor: Cursor,
}

impl BufferState {
    pub fn new(content: String) -> Self {
        let content = Dirty::new(Self::split_content(content));

        BufferState {
            content,
            cursor: Cursor::default(),
        }
    }

    pub fn left(&mut self) {
        if self.cursor.col == 0 {
            if self.cursor.row != 0 {
                let prev_row = &self.content.data[self.cursor.row - 1];
                self.cursor.col = prev_row.len() - 1;
                self.cursor.row -= 1;
            }
        } else {
            self.cursor.col -= 1;
        }
    }

    pub fn right(&mut self) {
        let current_row = &self.content.data[self.cursor.row];
        if self.cursor.col == current_row.len() {
            if self.cursor.row != self.content.data.len() - 1 {
                self.cursor.row += 1;
                self.cursor.col = 0;
            }
        } else {
            self.cursor.col += 1;
        }
    }

    pub fn up(&mut self) {
        if self.cursor.row == 0 {
            return;
        }

        let prev_row = &self.content.data[self.cursor.row - 1];
        self.cursor.row -= 1;

        if self.cursor.col > prev_row.len() {
            self.cursor.col = prev_row.len();
        }
    }

    pub fn down(&mut self) {
        if self.cursor.row == self.content.data.len() - 1 {
            return;
        }

        let next_row = &self.content.data[self.cursor.row + 1];
        self.cursor.row += 1;

        if self.cursor.col > next_row.len() {
            self.cursor.row = next_row.len();
        }
    }

    pub fn end(&mut self) {
        let curr_row = &mut self.content.data[self.cursor.row];

        if !curr_row.is_empty() {
            self.cursor.col = curr_row.len() - 1;
        }
    }

    pub fn start(&mut self) {
        self.cursor.col = 0;
    }

    pub fn new_line(&mut self) {
        let curr_row = &mut self.content.data[self.cursor.row];
        let remaining: String = curr_row.drain(..self.cursor.col).collect();

        self.content.data.insert(self.cursor.row, remaining);
        self.cursor.col = 0;
        self.cursor.row += 1;
    }

    pub fn get_lines_from_content(&self) -> Vec<Line> {
        self.content
            .data
            .iter()
            .map(|line| Line::from(line.as_str()))
            .collect()
    }

    fn split_content(content: String) -> Vec<String> {
        content.split("\n").map(|value| value.to_string()).collect()
    }
}

#[allow(dead_code)]
pub struct Dirty<T> {
    pub data: T,
    pub dirty: bool,
}

impl<T> Dirty<T> {
    pub fn new(data: T) -> Self {
        Dirty { data, dirty: false }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}
