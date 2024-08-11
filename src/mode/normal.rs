use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{context::Context, motion::Motion};

use super::Mode;

pub fn handle_normal_keys(cx: &mut Context, event: KeyEvent) {
    match event.code {
        KeyCode::Char(key) => handle_char(cx, key),
        KeyCode::Esc => cx.keys = String::new(),
        _ => {}
    }
}

fn handle_char(cx: &mut Context, key: char) {
    cx.keys.push(key);

    match Motion::new(&cx.keys) {
        Some(motion) => cx.cursor = motion.execute(cx),
        None => execute_keybindings(cx),
    }
}

fn execute_keybindings(cx: &mut Context) {
    match cx.keys.as_str() {
        "i" => {
            cx.mode = Mode::Insert;
            Motion::Left.execute(cx);
        }
        "a" => cx.mode = Mode::Insert,
        _ => {}
    }
}
