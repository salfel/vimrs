use std::{cell::RefCell, rc::Rc};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    widgets::Paragraph,
    Frame,
};

use crate::buffer::Dirty;

use super::{EditorMode, ModeType};

pub struct NormalMode {
    mode: ModeType,
    keys: String,
    content: Rc<RefCell<Dirty<String>>>,
}

impl NormalMode {
    pub fn new(content: Rc<RefCell<Dirty<String>>>) -> Self {
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
            ":" => self.mode = ModeType::Command,
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
