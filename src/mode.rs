use std::{cell::RefCell, rc::Rc};

use command::CommandMode;
use insert::InsertMode;
use normal::NormalMode;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::buffer::{Content, Dirty};

pub mod command;
pub mod insert;
pub mod normal;

#[derive(Clone, Copy, PartialEq)]
pub enum ModeType {
    Normal,
    Insert,
    Command,
    Exit,
}

pub trait EditorMode {
    fn label(&self) -> String {
        String::new()
    }

    fn new_type(&self) -> ModeType;

    fn handle_events(&mut self, event: KeyEvent);

    fn render(&self, frame: &mut Frame, area: Rect);
}

pub enum Mode {
    Normal(NormalMode),
    Insert(InsertMode),
    Command(CommandMode),
}

impl Mode {
    pub fn new(mode: ModeType, content: Content) -> Self {
        match mode {
            ModeType::Normal => Mode::Normal(NormalMode::new(content)),
            ModeType::Insert => Mode::Insert(InsertMode::new(content)),
            ModeType::Command => Mode::Command(CommandMode::new(content)),
            ModeType::Exit => panic!("exit should never be passed in here"),
        }
    }

    pub fn should_change(&self) -> bool {
        let current_mode = match self {
            Mode::Normal(_) => ModeType::Normal,
            Mode::Insert(_) => ModeType::Insert,
            Mode::Command(_) => ModeType::Command,
        };

        current_mode != self.new_type()
    }
}

impl EditorMode for Mode {
    fn label(&self) -> String {
        match self {
            Mode::Normal(mode) => mode.label(),
            Mode::Insert(mode) => mode.label(),
            Mode::Command(mode) => mode.label(),
        }
    }

    fn new_type(&self) -> ModeType {
        match self {
            Mode::Normal(mode) => mode.new_type(),
            Mode::Insert(mode) => mode.new_type(),
            Mode::Command(mode) => mode.new_type(),
        }
    }

    fn handle_events(&mut self, event: KeyEvent) {
        match self {
            Mode::Normal(mode) => mode.handle_events(event),
            Mode::Insert(mode) => mode.handle_events(event),
            Mode::Command(mode) => mode.handle_events(event),
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        match self {
            Mode::Normal(mode) => mode.render(frame, area),
            Mode::Insert(mode) => mode.render(frame, area),
            Mode::Command(mode) => mode.render(frame, area),
        }
    }
}
