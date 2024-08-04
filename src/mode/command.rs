use super::{normal::Normal, Mode};
use crate::{display::Display, state::State};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::Rect,
    Frame,
};
use std::rc::Rc;

pub struct Command {
    mode: Option<Box<dyn Mode>>,
    state: Rc<State>,
    command: String,
}

impl Command {
    pub fn new(state: Rc<State>) -> Self {
        Command {
            mode: None,
            state,
            command: String::new(),
        }
    }

    fn execute_command(&mut self) {
        self.exit_to_normal();
    }

    fn exit_to_normal(&mut self) {
        self.mode = Some(Box::new(Normal::new(Rc::clone(&self.state))));
    }
}

impl Mode for Command {
    fn label(&self) -> String {
        String::from("Command")
    }

    fn mode(&mut self) -> Option<Box<dyn Mode>> {
        self.mode.take()
    }

    fn handle_key(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => self.exit_to_normal(),
            KeyCode::Char(char) => self.command.push(char),
            KeyCode::Enter => self.execute_command(),
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let display = Display::new(self.state.get_content());
        frame.render_widget(display, rect);
    }
}
