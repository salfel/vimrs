use std::cmp::{max, min};

use crate::{
    buffer::{Buffer, Position},
    motion::Motion,
};

pub fn delete_motion(buf: &mut Buffer, motion: Motion) {
    let from = buf.cursor;
    let end = if motion.inclusive() {
        let pos = motion.execute(buf);
        Position {
            row: pos.row,
            col: pos.col - 1,
        }
    } else {
        motion.execute(buf)
    };

    // TODO implement a way to generate a min and max, to resolve range being in wrong direction
    if from.row == end.row {
        let line = buf
            .content
            .get_mut(from.row)
            .unwrap_or_else(|| panic!("row: {} not found", from.row));

        line.replace_range(from.col..end.col + 1, "")
    } else {
        let min = min(from.row, end.row);
        let max = max(from.row, end.row);

        for idx in min..max + 1 {
            buf.content.remove(idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::Position;

    use super::*;

    #[test]
    fn test_delete_motion() {
        let mut buf = Buffer::new(String::from("test.txt"));

        delete_motion(&mut buf, Motion::LineEnd);
        assert_eq!(buf.content[0], String::new());

        buf.cursor = Position { row: 1, col: 0 };
        delete_motion(&mut buf, Motion::StartWord);
        assert_eq!(buf.content[1], String::from("adipiscing elit. "));

        assert_eq!(buf.content.len(), 7);
        buf.cursor = Position { row: 2, col: 0 };
        delete_motion(&mut buf, Motion::Down);
        assert_eq!(buf.content.len(), 5);
    }
}
