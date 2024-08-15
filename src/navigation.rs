use std::{cmp::min, usize};

use crate::{
    buffer::{Buffer, Position},
    mode::Mode,
};

pub fn right(buf: &Buffer) -> Position {
    let mut row_len = buf.row(buf.cursor.row).len();
    if let Mode::Normal = buf.mode {
        row_len -= 1;
    }

    if buf.cursor.col >= row_len {
        buf.cursor
    } else {
        Position {
            row: buf.cursor.row,
            col: buf.cursor.col + 1,
        }
    }
}

pub fn left(buf: &Buffer) -> Position {
    if buf.cursor.col == 0 {
        buf.cursor
    } else {
        Position {
            row: buf.cursor.row,
            col: buf.cursor.col - 1,
        }
    }
}

pub fn up(buf: &Buffer) -> Position {
    if buf.cursor.row == 0 {
        return buf.cursor;
    }

    let prev_row = buf
        .content
        .get(buf.cursor.row - 1)
        .unwrap_or_else(|| panic!("row: {} doesn't exist", buf.cursor.row));

    Position {
        row: buf.cursor.row - 1,
        col: min(buf.cursor.col, prev_row.len().max(1) - 1),
    }
}

pub fn down(buf: &Buffer) -> Position {
    if buf.cursor.row >= buf.content.len() - 1 {
        return buf.cursor;
    }

    let next_row = buf
        .content
        .get(buf.cursor.row + 1)
        .unwrap_or_else(|| panic!("row: {}, doesn't exist", buf.cursor.row));

    Position {
        row: buf.cursor.row + 1,
        col: min(buf.cursor.col, next_row.len().max(1) - 1),
    }
}

pub fn start_line(buf: &Buffer) -> Position {
    Position {
        row: buf.cursor.row,
        col: 0,
    }
}

pub fn end_line(buf: &Buffer) -> Position {
    if let Some(row) = buf.content.get(buf.cursor.row) {
        Position {
            row: buf.cursor.row,
            col: row.len().max(1) - 1,
        }
    } else {
        buf.cursor
    }
}

const WORD_DELIMITERS: [char; 11] = ['(', ')', '[', ']', '{', '}', '$', '^', '!', '.', ','];

#[allow(clippy::iter_skip_zero)]
pub fn word_end(buf: &Buffer) -> Position {
    let line = buf.row(buf.cursor.row);
    let mut cursor = buf.cursor;

    let (iterator, mut prev) = if buf.cursor.col == last_not_whitespace(line) {
        match buf.content.get(buf.cursor.row + 1) {
            Some(line) => {
                cursor.row += 1;
                (line.chars().enumerate().skip(0), ' ')
            }
            None => return buf.cursor,
        }
    } else {
        let mut iterator = line.chars().enumerate().skip(buf.cursor.col);
        let prev = match iterator.next() {
            Some(item) => item.1,
            None => return buf.cursor,
        };
        (iterator, prev)
    };

    for (idx, char) in iterator {
        if ((is_word_delimiter(char) && !is_word_delimiter(prev)) || (char == ' ' && prev != ' '))
            && idx - 1 != cursor.col
        {
            return Position {
                row: cursor.row,
                col: idx - 1,
            };
        }

        prev = char;
    }

    Position {
        row: cursor.row,
        col: last_not_whitespace(line),
    }
}

#[allow(clippy::iter_skip_zero)]
pub fn prev_word_start(buf: &Buffer) -> Position {
    let mut line = buf.row(buf.cursor.row);
    let mut cursor = buf.cursor;

    let (iterator, mut prev) = if cursor.col == first_not_whitespace(line) && cursor.row != 0 {
        match buf.content.get(cursor.row - 1) {
            Some(row) => {
                cursor.row -= 1;
                line = row;
                (line.chars().rev().enumerate().skip(0), ' ')
            }
            None => return buf.cursor,
        }
    } else {
        let mut iterator = line
            .chars()
            .rev()
            .enumerate()
            .skip(line.len() - 1 - cursor.col);
        let prev = match iterator.next() {
            Some(item) => item.1,
            None => return buf.cursor,
        };
        (iterator, prev)
    };

    for (idx, char) in iterator {
        if ((is_word_delimiter(char) && (!is_word_delimiter(prev))) || (char == ' ' && prev != ' '))
            && line.len() - idx != cursor.col
        {
            return Position {
                row: cursor.row,
                col: line.len() - 1 - idx,
            };
        }

        prev = char;
    }

    Position {
        row: cursor.row,
        col: first_not_whitespace(line),
    }
}

#[allow(clippy::iter_skip_zero)]
pub fn word_start(buf: &Buffer) -> Position {
    let mut line = buf.row(buf.cursor.row);
    let mut cursor = buf.cursor;

    let (iterator, mut prev) =
        if no_word_after(line, cursor.col) && cursor.row != buf.content.len() - 1 {
            match buf.content.get(cursor.row + 1) {
                Some(row) => {
                    cursor.row += 1;
                    line = row;
                    (line.chars().enumerate().skip(0), ' ')
                }
                None => return buf.cursor,
            }
        } else {
            let mut iterator = line.chars().enumerate().skip(buf.cursor.col);
            let prev = match iterator.next() {
                Some(item) => item.1,
                None => return cursor,
            };
            (iterator, prev)
        };

    for (idx, char) in iterator {
        if (is_word_delimiter(char) && !is_word_delimiter(prev) || (char != ' ' && prev == ' '))
            && idx != cursor.col
        {
            return Position {
                row: cursor.row,
                col: idx,
            };
        }

        prev = char;
    }

    buf.cursor
}

