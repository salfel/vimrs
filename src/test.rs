use crate::motion::Motion;

#[cfg(test)]
use crate::buffer::Buffer;

#[derive(Debug, PartialEq)]
pub enum Event {
    Motion(Motion),
}

pub(crate) fn assert_count<T>(array: &Vec<T>, count: usize) {
    assert_eq!(array.len(), count);
}

#[cfg(test)]
pub(crate) fn assert_event(buf: &Buffer, event: Event) {
    assert!(buf.events.contains(&event));
}
