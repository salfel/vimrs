use std::cmp::min;

use crate::{
    context::{Context, Position},
    mode::Mode,
};

pub fn right(cx: &mut Context) -> Position {
    let mut row_len = cx.row(cx.cursor.row).len();
    if let Mode::Normal = cx.mode {
        row_len -= 1;
    }

    if cx.cursor.col >= row_len {
        if cx.cursor.row < cx.content.len() - 1 {
            Position {
                row: cx.cursor.row + 1,
                col: 0,
            }
        } else {
            cx.cursor
        }
    } else {
        Position {
            row: cx.cursor.row,
            col: cx.cursor.col + 1,
        }
    }
}

pub fn left(cx: &mut Context) -> Position {
    if cx.cursor.col == 0 {
        if cx.cursor.row == 0 {
            cx.cursor
        } else {
            Position {
                row: cx.cursor.row - 1,
                col: cx.row(cx.cursor.row - 1).len().max(1) - 1,
            }
        }
    } else {
        Position {
            row: cx.cursor.row,
            col: cx.cursor.col - 1,
        }
    }
}

pub fn up(cx: &mut Context) -> Position {
    if cx.cursor.row == 0 {
        return cx.cursor;
    }

    let prev_row = cx
        .content
        .get(cx.cursor.row - 1)
        .unwrap_or_else(|| panic!("row: {} doesn't exist", cx.cursor.row));

    Position {
        row: cx.cursor.row - 1,
        col: min(cx.cursor.col, prev_row.len().max(1) - 1),
    }
}

pub fn down(cx: &mut Context) -> Position {
    if cx.cursor.row >= cx.content.len() - 1 {
        return cx.cursor;
    }

    let next_row = cx
        .content
        .get(cx.cursor.row + 1)
        .unwrap_or_else(|| panic!("row: {}, doesn't exist", cx.cursor.row));

    Position {
        row: cx.cursor.row + 1,
        col: min(cx.cursor.col, next_row.len().max(1) - 1),
    }
}

pub fn start_line(cx: &mut Context) -> Position {
    Position {
        row: cx.cursor.row,
        col: 0,
    }
}

pub fn end_line(cx: &mut Context) -> Position {
    if let Some(row) = cx.content.get(cx.cursor.row) {
        Position {
            row: cx.cursor.row,
            col: row.len().max(1) - 1,
        }
    } else {
        cx.cursor
    }
}

const WORD_DELIMITER: [char; 10] = [' ', '(', ')', '[', ']', '{', '}', '$', '^', '!'];

pub fn end_word(cx: &mut Context) -> Position {
    match cx.content.get(cx.cursor.row) {
        Some(line) => {
            let iterator = line.chars().enumerate().skip(cx.cursor.col);

            let mut prev = 'a';
            for (idx, char) in iterator {
                if WORD_DELIMITER.contains(&char)
                    && !WORD_DELIMITER.contains(&prev)
                    && idx - 1 != cx.cursor.col
                {
                    return Position {
                        row: cx.cursor.row,
                        col: idx - 1,
                    };
                }

                prev = char;
            }

            Position {
                row: cx.cursor.row,
                col: last_not_delimiter(line),
            }
        }
        None => cx.cursor,
    }
}

pub fn start_word(cx: &mut Context) -> Position {
    match cx.content.get(cx.cursor.row) {
        Some(line) => {
            let iterator = line
                .chars()
                .rev()
                .enumerate()
                .skip(line.len() - 1 - cx.cursor.col);

            let mut prev = ' ';
            for (idx, char) in iterator {
                if WORD_DELIMITER.contains(&char)
                    && !WORD_DELIMITER.contains(&prev)
                    && line.len() - idx != cx.cursor.col
                {
                    return Position {
                        row: cx.cursor.row,
                        col: line.len() - idx,
                    };
                }

                prev = char;
            }

            Position {
                row: cx.cursor.row,
                col: first_not_delimiter(line),
            }
        }
        None => cx.cursor,
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
