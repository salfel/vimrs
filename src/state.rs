pub struct State {
    content: String,
    pub exit: bool,
}

impl State {
    pub fn new(content: String) -> Self {
        Self {
            content,
            exit: false,
        }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
