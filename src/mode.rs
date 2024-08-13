use std::fmt::Display;

use command::handle_command_keys;
use insert::handle_insert_keys;
use normal::handle_normal_keys;
use ratatui::crossterm::event::KeyEvent;

use crate::context::Context;

pub mod command;
pub mod insert;
pub mod normal;

#[derive(Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mode::Normal => "Normal",
                Mode::Insert => "Insert",
                Mode::Command => "Command",
            }
        )
    }
}

impl Mode {
    pub fn handle_keys(&self, cx: &mut Context, event: KeyEvent) {
        match self {
            Mode::Insert => handle_insert_keys(cx, event),
            Mode::Normal => handle_normal_keys(cx, event),
            Mode::Command => handle_command_keys(cx, event),
        }
    }
}
