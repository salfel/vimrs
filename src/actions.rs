use delete::{delete_end, delete_line, delete_motion};

use crate::{buffer::Buffer, mode::Mode, motion::Motion, utils::split_first_char};

pub mod delete;

#[derive(Debug, PartialEq)]
pub enum Action {
    Delete { motion: Motion },
    DeleteLine,
    DeleteEnd,
    Change { motion: Motion },
}

impl Action {
    pub fn new(keys: &str) -> Option<Self> {
        let (prefix, keys) = split_first_char(keys);
        let action = match prefix {
            'd' => match keys.as_str() {
                "d" => Some(Action::DeleteLine),
                keys => Motion::new(keys).map(|motion| Action::Delete { motion }),
            },
            'c' => Motion::new(&keys).map(|motion| Action::Change { motion }),
            'D' => Some(Action::DeleteEnd),
            _ => None,
        };
        action
    }

    pub fn execute(self, buf: &mut Buffer) {
        match self {
            Action::Delete { motion } => {
                #[cfg(test)]
                buf.events
                    .push(crate::test::Event::Action(Action::Delete { motion }));

                delete_motion(buf, motion);
            }
            Action::Change { motion } => {
                delete_motion(buf, motion);
                buf.change_mode(Mode::Insert);
            }
            Action::DeleteLine => {
                delete_line(buf);
            }
            Action::DeleteEnd => {
                delete_end(buf);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::Event;

    use super::*;

    #[test]
    fn check_executed() {
        let mut buf = Buffer::new(String::from("test.txt"));

        Action::new("d$").unwrap().execute(&mut buf);
        assert_eq!(
            buf.events,
            vec![
                Event::Action(Action::Delete {
                    motion: Motion::LineEnd
                }),
                Event::Motion(Motion::LineEnd)
            ]
        );
    }

    #[test]
    fn creates_multiple_char_action() {
        let action = Action::new("dd");
        assert_eq!(action, Some(Action::DeleteLine));
    }
}
