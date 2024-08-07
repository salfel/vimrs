use super::{command::CommandMode, insert::InsertMode, EditorMode, Mode};
use crate::{display::Display, state::State};
use command_resolver::CommandResolver;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

pub mod command_resolver;

pub struct NormalMode {
    state: State,
    command_resolver: CommandResolver,
}

impl NormalMode {
    pub fn new(state: State) -> Self {
        NormalMode {
            command_resolver: CommandResolver::new(),
            state,
        }
    }
}

impl EditorMode for NormalMode {
    fn label(&self) -> String {
        String::new()
    }

    fn should_change_mode(&self) -> bool {
        self.command_resolver.mode != String::new()
    }

    fn mode(self) -> Option<Box<Mode>> {
        match self.command_resolver.mode.as_str() {
            "insert" => Some(Box::new(Mode::Insert(InsertMode::new(self.state)))),
            "command" => Some(Box::new(Mode::Command(CommandMode::new(self.state)))),
            _ => None,
        }
    }

    fn handle_key(&mut self, event: KeyEvent) {
        self.command_resolver.handle(event, &mut self.state);
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
