use delete::delete_motion;

use crate::{buffer::Buffer, motion::Motion, utils::split_first_char};

pub mod delete;

#[derive(Debug, PartialEq)]
pub enum Action {
    Delete { motion: Motion },
    Change { motion: Motion },
}

impl Action {
    pub fn new(keys: &str) -> Option<Self> {
        let (prefix, keys) = split_first_char(keys);
        match prefix {
            'd' => Motion::new(&keys).map(|motion| Action::Delete { motion }),
            'c' => Motion::new(&keys).map(|motion| Action::Change { motion }),
            _ => None,
        }
    }

    #[allow(clippy::single_match)]
    pub fn execute(self, buf: &mut Buffer) {
        match self {
            Action::Delete { motion } => {
                #[cfg(test)]
                buf.events
                    .push(crate::test::Event::Action(Action::Delete { motion }));

                delete_motion(buf, motion);
            }
            _ => {}
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
}
