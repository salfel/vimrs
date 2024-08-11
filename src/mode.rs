use std::fmt::Display;

use insert::handle_insert_keys;
use normal::handle_normal_keys;
use ratatui::crossterm::event::KeyEvent;

use crate::context::Context;

pub mod insert;
pub mod normal;

#[derive(Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mode::Normal => String::from("Normal"),
                Mode::Insert => String::from("Insert"),
            }
        )
    }
}

impl Mode {
    pub fn handle_keys(&self, cx: &mut Context, event: KeyEvent) {
        match self {
            Self::Insert => handle_insert_keys(cx, event),
            Self::Normal => handle_normal_keys(cx, event),
        }
    }
}
