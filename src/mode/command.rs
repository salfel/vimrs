use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::context::Context;

use super::Mode;

pub fn handle_command_keys(cx: &mut Context, event: KeyEvent) {
    match event.code {
        KeyCode::Char(key) => cx.keys.push(key),
        KeyCode::Backspace => pop_char(cx),
        KeyCode::Enter => execute_command(cx),
        KeyCode::Esc => cx.keys = String::new(),
        _ => {}
    }
}

#[allow(clippy::single_match)]
fn execute_command(cx: &mut Context) {
    match cx.keys.as_str() {
        "q" => cx.exit = true,
        _ => {}
    }

    cx.keys = String::new();
}

fn pop_char(cx: &mut Context) {
    if cx.keys.pop().is_none() {
        cx.change_mode(Mode::Normal);
    }
}
