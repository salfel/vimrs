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

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::KeyModifiers;

    use crate::{
        buffer::Position,
        test::{assert_count, assert_event},
    };

    use super::*;

    #[test]
    fn switch_to_insert() {
        let mut buf = Buffer::new(String::from("test.txt"));
        buf.handle_keys(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));

        assert_eq!(buf.mode, Mode::Insert);
        assert_eq!(buf.cursor, Position { row: 0, col: 0 });

        buf.mode = Mode::Normal;
        buf.handle_keys(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));

        assert_eq!(buf.mode, Mode::Insert);
        assert_eq!(buf.cursor, Position { row: 0, col: 1 });

        assert_eq!(buf.keys, String::new());
    }

    #[test]
    fn switch_to_command() {
        let mut buf = Buffer::new(String::new());
        buf.handle_keys(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));

        assert_eq!(buf.mode, Mode::Command);
        assert_eq!(buf.keys, String::new());
    }

    #[test]
    fn motion_executed() {
        let mut buf = Buffer::new(String::from("test.txt"));
        buf.handle_keys(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));

        assert_count(&buf.events, 1);
        assert_event(&buf, crate::test::Event::Motion(Motion::Left));
        assert_eq!(buf.keys, String::new());
    }

    #[test]
    fn added_to_keys() {
        let mut buf = Buffer::new(String::new());
        buf.handle_keys(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));

        assert_eq!(buf.keys, String::from("d"));
    }
}
