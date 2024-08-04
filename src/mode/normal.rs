use super::{command::Command, insert::Insert, Mode};
use crate::{display::Display, state::State};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};
use std::rc::Rc;

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
    fn label(&self) -> String {
        String::from("Normal")
    }

    fn mode(&mut self) -> Option<Box<dyn Mode>> {
        self.mode.take()
    }

    fn handle_key(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('i') => self.mode = Some(Box::new(Insert::new(Rc::clone(&self.state)))),
            KeyCode::Char(':') => self.mode = Some(Box::new(Command::new(Rc::clone(&self.state)))),
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let display = Display::new(self.state.get_content());
        frame.render_widget(display, rect);
    }
}
