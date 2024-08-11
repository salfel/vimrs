pub struct Context {
    pub content: Vec<String>,
    pub cursor: Cursor,
}

impl Context {
    pub fn new(content: String) -> Self {
        let content = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(String::from).collect::<Vec<String>>()
        };

        Context {
            content,
            cursor: Cursor::default(),
        }
    }
}

#[derive(Default)]
pub struct Cursor {
    pub col: usize,
    pub row: usize,
}
