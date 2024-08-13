use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{buffer::Buffer, motion::Motion, navigation::right};

use super::Mode;

pub fn handle_normal_keys(buf: &mut Buffer, event: KeyEvent) {
    match event.code {
        KeyCode::Char(key) => handle_char(buf, key),
        KeyCode::Esc => buf.keys = String::new(),
        _ => {}
    }
}

fn handle_char(buf: &mut Buffer, key: char) {
    buf.keys.push(key);

    match Motion::new(&buf.keys) {
        Some(motion) => {
            buf.cursor = motion.execute(buf);
            buf.keys = String::new();
        }
        None => execute_keybindings(buf),
    }
}

fn execute_keybindings(buf: &mut Buffer) {
    match buf.keys.as_str() {
        "i" => buf.change_mode(Mode::Insert),
        "a" => {
            buf.change_mode(Mode::Insert);
            buf.cursor = right(buf);
        }
        ":" => buf.change_mode(Mode::Command),
        _ => {}
    }
}
