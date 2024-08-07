use std::{cell::RefCell, rc::Rc};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    prelude::Rect,
    widgets::Paragraph,
    Frame,
};

use crate::buffer::Dirty;

use super::{
    EditorMode,
    ModeType::{self, Exit, Normal},
};

pub struct CommandMode {
    mode: ModeType,
    keys: String,
    content: Rc<RefCell<Dirty<String>>>,
}

impl CommandMode {
    pub fn new(content: Rc<RefCell<Dirty<String>>>) -> Self {
        Self {
            mode: ModeType::Command,
            keys: String::new(),
            content,
        }
    }

    fn pop_char(&mut self) {
        if self.keys.is_empty() {
            self.mode = Normal;
        } else {
            self.keys.pop();
        }
    }

    #[allow(clippy::single_match)]
    fn execute_command(&mut self) {
        match self.keys.as_str() {
            "q" => self.mode = Exit,
            _ => {}
        }

        self.keys = String::new();
    }
}

impl EditorMode for CommandMode {
    fn label(&self) -> String {
        String::from("Command")
    }

    fn new_type(&self) -> ModeType {
        self.mode
    }

    fn handle_events(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(char) => self.keys.push(char),
            KeyCode::Backspace => self.pop_char(),
            KeyCode::Esc => self.mode = Normal,
            KeyCode::Enter => self.execute_command(),
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let content = (*self.content).borrow_mut();
        let paragraph = Paragraph::new(content.data.as_str());
        frame.render_widget(paragraph, layout[0]);

        let command = format!(":{}", self.keys);
        let paragraph = Paragraph::new(command);
        frame.render_widget(paragraph, layout[1]);
    }
}
