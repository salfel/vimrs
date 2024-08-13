use std::fmt::Display;

use command::handle_command_keys;
use insert::handle_insert_keys;
use normal::handle_normal_keys;
use ratatui::crossterm::event::KeyEvent;

use crate::buffer::Buffer;

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
    pub fn handle_keys(&self, buf: &mut Buffer, event: KeyEvent) {
        match self {
            Mode::Insert => handle_insert_keys(buf, event),
            Mode::Normal => handle_normal_keys(buf, event),
            Mode::Command => handle_command_keys(buf, event),
        }
    }
}
