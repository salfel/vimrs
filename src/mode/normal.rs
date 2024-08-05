use super::{command::CommandMode, insert::InsertMode, EditorMode, Mode};
use crate::{display::Display, state::State};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};
pub struct NormalMode {
    mode: String,
    state: State,
}

impl NormalMode {
    pub fn new(state: State) -> Self {
        NormalMode {
            mode: String::new(),
            state,
        }
    }
}

impl EditorMode for NormalMode {
    fn label(&self) -> String {
        String::from("Normal")
    }

    fn should_change_mode(&self) -> bool {
        self.mode != String::new()
    }

    fn mode(self) -> Option<Box<Mode>> {
        match self.mode.as_str() {
            "insert" => Some(Box::new(Mode::Insert(InsertMode::new(self.state)))),
            "command" => Some(Box::new(Mode::Command(CommandMode::new(self.state)))),
            _ => None,
        }
    }

    fn handle_key(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('i') => self.mode = String::from("insert"),
            KeyCode::Char(':') => self.mode = String::from("command"),
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let display = Display::new(&self.state.content);
        frame.render_widget(display, rect);
    }

    fn get_state(&self) -> &State {
        &self.state
    }
}
