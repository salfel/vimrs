use std::io::stdout;

use ratatui::{
    crossterm::{
        cursor,
        event::{KeyCode, KeyEvent},
        execute,
    },
    prelude::Rect,
    widgets::Paragraph,
    Frame,
};

use crate::buffer::State;

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
        let mut state = (*self.state).borrow_mut();
        let cursor = state.cursor;

        if let Some(line) = state.content.data.get_mut(cursor.row) {
            line.insert(cursor.col, char);
            state.right();
        }
    }

    fn pop_char(&mut self) {
        let mut state = (*self.state).borrow_mut();
        let cursor = state.cursor;

        if let Some(line) = state.content.data.get_mut(cursor.row) {
            if line.is_empty() {
                return;
            }

            line.remove(cursor.col - 1);

            state.left();
        }
    }

    fn left(&self) {
        let mut state = (*self.state).borrow_mut();

        state.left();
    }

    fn right(&self) {
        let mut state = (*self.state).borrow_mut();

        state.right();
    }

    fn up(&self) {
        let mut state = (*self.state).borrow_mut();

        state.up();
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
            KeyCode::Left => self.left(),
            KeyCode::Right => self.right(),
            KeyCode::Up => self.up(),
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
