use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::buffer::Buffer;

use super::Mode;

pub fn handle_command_keys(buf: &mut Buffer, event: KeyEvent) {
    match event.code {
        KeyCode::Char(key) => buf.keys.push(key),
        KeyCode::Backspace => pop_char(buf),
        KeyCode::Enter => execute_command(buf),
        KeyCode::Esc => buf.change_mode(Mode::Normal),
        _ => {}
    }
}

#[allow(clippy::single_match)]
fn execute_command(buf: &mut Buffer) {
    match buf.keys.as_str() {
        "q" => buf.exit = true,
        "w" => buf.write(),
        "wq" => {
            buf.exit = true;
            buf.write();
        }
        _ => {}
    }

    buf.keys = String::new();
}

fn pop_char(buf: &mut Buffer) {
    if buf.keys.pop().is_none() {
        buf.change_mode(Mode::Normal);
    }
}
