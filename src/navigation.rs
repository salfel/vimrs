use crate::context::{Context, Position};

pub fn move_right(cx: &mut Context) -> Position {
    match cx.content.get(cx.cursor.row) {
        Some(line) if cx.cursor.col < line.len() - 1 => Position {
            row: cx.cursor.row,
            col: cx.cursor.col + 1,
        },
        _ => cx.cursor,
    }
}

pub fn move_left(cx: &mut Context) -> Position {
    if cx.cursor.col == 0 {
        cx.cursor
    } else {
        Position {
            row: cx.cursor.row,
            col: cx.cursor.col - 1,
        }
    }
}
