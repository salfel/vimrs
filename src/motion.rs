use crate::{
    context::{Context, Position},
    navigation::{move_left, move_right},
};

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
