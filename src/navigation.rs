use std::cmp::min;

use crate::{
    buffer::{Buffer, Position},
    mode::Mode,
};

pub fn right(buf: &mut Buffer) -> Position {
    let mut row_len = buf.row(buf.cursor.row).len();
    if let Mode::Normal = buf.mode {
        row_len -= 1;
    }

    if buf.cursor.col >= row_len {
        if buf.cursor.row < buf.content.len() - 1 {
            Position {
                row: buf.cursor.row + 1,
                col: 0,
            }
        } else {
            buf.cursor
        }
    } else {
        Position {
            row: buf.cursor.row,
            col: buf.cursor.col + 1,
        }
    }
}

pub fn left(buf: &mut Buffer) -> Position {
    if buf.cursor.col == 0 {
        if buf.cursor.row == 0 {
            buf.cursor
        } else {
            Position {
                row: buf.cursor.row - 1,
                col: buf.row(buf.cursor.row - 1).len().max(1) - 1,
            }
        }
    } else {
        Position {
            row: buf.cursor.row,
            col: buf.cursor.col - 1,
        }
    }
}

pub fn up(buf: &mut Buffer) -> Position {
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

pub fn down(buf: &mut Buffer) -> Position {
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

pub fn start_line(buf: &mut Buffer) -> Position {
    Position {
        row: buf.cursor.row,
        col: 0,
    }
}

pub fn end_line(buf: &mut Buffer) -> Position {
    if let Some(row) = buf.content.get(buf.cursor.row) {
        Position {
            row: buf.cursor.row,
            col: row.len().max(1) - 1,
        }
    } else {
        buf.cursor
    }
}

const WORD_DELIMITER: [char; 10] = [' ', '(', ')', '[', ']', '{', '}', '$', '^', '!'];

pub fn end_word(buf: &mut Buffer) -> Position {
    match buf.content.get(buf.cursor.row) {
        Some(line) => {
            let iterator = line.chars().enumerate().skip(buf.cursor.col);

            let mut prev = 'a';
            for (idx, char) in iterator {
                if WORD_DELIMITER.contains(&char)
                    && !WORD_DELIMITER.contains(&prev)
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
                col: last_not_delimiter(line),
            }
        }
        None => buf.cursor,
    }
}

pub fn start_word(buf: &mut Buffer) -> Position {
    match buf.content.get(buf.cursor.row) {
        Some(line) => {
            let iterator = line
                .chars()
                .rev()
                .enumerate()
                .skip(line.len() - 1 - buf.cursor.col);

            let mut prev = ' ';
            for (idx, char) in iterator {
                if WORD_DELIMITER.contains(&char)
                    && !WORD_DELIMITER.contains(&prev)
                    && line.len() - idx != buf.cursor.col
                {
                    return Position {
                        row: buf.cursor.row,
                        col: line.len() - idx,
                    };
                }

                prev = char;
            }

            Position {
                row: buf.cursor.row,
                col: first_not_delimiter(line),
            }
        }
        None => buf.cursor,
    }
}

fn first_not_delimiter(line: &str) -> usize {
    let iterator = line.chars().enumerate();

    for (idx, char) in iterator {
        if !WORD_DELIMITER.contains(&char) {
            return idx;
        }
    }

    line.len().max(1) - 1
}

fn last_not_delimiter(line: &str) -> usize {
    let iterator = line.chars().rev().enumerate();

    for (idx, char) in iterator {
        if !WORD_DELIMITER.contains(&char) {
            return line.len() - 1 - idx;
        }
    }

    0
}
