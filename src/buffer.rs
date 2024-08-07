use std::{cell::RefCell, fs, io, rc::Rc};

use ratatui::{layout::Rect, text::Line, Frame};

use crate::mode::{
    EditorMode, Mode,
    ModeType::{Exit, Normal},
};

pub type State = Rc<RefCell<BufferState>>;

pub struct BufferState {
    pub content: Dirty<Vec<String>>,
    pub cursor: Cursor,
}

impl BufferState {
    fn new(content: String) -> Self {
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

pub struct Buffer {
    name: String,
    pub state: State,
    pub mode: Mode,
}

impl Buffer {
    pub fn new(name: String, errors: &mut Vec<String>) -> Self {
        match Self::get_file_contents(&name) {
            Ok(content) => {
                let content = Rc::new(RefCell::new(BufferState::new(content)));
                Buffer {
                    name,
                    mode: Mode::new(Normal, Rc::clone(&content)),
                    state: content,
                }
            }
            Err(msg) => {
                errors.push(msg.to_string());
                let content = Rc::new(RefCell::new(BufferState::new(String::new())));

                Buffer {
                    name: String::new(),
                    mode: Mode::new(Normal, Rc::clone(&content)),
                    state: content,
                }
            }
        }
    }

    pub fn change_mode(&mut self) {
        if self.mode.should_change() && !self.should_exit() {
            self.mode = Mode::new(self.mode.new_type(), Rc::clone(&self.state));
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.mode.render(frame, area);
    }

    pub fn should_exit(&self) -> bool {
        self.mode.new_type() == Exit
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    fn get_file_contents(path: &str) -> io::Result<String> {
        if !path.is_empty() {
            fs::read_to_string(path)
        } else {
            Ok(String::new())
        }
    }
}

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
