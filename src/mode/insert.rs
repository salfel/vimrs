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

    fn write_char(&mut self, char: char) {
        let mut state = self.get_state();
        let cursor = state.cursor;

        if let Some(line) = state.content.data.get_mut(cursor.row) {
            line.insert(cursor.col, char);
            state.right();
        }
    }

    fn pop_char(&mut self) {
        let mut state = self.get_state();
        let cursor = state.cursor;

        if let Some(line) = state.content.data.get_mut(cursor.row) {
            if line.is_empty() {
                return;
            }

            line.remove(cursor.col - 1);

            state.left();
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
            KeyCode::Esc => self.mode = Normal,
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let state = (*self.state).borrow_mut();
        let paragraph = Paragraph::new(state.get_lines_from_content());
        frame.render_widget(paragraph, area);

        frame.render_widget(state.cursor, area);
    }
}
