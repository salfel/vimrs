use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
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
}

impl EditorMode for InsertMode {
    fn label(&self) -> String {
        String::from("Insert")
    }

    fn new_type(&self) -> ModeType {
        self.mode
    }

    #[allow(clippy::single_match)]
    fn handle_events(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => self.mode = Normal,
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let state = (*self.state).borrow_mut();
        let paragraph = Paragraph::new(state.get_lines_from_content());
        frame.render_widget(paragraph, area);
    }
}
