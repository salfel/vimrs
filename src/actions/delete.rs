use std::cmp::{max, min};

use crate::{buffer::Buffer, motion::Motion};

pub fn delete_motion(buf: &mut Buffer, motion: Motion) {
    let from = buf.cursor;
    let end = motion.execute(buf);

    if from.row == end.row {
        let min = min(from.col, end.col);
        let mut max = max(from.col, end.col);

        if motion.inclusive() {
            max -= 1;
        }

        let line = buf
            .content
            .get_mut(from.row)
            .unwrap_or_else(|| panic!("row: {} not found", from.row));

        line.replace_range(min..max + 1, "");
        buf.cursor.col = min;
    } else {
        let min = min(from.row, end.row);
        let max = max(from.row, end.row);

        for idx in min..max + 1 {
            buf.content.remove(idx);
        }
    }
}

pub fn delete_line(buf: &mut Buffer) {
    buf.content.remove(buf.cursor.row);
    buf.cursor.row = min(buf.cursor.row, buf.content.len() - 1);
}

pub fn delete_end(buf: &mut Buffer) {
    let line = &mut buf.content[buf.cursor.row];
    line.replace_range(buf.cursor.col.., "");
    buf.cursor.col = min(buf.cursor.col, max(line.len(), 1) - 1);
}

#[cfg(test)]
mod tests {
    use crate::{buffer::Position, test::assert_count};

    use super::*;

    #[test]
    fn test_delete_motion() {
        let mut buf = Buffer::new(String::from("test.txt"));

        delete_motion(&mut buf, Motion::LineEnd);
        assert_eq!(buf.content[0], String::new());

        buf.cursor = Position { row: 1, col: 0 };
        delete_motion(&mut buf, Motion::StartWord);
        assert_eq!(buf.content[1], String::from("adipiscing elit. "));

        buf.cursor = Position { row: 3, col: 13 };
        delete_motion(&mut buf, Motion::PrevWordStart);
        assert_eq!(
            buf.content[3],
            String::from("Mauris semper varius eros morbi.")
        );

        assert_eq!(buf.content.len(), 7);
        buf.cursor = Position { row: 2, col: 0 };
        delete_motion(&mut buf, Motion::Down);
        assert_eq!(buf.content.len(), 5);
    }

    #[test]
    fn test_delete_line() {
        let mut buf = Buffer::new(String::from("test.txt"));

        delete_line(&mut buf);
        assert_count(&buf.content, 6);
        assert_eq!(
            buf.content[0],
            String::from("consectetuer adipiscing elit. ")
        );

        buf.cursor.row = buf.content.len() - 1;
        delete_line(&mut buf);
        assert_count(&buf.content, 5);
        assert_eq!(buf.cursor.row, 4);
    }

    #[test]
    fn test_delete_end() {
        let mut buf = Buffer::new(String::from("test.txt"));

        delete_end(&mut buf);
        assert_eq!(buf.content[0], String::new());

        buf.cursor = Position { row: 1, col: 13 };
        delete_end(&mut buf);
        assert_eq!(buf.content[1], String::from("consectetuer "));
    }
}
