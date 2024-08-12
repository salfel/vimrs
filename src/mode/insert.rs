use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{context::Context, mode::Mode, navigation::move_left};

pub fn handle_insert_keys(cx: &mut Context, event: KeyEvent) {
    match event.code {
        KeyCode::Char(key) => insert_char(cx, key),
        KeyCode::Backspace => pop_char(cx),
        KeyCode::Enter => new_line(cx),
        KeyCode::Esc => {
            cx.change_mode(Mode::Normal);
            move_left(cx);
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
        if cx.cursor.col == 0 {
            if cx.cursor.row != 0 {
                let content = cx.content.remove(cx.cursor.row);
                let prev_row = cx
                    .content
                    .get_mut(cx.cursor.row - 1)
                    .unwrap_or_else(|| panic!("row: {} doesn't exist", cx.cursor.row));

                cx.cursor.row -= 1;
                cx.cursor.col = prev_row.len();
                prev_row.push_str(&content);
            };

            return;
        }

        line.remove(cx.cursor.col - 1);
        cx.cursor.col -= 1;
    }
}

fn new_line(cx: &mut Context) {
    if let Some(line) = cx.content.get_mut(cx.cursor.row) {
        let content = line.drain(cx.cursor.col..).collect();
        cx.content.insert(cx.cursor.row + 1, content);
        cx.cursor.row += 1;
        cx.cursor.col = 0;
    }
}
