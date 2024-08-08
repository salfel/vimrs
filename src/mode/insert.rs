use std::cell::RefMut;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::Rect,
    widgets::Paragraph,
    Frame,
};

use crate::state::{BufferState, State};

use super::{
    EditorMode,
    ModeType::{self, Normal},
};

pub struct InsertMode {
    mode: ModeType,
    state: State,
}

impl InsertMode {
    pub fn new(state: State) -> Self {
        Self {
            mode: ModeType::Insert,
            state,
        }
    }

    fn write_char(&self, char: char) {
        let mut state = self.get_state();
        let cursor = state.cursor;

        if let Some(line) = state.content.data.get_mut(cursor.row) {
            line.insert(cursor.col, char);
            state.right();
        }
    }

    fn pop_char(&self) {
        let mut state = self.get_state();
        let cursor = state.cursor;

        if let Some(line) = state.content.data.get_mut(cursor.row) {
            if cursor.col == 0 {
                self.pop_line(state);
            } else {
                line.remove(cursor.col - 1);

                state.left();
            }
        }
    }

    fn pop_line(&self, mut state: RefMut<BufferState>) {
        let cursor = state.cursor;

        if cursor.row == 0 {
            return;
        }

        let remaining = state.content.data.remove(cursor.row);
        let prev_row = &mut state.content.data[cursor.row - 1];
        let prev_row_len = prev_row.len();
        prev_row.push_str(&remaining);

        state.up();
        for _ in 0..prev_row_len {
            state.right();
        }
    }

    fn get_state(&self) -> RefMut<BufferState> {
        (*self.state).borrow_mut()
    }
}

impl EditorMode for InsertMode {
    fn label(&self) -> String {
        String::from("Insert")
    }

    fn new_type(&self) -> ModeType {
        self.mode
    }

    fn handle_events(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(char) => self.write_char(char),
            KeyCode::Backspace => self.pop_char(),
            KeyCode::Left => self.get_state().left(),
            KeyCode::Right => self.get_state().right(),
            KeyCode::Up => self.get_state().up(),
            KeyCode::Down => self.get_state().down(),
            KeyCode::Enter => self.get_state().new_line(),
            KeyCode::Esc => {
                self.mode = Normal;
                self.get_state().left()
            }
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let state = (*self.state).borrow_mut();
        let paragraph = Paragraph::new(state.get_lines_from_content());
        frame.render_widget(paragraph, area);

        state.cursor.render(frame, 1);
    }
}
