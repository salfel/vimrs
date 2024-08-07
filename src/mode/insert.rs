use std::{cell::RefCell, rc::Rc};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::Rect,
    widgets::Paragraph,
    Frame,
};

use crate::buffer::Dirty;

use super::{
    EditorMode,
    ModeType::{self, Normal},
};

pub struct InsertMode {
    mode: ModeType,
    content: Rc<RefCell<Dirty<String>>>,
}

impl InsertMode {
    pub fn new(content: Rc<RefCell<Dirty<String>>>) -> Self {
        Self {
            mode: ModeType::Insert,
            content,
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
        let content = (*self.content).borrow_mut();
        let paragraph = Paragraph::new(content.data.as_str());
        frame.render_widget(paragraph, area);
    }
}
