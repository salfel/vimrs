use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{context::Context, mode::Mode};

pub fn handle_insert_keys(cx: &mut Context, event: KeyEvent) {
    match event.code {
        KeyCode::Char(key) => insert_char(cx, key),
        KeyCode::Backspace => pop_char(cx),
        KeyCode::Esc => {
            cx.change_mode(Mode::Normal);
            cx.cursor.col -= 1;
        }
        _ => {}
    }
}

fn insert_char(cx: &mut Context, key: char) {
    if let Some(line) = cx.content.get_mut(cx.cursor.row) {
        line.insert(cx.cursor.col, key);
        cx.cursor.col += 1;
    }
}

fn pop_char(cx: &mut Context) {
    if let Some(line) = cx.content.get_mut(cx.cursor.row) {
        // TODO implement remove line
        if cx.cursor.col == 0 {
            return;
        }

        line.remove(cx.cursor.col - 1);
        cx.cursor.col -= 1;
    }
}
