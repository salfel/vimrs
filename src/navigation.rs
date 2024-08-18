use std::cmp::{max, min};

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
        col: min(buf.cursor.col, max(1, prev_row.len()) - 1),
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
        col: min(buf.cursor.col, max(next_row.len(), 1) - 1),
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
            col: max(1, row.len()) - 1,
        }
    } else {
        buf.cursor
    }
}

const WORD_DELIMITERS: [char; 11] = ['(', ')', '[', ']', '{', '}', '$', '^', '!', '.', ','];

#[allow(clippy::iter_skip_zero)]
pub fn word_end(buf: &Buffer) -> Position {
    let mut prev = 'a';
    let row_iterator = buf.content.iter().enumerate().skip(buf.cursor.row);

    for (row, line) in row_iterator {
        let iterator = if buf.cursor.row == row {
            let mut iterator = line.chars().enumerate().skip(buf.cursor.col + 1);

            if let Some((_, char)) = iterator.next() {
                prev = char;
            }

            iterator
        } else {
            line.chars().enumerate().skip(0)
        };

        for (idx, char) in iterator {
            if (buf.cursor.row != row || buf.cursor.col != max(idx, 1) - 1)
                && ((is_word_delimiter(char) && !is_word_delimiter(prev))
                    || (is_word_delimiter(prev) && !is_word_delimiter(char) && char != ' ')
                    || (char == ' ' && prev != ' '))
            {
                return Position {
                    row,
                    col: max(idx, 1) - 1,
                };
            }

            prev = char;
        }

        if buf.cursor.col != last_not_whitespace(line) && !line.is_empty() {
            return Position {
                row,
                col: last_not_whitespace(line),
            };
        }

        prev = ' ';
    }

    Position {
        row: buf.content.len() - 1,
        col: last_not_whitespace(buf.content.last().unwrap()),
    }
}

#[allow(clippy::iter_skip_zero, non_snake_case)]
pub fn prev_word_start(buf: &Buffer) -> Position {
    let mut prev = 'a';
    let row_iterator = buf
        .content
        .iter()
        .rev()
        .enumerate()
        .skip(buf.content.len() - 1 - buf.cursor.row)
        .map(|value| (buf.content.len() - 1 - value.0, value.1));

    for (row, line) in row_iterator {
        let iterator = if buf.cursor.row == row {
            line.chars()
                .rev()
                .enumerate()
                .skip(line.len() - buf.cursor.col)
        } else {
            line.chars().rev().enumerate().skip(0)
        }
        .map(|value| (line.len() - 1 - value.0, value.1));

        for (idx, char) in iterator {
            if (buf.cursor.row != row || buf.cursor.col != idx + 1)
                && ((is_word_delimiter(char) && !is_word_delimiter(prev) && prev != ' ')
                    || (!is_word_delimiter(char) && is_word_delimiter(prev))
                    || (char == ' ' && prev != ' '))
            {
                return Position { row, col: idx + 1 };
            }

            prev = char;
        }

        if buf.cursor.col != first_not_whitespace(line) {
            return Position {
                row,
                col: first_not_whitespace(line),
            };
        } else if line.is_empty() && buf.cursor.row != row {
            return Position { row, col: 0 };
        }

        prev = ' ';
    }

    Position {
        row: 0,
        col: first_not_whitespace(&buf.content[0]),
    }
}

#[allow(clippy::iter_skip_zero)]
pub fn word_start(buf: &Buffer) -> Position {
    let mut prev = 'a';
    let row_iterator = buf.content.iter().enumerate().skip(buf.cursor.row);

    for (row, line) in row_iterator {
        let iterator = if buf.cursor.row == row {
            let mut iterator = line.chars().enumerate().skip(buf.cursor.col);

            if let Some((_, char)) = iterator.next() {
                prev = char;
            }

            iterator
        } else {
            line.chars().enumerate().skip(0)
        };

        for (idx, char) in iterator {
            if (buf.cursor.row != row || buf.cursor.col != idx)
                && ((is_word_delimiter(char) && !is_word_delimiter(prev))
                    || (is_word_delimiter(prev) && !is_word_delimiter(char) && char != ' ')
                    || (char != ' ' && prev == ' '))
            {
                return Position { row, col: idx };
            }

            prev = char;
        }

        if line.is_empty() && buf.cursor.row != row {
            return Position { row, col: 0 };
        }

        prev = ' ';
    }

    Position {
        row: buf.content.len() - 1,
        col: last_not_whitespace(buf.content.last().unwrap()),
    }
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
        None => max(line.len(), 1) - 1,
    }
}

