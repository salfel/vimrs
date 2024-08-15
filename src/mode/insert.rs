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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Position;
    use ratatui::crossterm::event::KeyModifiers;

    #[test]
    fn exit() {
        let mut buf = Buffer::new(String::new());
        buf.mode = Mode::Insert;

        buf.handle_keys(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(buf.mode, Mode::Normal);
    }

    #[test]
    fn add_char() {
        let mut buf = Buffer::new(String::new());
        buf.mode = Mode::Insert;

        buf.input_text("hi");
        assert_eq!(buf.content[0], "hi");
    }

    #[test]
    fn pop_char() {
        let mut buf = Buffer::new(String::new());
        buf.mode = Mode::Insert;
        buf.input_text("test");

        buf.handle_keys(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
        assert_eq!(buf.content[0], "tes");

        buf.handle_keys(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        buf.input_text("test2");
        buf.cursor = Position { row: 1, col: 0 };
        buf.handle_keys(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
        assert_eq!(buf.content[0], "testest2");
    }

    #[test]
    fn new_line() {
        let mut buf = Buffer::new(String::new());
        buf.mode = Mode::Insert;
        buf.input_text("test");

        buf.handle_keys(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        buf.input_text("test2");

        assert_eq!(buf.content, vec!["test", "test2"]);
    }
}
