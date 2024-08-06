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
        String::new()
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
            KeyCode::Char('i') => {
                self.state.left();
                self.mode = String::from("insert")
            }
            KeyCode::Char(':') => self.mode = String::from("command"),
            KeyCode::Char('j') | KeyCode::Down => self.state.down(),
            KeyCode::Char('k') | KeyCode::Up => self.state.up(),
            KeyCode::Char('h') | KeyCode::Left => self.state.left(),
            KeyCode::Char('l') | KeyCode::Right => self.state.right(),
            KeyCode::Char('x') => self.state.remove_char(),
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        self.state
            .print(format!("cursor: {}", self.state.cursor.col));
        let display = Display::new(&self.state, false);
        frame.render_widget(display, rect);
    }

    fn get_state(&mut self) -> &mut State {
        &mut self.state
    }
}
