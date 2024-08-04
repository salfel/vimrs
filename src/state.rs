pub struct State {
    content: String,
}

impl State {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}
