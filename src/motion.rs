use crate::context::{Context, Position};

pub enum Motion {
    Left,
    Right,
}

impl Motion {
    pub fn new(key: &str) -> Option<Self> {
        match key {
            "l" => Some(Motion::Right),
            "h" => Some(Motion::Left),
            _ => None,
        }
    }

    pub fn execute(self, cx: &mut Context) -> Position {
        match self {
            Motion::Right => move_right(cx),
            Motion::Left => move_left(cx),
        }
    }
}

fn move_right(cx: &mut Context) -> Position {
    match cx.content.get(cx.cursor.row) {
        Some(line) if cx.cursor.col < line.len() - 1 => Position {
            row: cx.cursor.row,
            col: cx.cursor.col + 1,
        },
        _ => cx.cursor,
    }
}

fn move_left(cx: &mut Context) -> Position {
    if cx.cursor.col == 0 {
        cx.cursor
    } else {
        Position {
            row: cx.cursor.row,
            col: cx.cursor.col - 1,
        }
    }
}
