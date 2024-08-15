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

#[cfg(test)]
mod tests {
    use std::fs;

    use ratatui::crossterm::event::KeyModifiers;

    use crate::filesystem::read_file;

    use super::*;

    fn assert_written(filename: &str, content: &str) {
        assert_eq!(read_file(filename), content);
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn exit() {
        let mut buf = Buffer::new(String::new());
        buf.mode = Mode::Command;

        buf.input_keys("q");
        buf.handle_keys(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert!(buf.exit);
    }

    #[test]
    fn write() {
        let filename = String::from("test2");
        let mut buf = Buffer::new(filename.clone());
        buf.mode = Mode::Command;
        buf.content[0] = String::from("test");

        buf.input_keys("w");
        buf.handle_keys(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert_written(&filename, "test");
    }

    #[test]
    fn write_and_exit() {
        let filename = String::from("test3");
        let mut buf = Buffer::new(filename.clone());
        buf.mode = Mode::Command;
        buf.content[0] = String::from("test");

        buf.input_keys("wq");
        buf.handle_keys(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert_written(&filename, "test");
        assert!(buf.exit);
    }
}
