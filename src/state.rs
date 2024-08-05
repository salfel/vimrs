pub struct State {
    content: Vec<String>,
    pub exit: bool,
    cursor: Cursor,
}

impl State {
    pub fn new(content: Vec<String>) -> Self {
        Self {
            content,
            exit: false,
            cursor: Cursor::default(),
        }
    }

    pub fn write_char(&mut self, char: char) {
        let row = match self.content.get_mut(self.cursor.row) {
            Some(content) => content,
            None => {
                self.content.push(String::new());
                &mut self.content[0]
            }
        };

        row.insert(self.cursor.col, char);
        self.cursor.col += 1;
    }

    pub fn pop_char(&mut self) {
        if let Some(row) = self.content.get_mut(self.cursor.row) {
            if row.is_empty() {
                return;
            }

            row.remove(self.cursor.col - 1);
            self.cursor.col -= 1;
        }
    }

    pub fn new_row(&mut self) {
        self.content.push(String::new());
        self.cursor.row += 1;
        self.cursor.col = 0;
    }

    pub fn get_content(&self) -> String {
        self.content.join("\n")
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}

#[derive(Default)]
struct Cursor {
    col: usize,
    row: usize,
}
