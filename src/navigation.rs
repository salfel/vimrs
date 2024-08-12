use std::cmp::min;

use crate::context::{Context, Position};

pub fn right(cx: &mut Context) -> Position {
    if cx.cursor.col >= cx.row(cx.cursor.row).len() - 1 {
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
                col: cx.row(cx.cursor.row - 1).len() - 1,
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
        col: min(cx.cursor.col, prev_row.len() - 1),
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
        col: min(cx.cursor.col, next_row.len() - 1),
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
            col: row.len() - 1,
        }
    } else {
        cx.cursor
    }
}
