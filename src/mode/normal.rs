use std::{cell::RefMut, time::SystemTime};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    widgets::Paragraph,
    Frame,
};

use crate::state::{BufferState, State};

use super::{
    insert, EditorMode,
    ModeType::{self, Command, Insert},
};

const KEY_TIMEOUT: u64 = 2;

pub struct NormalMode {
    mode: ModeType,
    keys: String,
    last_key: Option<SystemTime>,
    state: State,
}

impl NormalMode {
    pub fn new(state: State) -> Self {
        Self {
            mode: ModeType::Normal,
            keys: String::new(),
            last_key: None,
            state,
        }
    }
}

impl NormalMode {
    fn handle_keybindings(&mut self) {
        match self.keys.as_str() {
            ":" => self.mode = Command,
            "i" => self.mode = Insert,
            "a" => {
                self.mode = Insert;
                self.get_state().right()
            }
            "h" => self.get_state().left(),
            "j" => self.get_state().down(),
            "k" => self.get_state().up(),
            "l" => self.get_state().right(),
            "$" => self.get_state().end(),
            "^" => self.get_state().start(),
            "x" => self.get_state().remove_char(),
            _ => return,
        }

        self.keys = String::new();
    }

    fn get_state(&self) -> RefMut<BufferState> {
        (*self.state).borrow_mut()
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
        if let Some(time) = self.last_key {
            match SystemTime::now().duration_since(time) {
                Ok(time) if time.as_secs() >= KEY_TIMEOUT => {
                    self.last_key = None;
                    self.keys = String::new()
                }
                _ => {}
            }
        }

        match event.code {
            KeyCode::Char(char) => {
                self.keys.push(char);
                self.last_key = Some(SystemTime::now());
            }
            KeyCode::Left => self.get_state().left(),
            KeyCode::Up => self.get_state().up(),
            KeyCode::Down => self.get_state().down(),
            KeyCode::Right => self.get_state().right(),
            _ => {}
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
