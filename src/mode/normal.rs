use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    widgets::Paragraph,
    Frame,
};

use crate::buffer::State;

use super::{
    EditorMode,
    ModeType::{self, Command, Insert},
};

pub struct NormalMode {
    mode: ModeType,
    keys: String,
    state: State,
}

impl NormalMode {
    pub fn new(state: State) -> Self {
        Self {
            mode: ModeType::Normal,
            keys: String::new(),
            state,
        }
    }
}

impl NormalMode {
    fn handle_keybindings(&mut self) {
        match self.keys.as_str() {
            ":" => self.mode = Command,
            "i" => self.mode = Insert,
            _ => return,
        }

        self.keys = String::new();
    }
}

impl EditorMode for NormalMode {
    fn label(&self) -> String {
        String::from("Normal")
    }

    fn new_type(&self) -> ModeType {
        self.mode
    }

    fn handle_events(&mut self, event: KeyEvent) {
        if let KeyCode::Char(char) = event.code {
            self.keys.push(char);
        }

        self.handle_keybindings();
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let state = (*self.state).borrow_mut();
        let paragraph = Paragraph::new(state.get_lines_from_content());
        frame.render_widget(paragraph, area);

        frame.render_widget(state.cursor, area);
    }
}
