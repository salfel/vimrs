use super::{normal::Normal, Mode};
use crate::{display::Display, state::State};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};
use std::rc::Rc;

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
    fn label(&self) -> String {
        String::from("Insert")
    }

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

    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let display = Display::new(self.state.get_content());
        frame.render_widget(display, rect);
    }
}
