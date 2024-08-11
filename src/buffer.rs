use crate::context::Context;

pub struct Buffer {
    pub filename: String,
    pub context: Context,
}

impl Buffer {
    pub fn new(filename: String, content: String) -> Self {
        Buffer {
            filename,
            context: Context::new(content),
        }
    }
}
