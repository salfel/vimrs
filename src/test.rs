use crate::{actions::Action, buffer::Buffer, motion::Motion};

#[derive(Debug, PartialEq)]
pub enum Event {
    Motion(Motion),
    Action(Action),
}

pub(crate) fn assert_count<T>(array: &[T], count: usize) {
    assert_eq!(array.len(), count);
}

pub(crate) fn assert_event(buf: &Buffer, event: Event) {
    assert!(buf.events.contains(&event));
}
