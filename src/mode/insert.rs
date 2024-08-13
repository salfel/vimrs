use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{buffer::Buffer, mode::Mode, navigation::left};

pub fn handle_insert_keys(buf: &mut Buffer, event: KeyEvent) {
    match event.code {
        KeyCode::Char(key) => insert_char(buf, key),
        KeyCode::Backspace => pop_char(buf),
        KeyCode::Enter => new_line(buf),
        KeyCode::Esc => {
            buf.change_mode(Mode::Normal);
            buf.cursor = left(buf);
        }
        _ => {}
    }
}

fn insert_char(buf: &mut Buffer, key: char) {
    if let Some(line) = buf.content.get_mut(buf.cursor.row) {
        line.insert(buf.cursor.col, key);
        buf.cursor.col += 1;
    }
}

fn pop_char(buf: &mut Buffer) {
    if let Some(line) = buf.content.get_mut(buf.cursor.row) {
        if buf.cursor.col == 0 {
            if buf.cursor.row != 0 {
                let content = buf.content.remove(buf.cursor.row);
                let prev_row = buf
                    .content
                    .get_mut(buf.cursor.row - 1)
                    .unwrap_or_else(|| panic!("row: {} doesn't exist", buf.cursor.row));

                buf.cursor.row -= 1;
                buf.cursor.col = prev_row.len();
                prev_row.push_str(&content);
            };

            return;
        }

        line.remove(buf.cursor.col - 1);
        buf.cursor.col -= 1;
    }
}

fn new_line(buf: &mut Buffer) {
    if let Some(line) = buf.content.get_mut(buf.cursor.row) {
        let content = line.drain(buf.cursor.col..).collect();
        buf.content.insert(buf.cursor.row + 1, content);
        buf.cursor.row += 1;
        buf.cursor.col = 0;
    }
}
