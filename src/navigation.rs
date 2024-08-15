use std::cmp::min;

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

const WORD_DELIMITERS: [char; 12] = ['(', ')', '[', ']', '{', '}', '$', '^', '!', '.', ',', ' '];

pub fn end_word(buf: &Buffer) -> Position {
    let line = buf.row(buf.cursor.row);
    let mut iterator = line.chars().enumerate().skip(buf.cursor.col);

    let mut prev = match iterator.next() {
        Some(item) => item.1,
        None => return buf.cursor,
    };
    for (idx, char) in iterator {
        if ((is_word_delimiter(char) && !is_word_delimiter(prev)) || (char == ' ' && prev != ' '))
            && idx - 1 != buf.cursor.col
        {
            return Position {
                row: buf.cursor.row,
                col: idx - 1,
            };
        }

        prev = char;
    }

    Position {
        row: buf.cursor.row,
        col: line.len().max(1) - 1,
    }
}

pub fn start_word(buf: &Buffer) -> Position {
    let mut line = buf.row(buf.cursor.row);
    let mut cursor = buf.cursor;
    let mut skip = line.len() - cursor.col;

    if cursor.col == 0 {
        if cursor.row == 0 {
            return Position { row: 0, col: 0 };
        }

        cursor.row -= 1;
        line = buf.row(cursor.row);
        skip = 0;
    }

    let mut iterator = line.chars().rev().enumerate().skip(skip);
    let mut prev = match iterator.next() {
        Some(item) => item.1,
        None => return cursor,
    };

    for (idx, char) in iterator {
        if ((is_word_delimiter(char) && (!is_word_delimiter(prev))) || (char == ' ' && prev != ' '))
            && line.len() - idx != cursor.col
        {
            return Position {
                row: cursor.row,
                col: line.len() - idx,
            };
        }

        prev = char;
    }

    Position {
        row: cursor.row,
        col: 0,
    }
}

fn is_word_delimiter(char: char) -> bool {
    WORD_DELIMITERS.contains(&char)
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
        assert_eq!(up(&buf), Position { row: 0, col: 21 });

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

        buf.cursor = Position { row: 0, col: 21 };
        assert_eq!(right(&buf), buf.cursor);
    }

    #[test]
    fn cursor_end() {
        let buf = Buffer::new(String::from("test.txt"));

        assert_eq!(end_line(&buf), Position { row: 0, col: 21 });
    }

    #[test]
    fn cursor_start() {
        let mut buf = Buffer::new(String::from("test.txt"));
        buf.cursor = Position { row: 0, col: 15 };

        assert_eq!(start_line(&buf), Position { row: 0, col: 0 });
    }
}
