use std::rc::Rc;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::state::State;

use super::{normal::Normal, Mode};

pub struct Insert {
    state: Rc<State>,
    exit: bool,
}

impl Insert {
    pub fn new(state: Rc<State>) -> Self {
        Insert { exit: false, state }
    }
}

impl Mode for Insert {
    fn mode(&mut self) -> Option<Box<dyn Mode>> {
        if self.exit {
            Some(Box::new(Normal::new(Rc::clone(&self.state))))
        } else {
            None
        }
    }

    fn handle_key(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => self.exit = true,
            _ => {}
        }
    }
}
