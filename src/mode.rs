use std::{cell::RefCell, rc::Rc};

use command::CommandMode;
use normal::NormalMode;
use ratatui::crossterm::event::KeyEvent;

use crate::buffer::Dirty;

pub mod command;
pub mod normal;

#[derive(Clone, Copy, PartialEq)]
pub enum ModeType {
    Normal,
    Command,
    Exit,
}

pub trait EditorMode {
    fn label(&self) -> String {
        String::new()
    }

    fn new_type(&self) -> ModeType;

    fn handle_events(&mut self, event: KeyEvent);
}

pub enum Mode {
    Normal(NormalMode),
    Command(CommandMode),
}

impl Mode {
    pub fn new(mode: ModeType, content: Rc<RefCell<Dirty<String>>>) -> Self {
        match mode {
            ModeType::Normal => Mode::Normal(NormalMode::new(content)),
            ModeType::Command => Mode::Command(CommandMode::new(content)),
            ModeType::Exit => panic!("exit should never be passed in here"),
        }
    }

    pub fn should_change(&self) -> bool {
        let current_mode = match self {
            Mode::Normal(_) => ModeType::Normal,
            Mode::Command(_) => ModeType::Command,
        };

        current_mode != self.new_type()
    }
}

impl EditorMode for Mode {
    fn label(&self) -> String {
        match self {
            Mode::Normal(mode) => mode.label(),
            Mode::Command(mode) => mode.label(),
        }
    }

    fn new_type(&self) -> ModeType {
        match self {
            Mode::Normal(mode) => mode.new_type(),
            Mode::Command(mode) => mode.new_type(),
        }
    }

    fn handle_events(&mut self, event: KeyEvent) {
        match self {
            Mode::Normal(mode) => mode.handle_events(event),
            Mode::Command(mode) => mode.handle_events(event),
        }
    }
}
