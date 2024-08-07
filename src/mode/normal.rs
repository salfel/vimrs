use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    widgets::Paragraph,
    Frame,
};

use crate::buffer::Content;

use super::{
    EditorMode,
    ModeType::{self, Command, Insert},
};

pub struct NormalMode {
    mode: ModeType,
    keys: String,
    content: Content,
}

impl NormalMode {
    pub fn new(content: Content) -> Self {
        Self {
            mode: ModeType::Normal,
            keys: String::new(),
            content,
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
        let content = (*self.content).borrow_mut();
        let paragraph = Paragraph::new(content.data.as_str());
        frame.render_widget(paragraph, area);
    }
}
