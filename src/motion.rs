use crate::{
    context::{Context, Position},
    navigation::{move_down, move_left, move_right, move_up},
};

pub enum Motion {
    Left,
    Right,
    Up,
    Down,
}

impl Motion {
    pub fn new(key: &str) -> Option<Self> {
        match key {
            "h" => Some(Motion::Left),
            "j" => Some(Motion::Down),
            "k" => Some(Motion::Up),
            "l" => Some(Motion::Right),
            _ => None,
        }
    }

    pub fn execute(self, cx: &mut Context) -> Position {
        match self {
            Motion::Left => move_left(cx),
            Motion::Down => move_down(cx),
            Motion::Up => move_up(cx),
            Motion::Right => move_right(cx),
        }
    }
}
