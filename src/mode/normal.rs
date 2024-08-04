use std::rc::Rc;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::state::State;

use super::{insert::Insert, Mode};

pub struct Normal {
    mode: Option<Box<dyn Mode>>,
    state: Rc<State>,
}

impl Normal {
    pub fn new(state: Rc<State>) -> Self {
        Normal { mode: None, state }
    }
}

impl Mode for Normal {
    fn mode(&mut self) -> Option<Box<dyn Mode>> {
        self.mode.take()
    }

    fn handle_key(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('i') => self.mode = Some(Box::new(Insert::new(Rc::clone(&self.state)))),
            _ => {}
        }
    }
}
