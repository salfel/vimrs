use crate::buffer::Buffer;

#[derive(Debug, PartialEq)]
pub struct Word {
    content: String,
    start: usize,
    end: usize,
    row: usize,
}

impl Word {
    pub fn new(content: String, start: usize, end: usize, row: usize) -> Self {
        Word {
            content,
            start,
            end,
            row,
        }
    }
}

const DELIMITERS: [char; 21] = [
    '(', ')', '[', ']', '{', '}', '$', '?', '^', '&', '=', '-', '/', '\\', '-', ',', '.', '\'',
    '"', '>', '<',
];

pub fn parse_words(buf: &Buffer) -> Vec<Word> {
    let iterator = buf.content[buf.cursor.row].chars().enumerate();
    let mut words = Vec::new();

    let mut start = 0;
    let mut chars = String::new();
    for (idx, char) in iterator {
        if char == ' ' && !chars.is_empty() {
            words.push(Word {
                content: chars,
                row: buf.cursor.row,
                start,
                end: idx - 1,
            });

            chars = String::new();
        } else if DELIMITERS.contains(&char) {
            words.push(Word {
                content: chars,
                row: buf.cursor.row,
                start,
                end: idx - 1,
            });

            chars = String::from(char);
            start = idx;
        } else {
            if chars.is_empty() {
                start = idx;
            }

            chars.push(char);
        }
    }

    if !chars.is_empty() {
        words.push(Word::new(
            chars,
            start,
            buf.content[buf.cursor.row].len() - 1,
            buf.cursor.row,
        ));
    }

    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_words() {
        let buf = Buffer::new(String::from("test.txt"));

        assert_eq!(
            parse_words(&buf),
            vec![
                Word::new(String::from("Lorem"), 0, 4, 0),
                Word::new(String::from("ipsum"), 6, 10, 0),
                Word::new(String::from("odor"), 12, 15, 0),
                Word::new(String::from("amet"), 17, 20, 0),
                Word::new(String::from(","), 21, 21, 0),
            ]
        );
    }
}