fn first_not_whitespace(line: &str) -> usize {
    let mut iterator =
        line.chars()
            .enumerate()
            .filter_map(|value| if value.1 != ' ' { Some(value.0) } else { None });

    iterator.next().unwrap_or_default()
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

        buf.cursor = Position { row: 6, col: 0 };
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

        buf.cursor = Position { row: 3, col: 36 };
        buf.cursor = word_end(&buf);
        assert_eq!(buf.cursor, Position { row: 3, col: 37 });
        assert_eq!(word_end(&buf), Position { row: 4, col: 4 });

        buf.cursor = Position { row: 4, col: 0 };
        buf.cursor = word_end(&buf);
        assert_eq!(buf.cursor, Position { row: 4, col: 4 });
        buf.cursor = word_end(&buf);
        assert_eq!(buf.cursor, Position { row: 4, col: 5 });
        buf.cursor = word_end(&buf);
        assert_eq!(buf.cursor, Position { row: 4, col: 10 });

        buf.cursor = Position { row: 4, col: 10 };
        assert_eq!(word_end(&buf), Position { row: 6, col: 4 })
    }

    #[test]
    fn cursor_prev_word_start() {
        let mut buf = Buffer::new(String::from("test.txt"));
        assert_eq!(prev_word_start(&buf), Position { row: 0, col: 0 });

        buf.cursor = Position { row: 0, col: 6 };
        assert_eq!(prev_word_start(&buf), Position { row: 0, col: 0 });

        buf.cursor = Position { row: 0, col: 14 };
        assert_eq!(prev_word_start(&buf), Position { row: 0, col: 12 });

        buf.cursor = Position { row: 1, col: 0 };
        assert_eq!(prev_word_start(&buf), Position { row: 0, col: 21 });

        buf.cursor = Position { row: 4, col: 10 };
        buf.cursor = prev_word_start(&buf);
        assert_eq!(buf.cursor, Position { row: 4, col: 6 });
        buf.cursor = prev_word_start(&buf);
        assert_eq!(buf.cursor, Position { row: 4, col: 5 });
        assert_eq!(prev_word_start(&buf), Position { row: 4, col: 0 });

        buf.cursor = Position { row: 6, col: 0 };
        buf.cursor = prev_word_start(&buf);
        assert_eq!(buf.cursor, Position { row: 5, col: 0 });
        assert_eq!(prev_word_start(&buf), Position { row: 4, col: 6 });
    }

    #[test]
    fn cursor_word_start() {
        let mut buf = Buffer::new(String::from("test.txt"));
        assert_eq!(word_start(&buf), Position { row: 0, col: 6 });

        buf.cursor = Position { row: 0, col: 17 };
        assert_eq!(word_start(&buf), Position { row: 0, col: 21 });

        buf.cursor = Position { row: 0, col: 21 };
        assert_eq!(word_start(&buf), Position { row: 1, col: 0 });

        buf.cursor = Position { row: 4, col: 0 };
        buf.cursor = word_start(&buf);
        assert_eq!(buf.cursor, Position { row: 4, col: 5 });

        buf.cursor = Position { row: 4, col: 5 };
        buf.cursor = word_start(&buf);
        assert_eq!(buf.cursor, Position { row: 4, col: 6 });
        buf.cursor = word_start(&buf);
        assert_eq!(buf.cursor, Position { row: 5, col: 0 });
        buf.cursor = word_start(&buf);
        assert_eq!(buf.cursor, Position { row: 6, col: 0 });
        assert_eq!(word_start(&buf), Position { row: 6, col: 4 });
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
}
