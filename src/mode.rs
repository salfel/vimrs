use command::CommandMode;
use insert::InsertMode;
use normal::NormalMode;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::state::State;

pub mod command;
pub mod insert;
pub mod normal;

pub trait EditorMode {
    fn should_change_mode(&self) -> bool;

    fn mode(self) -> Option<Box<Mode>>;

    fn handle_key(&mut self, event: KeyEvent);

    fn label(&self) -> String;

    fn render(&mut self, frame: &mut Frame, rect: Rect);

    fn get_state(&mut self) -> &mut State;
}

pub enum Mode {
    Normal(NormalMode),
    Insert(InsertMode),
    Command(CommandMode),
}

impl Mode {
    pub fn new(state: State) -> Self {
        Mode::Normal(NormalMode::new(state))
    }
}

impl EditorMode for Mode {
    fn label(&self) -> String {
        match self {
            Self::Normal(mode) => mode.label(),
            Self::Insert(mode) => mode.label(),
            Self::Command(mode) => mode.label(),
        }
    }

    fn mode(self) -> Option<Box<Mode>> {
        match self {
            Self::Normal(mode) => mode.mode(),
            Self::Insert(mode) => mode.mode(),
            Self::Command(mode) => mode.mode(),
        }
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        match self {
            Self::Normal(mode) => mode.render(frame, rect),
            Self::Insert(mode) => mode.render(frame, rect),
            Self::Command(mode) => mode.render(frame, rect),
        }
    }

    fn handle_key(&mut self, event: KeyEvent) {
        match self {
            Self::Normal(mode) => mode.handle_key(event),
            Self::Insert(mode) => mode.handle_key(event),
            Self::Command(mode) => mode.handle_key(event),
        }
    }

    fn get_state(&mut self) -> &mut State {
        match self {
            Self::Normal(mode) => mode.get_state(),
            Self::Insert(mode) => mode.get_state(),
            Self::Command(mode) => mode.get_state(),
        }
    }

    fn should_change_mode(&self) -> bool {
        match self {
            Self::Normal(mode) => mode.should_change_mode(),
            Self::Insert(mode) => mode.should_change_mode(),
            Self::Command(mode) => mode.should_change_mode(),
        }
    }
}
