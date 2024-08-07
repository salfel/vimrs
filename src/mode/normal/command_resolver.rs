use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::state::State;

pub struct CommandResolver {
    keys: String,
    pub mode: String,
}

impl CommandResolver {
    pub fn new() -> Self {
        CommandResolver {
            keys: String::new(),
            mode: String::new(),
        }
    }

    pub fn handle(&mut self, event: KeyEvent, state: &mut State) {
        match event.code {
            KeyCode::Char(char) => self.keys.push(char),
            KeyCode::Down => state.down(),
            KeyCode::Up => state.up(),
            KeyCode::Left => state.left(),
            KeyCode::Right => state.right(),
            _ => {}
        }

        self.handle_command(state);
    }

    fn handle_command(&mut self, state: &mut State) {
        match self.keys.as_str() {
            "i" => self.mode = String::from("insert"),
            "a" => {
                state.right();
                self.mode = String::from("insert");
            }
            ":" => self.mode = String::from("command"),
            "j" => state.down(),
            "k" => state.up(),
            "h" => state.left(),
            "l" => state.right(),
            "x" => self.remove_char(state),
            _ => {}
        }
    }

    pub fn remove_char(&mut self, state: &mut State) {
        if let Some(row) = state.content.get_mut(state.cursor.row) {
            row.remove(state.cursor.col);
            if state.cursor.col == row.len() {
                state.left();
            }
        }
    }
}
