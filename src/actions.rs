use delete::delete_motion;

use crate::{buffer::Buffer, motion::Motion, utils::split_first_char};

pub mod delete;

pub enum Action {
    Delete { keys: String },
    Change { keys: String },
}

impl Action {
    pub fn new(keys: &str) -> Option<Self> {
        let (prefix, keys) = split_first_char(keys);
        match prefix {
            'd' => Some(Action::Delete { keys }),
            'c' => Some(Action::Change { keys }),
            _ => None,
        }
    }

    #[allow(clippy::single_match)]
    pub fn execute(self, buf: &mut Buffer) {
        match self {
            Action::Delete { keys } => {
                if let Some(motion) = Motion::new(&keys) {
                    delete_motion(buf, motion);
                }
            }
            _ => {}
        }
    }
}
