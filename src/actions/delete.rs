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

        buf.register.set_default(line[min..max + 1].to_string());
        line.replace_range(min..max + 1, "");
        buf.cursor.col = min;
    } else {
        let min = min(from.row, end.row);
        let max = max(from.row, end.row);

        let mut data = String::from("\n");

        for _ in min..max + 1 {
            data.push_str(&format!("{}\n", buf.content.remove(min)));
        }
        buf.register.set_default(data);
    }
}

pub fn delete_line(buf: &mut Buffer) {
    let line = format!("\n{}\n", buf.content.remove(buf.cursor.row));
    println!("{}", line);
    buf.register.set_default(line);
    buf.cursor.row = min(buf.cursor.row, buf.content.len() - 1);
}

pub fn delete_end(buf: &mut Buffer) {
    let line = &mut buf.content[buf.cursor.row];
    buf.register.set_default(line[buf.cursor.col..].to_string());
    line.replace_range(buf.cursor.col.., "");
    buf.cursor.col = min(buf.cursor.col, max(line.len(), 1) - 1);
}

#[cfg(test)]
mod tests {
    use crate::buffer::Position;

    use super::*;

    #[test]
    fn test_delete_motion() {
        let mut buf = Buffer::test(String::from("test.txt"));

        delete_motion(&mut buf, Motion::LineEnd);
        assert_eq!(buf.content[0], String::new());
        assert_eq!(
            buf.register.get_default(),
            String::from("Lorem ipsum odor amet, ")
        );

        buf.cursor = Position { row: 1, col: 0 };
        delete_motion(&mut buf, Motion::StartWord);
        assert_eq!(buf.content[1], String::from("adipiscing elit. "));
        assert_eq!(buf.register.get_default(), String::from("consectetuer "));

        buf.cursor = Position { row: 3, col: 13 };
        delete_motion(&mut buf, Motion::PrevWordStart);
        assert_eq!(
            buf.content[3],
            String::from("Mauris semper varius eros morbi.")
        );
        assert_eq!(buf.register.get_default(), String::from("vitae "));

        assert_eq!(buf.content.len(), 7);
        for line in buf.content.iter() {
            println!("{}", line);
        }

        buf.cursor = Position { row: 2, col: 0 };
        delete_motion(&mut buf, Motion::Down);
        println!();

        for line in buf.content.iter() {
            println!("{}", line);
        }
        assert_eq!(buf.content.len(), 5);
        assert_eq!(
            buf.register.get_default(),
            String::from(
                "
Ridiculus nulla consectetur proin purus ad justo nullam. 
Mauris semper varius eros morbi.
"
            )
        );
    }

    #[test]
    fn test_delete_line() {
        let mut buf = Buffer::test(String::from("test.txt"));

        delete_line(&mut buf);
        assert_eq!(buf.content.len(), 6);
        assert_eq!(
            buf.content[0],
            String::from("consectetuer adipiscing elit. ")
        );
        assert_eq!(
            buf.register.get_default(),
            String::from("\nLorem ipsum odor amet, \n")
        );

        buf.cursor.row = buf.content.len() - 1;
        delete_line(&mut buf);
        assert_eq!(buf.content.len(), 5);
        assert_eq!(buf.cursor.row, 4);
        assert_eq!(buf.register.get_default(), String::from("\nnulla\n"));
    }

    #[test]
    fn test_delete_end() {
        let mut buf = Buffer::test(String::from("test.txt"));

        delete_end(&mut buf);
        assert_eq!(buf.content[0], String::new());

        buf.cursor = Position { row: 1, col: 13 };
        delete_end(&mut buf);
        assert_eq!(buf.content[1], String::from("consectetuer "));
        assert_eq!(
            buf.register.get_default(),
            String::from("adipiscing elit. ")
        )
    }
}
