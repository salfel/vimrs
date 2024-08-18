use std::cmp::min;

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

    let found = match Motion::new(&buf.keys) {
        Some(motion) => {
            buf.cursor = motion.execute(buf);
            true
        }
        None => execute_keybindings(buf),
    };

    if found {
        buf.keys = String::new();
    }
}

fn execute_keybindings(buf: &mut Buffer) -> bool {
    match buf.keys.as_str() {
        "i" => buf.mode = Mode::Insert,
        "a" => {
            buf.mode = Mode::Insert;
            buf.cursor = right(buf);
        }
        ":" => buf.mode = Mode::Command,
        "x" => delete_char(buf),
        _ => return false,
    }

    true
}

fn delete_char(buf: &mut Buffer) {
    let line = &mut buf.content[buf.cursor.row];
    if line.is_empty() {
        return;
    }

    line.remove(buf.cursor.col);
    buf.cursor.col = min(buf.cursor.col, line.len() - 1);
}

#[cfg(test)]
mod tests {
    use crate::{
        buffer::Position,
        test::{assert_count, assert_event, Event},
    };

    use super::*;

    #[test]
    fn switch_to_insert() {
        let mut buf = Buffer::new(String::from("test.txt"));
        buf.input_keys("i");

        assert_eq!(buf.mode, Mode::Insert);
        assert_eq!(buf.cursor, Position { row: 0, col: 0 });

        buf.mode = Mode::Normal;
        buf.input_keys("a");

        assert_eq!(buf.mode, Mode::Insert);
        assert_eq!(buf.cursor, Position { row: 0, col: 1 });

        assert_eq!(buf.keys, String::new());
    }

    #[test]
    fn switch_to_command() {
        let mut buf = Buffer::new(String::new());
        buf.input_keys(":");

        assert_eq!(buf.mode, Mode::Command);
        assert_eq!(buf.keys, String::new());
    }

    #[test]
    fn motion_executed() {
        let mut buf = Buffer::new(String::from("test.txt"));
        buf.input_keys("h");

        assert_count(&buf.events, 1);
        assert_event(&buf, Event::Motion(Motion::Left));
        assert_eq!(buf.keys, String::new());
    }

    #[test]
    fn added_to_keys() {
        let mut buf = Buffer::new(String::new());
        buf.input_keys("d");

        assert_eq!(buf.keys, String::from("d"));
    }

    #[test]
    fn deletes_char() {
        let mut buf = Buffer::new(String::from("test.txt"));
        buf.cursor = Position { row: 0, col: 4 };
        buf.input_keys("x");

        assert_eq!(buf.content[0], "Lore ipsum odor amet, ");
        assert_eq!(buf.cursor, Position { row: 0, col: 4 });

        buf.cursor = Position { row: 5, col: 0 };
        buf.input_keys("x");
        assert_eq!(buf.content[5], "");

        buf.cursor = Position { row: 0, col: 21 };
        buf.input_keys("x");
        assert_eq!(buf.content[0], "Lore ipsum odor amet,");
    }
}
