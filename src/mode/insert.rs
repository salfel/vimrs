use super::{normal::NormalMode, EditorMode, Mode};
use crate::{display::Display, state::State};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};

pub struct InsertMode {
    state: State,
    exit: bool,
}

impl InsertMode {
    pub fn new(state: State) -> Self {
        InsertMode { exit: false, state }
    }
}

impl EditorMode for InsertMode {
    fn label(&self) -> String {
        String::from("Insert")
    }

    fn should_change_mode(&self) -> bool {
        self.exit
    }

    fn mode(self) -> Option<Box<Mode>> {
        if self.exit {
            Some(Box::new(Mode::Normal(NormalMode::new(self.state))))
        } else {
            None
        }
    }

    fn handle_key(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => self.exit = true,
            KeyCode::Char(char) => self.state.write_char(char),
            KeyCode::Backspace => self.state.remove_char(),
            KeyCode::Enter => self.state.new_row(),
            _ => {}
        };
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let display = Display::new(&self.state);
        frame.render_widget(display, rect);
    }

    fn get_state(&self) -> &State {
        &self.state
    }
}
