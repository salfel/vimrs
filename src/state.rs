pub struct State {
    pub content: String,
    pub exit: bool,
}

impl State {
    pub fn new(content: String) -> Self {
        Self {
            content,
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