fn is_word_delimiter(char: char) -> bool {
    WORD_DELIMITERS.contains(&char)
}

fn last_not_whitespace(line: &str) -> usize {
    let mut iterator = line.chars().rev().enumerate().filter_map(|value| {
        if value.1 != ' ' {
            Some(line.len() - 1 - value.0)
        } else {
            None
        }
    });

    match iterator.next() {
        Some(idx) => idx,
        None => line.len().max(1) - 1,
    }
}

fn first_not_whitespace(line: &str) -> usize {
    let mut iterator =
        line.chars()
            .enumerate()
            .filter_map(|value| if value.1 != ' ' { Some(value.0) } else { None });

    iterator.next().unwrap_or_default()
}

fn no_word_after(line: &str, col: usize) -> bool {
    let iterator = line.chars().enumerate().skip(col);

    let mut prev = ' ';
    for (idx, char) in iterator {
        if (is_word_delimiter(char) && !is_word_delimiter(prev) || (char != ' ' && prev == ' '))
            && idx != col
        {
            return false;
        }

        prev = char;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_down() {
        let mut buf = Buffer::new(String::from("test.txt"));
        assert_eq!(down(&buf), Position { row: 1, col: 0 });

        buf.cursor = Position { row: 2, col: 56 };
        assert_eq!(down(&buf), Position { row: 3, col: 37 });

        buf.cursor = Position { row: 3, col: 0 };
        assert_eq!(down(&buf), buf.cursor);
    }

    #[test]
    fn cursor_up() {
        let mut buf = Buffer::new(String::from("test.txt"));

        buf.cursor = Position { row: 1, col: 28 };
        assert_eq!(up(&buf), Position { row: 0, col: 22 });

        buf.cursor = Position { row: 0, col: 0 };
        assert_eq!(up(&buf), buf.cursor);
    }

    #[test]
    fn cursor_left() {
        let mut buf = Buffer::new(String::from("test.txt"));

        buf.cursor = Position { row: 0, col: 1 };
        assert_eq!(left(&buf), Position { row: 0, col: 0 });

        buf.cursor = Position { row: 1, col: 0 };
        assert_eq!(left(&buf), buf.cursor);
    }

    #[test]
    fn cursor_right() {
        let mut buf = Buffer::new(String::from("test.txt"));

        assert_eq!(right(&buf), Position { row: 0, col: 1 });

        buf.cursor = Position { row: 0, col: 22 };
        assert_eq!(right(&buf), buf.cursor);
    }

    #[test]
    fn cursor_end() {
        let buf = Buffer::new(String::from("test.txt"));

        assert_eq!(end_line(&buf), Position { row: 0, col: 22 });
    }

    #[test]
    fn cursor_start() {
        let mut buf = Buffer::new(String::from("test.txt"));
        buf.cursor = Position { row: 0, col: 15 };

        assert_eq!(start_line(&buf), Position { row: 0, col: 0 });
    }

    #[test]
    fn cursor_word_end() {
        let mut buf = Buffer::new(String::from("test.txt"));
        assert_eq!(word_end(&buf), Position { row: 0, col: 4 });

        buf.cursor = Position { row: 0, col: 17 };
        buf.cursor = word_end(&buf);
        assert_eq!(buf.cursor, Position { row: 0, col: 20 });

        buf.cursor = word_end(&buf);
        assert_eq!(buf.cursor, Position { row: 0, col: 21 });

        // new line
        assert_eq!(word_end(&buf), Position { row: 1, col: 11 });
    }

    #[test]
    fn cursor_prev_word_start() {
        let mut buf = Buffer::new(String::from("test.txt"));
        assert_eq!(prev_word_start(&buf), Position { row: 0, col: 0 });

        buf.cursor = Position { row: 0, col: 6 };
        assert_eq!(prev_word_start(&buf), Position { row: 0, col: 0 });

        buf.cursor = Position { row: 1, col: 0 };
        assert_eq!(prev_word_start(&buf), Position { row: 0, col: 21 });
    }

    #[test]
    fn cursor_word_start() {
        let mut buf = Buffer::new(String::from("test.txt"));
        assert_eq!(word_start(&buf), Position { row: 0, col: 6 });

        buf.cursor = Position { row: 0, col: 17 };
        assert_eq!(word_start(&buf), Position { row: 0, col: 21 });

        buf.cursor = Position { row: 0, col: 21 };
        assert_eq!(word_start(&buf), Position { row: 1, col: 0 });
    }

    #[test]
    fn first_not_whitespace_test() {
        let line = " This is a string";
        assert_eq!(first_not_whitespace(line), 1);
    }

    #[test]
    fn last_not_whitespace_test() {
        let line = "This is a string ";
        assert_eq!(last_not_whitespace(line), 15);
    }

    #[test]
    fn word_after_test() {
        let line = "This is a string ";
        assert!(no_word_after(line, 12));

        let line = "This is a string with a whitespace at the end, ";
        assert!(!no_word_after(line, 42));
    }
}
