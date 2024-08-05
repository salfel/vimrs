pub struct State {
    content: Vec<String>,
    pub exit: bool,
    pub cursor: Cursor,
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
        let row = self
            .content
            .get_mut(self.cursor.row)
            .expect("row was empty");
        row.insert(self.cursor.col, char);
        self.cursor.col += 1;
    }

    pub fn pop_char(&mut self) {
        if let Some(row) = self.content.get_mut(self.cursor.row) {
            if row.is_empty() && self.cursor.row != 0 {
                self.content.remove(self.cursor.row);
                return;
            }

            row.remove(self.cursor.col - 1);
            self.cursor.col -= 1;
        }
    }

    pub fn new_row(&mut self) {
        self.content.push(String::new());

        self.cursor.down(&self.content);
    }

    pub fn get_content(&self) -> &Vec<String> {
        &self.content
    }

    pub fn get_cursor(&self) -> Cursor {
        self.cursor.clone()
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}

#[derive(Default, Clone)]
pub struct Cursor {
    pub col: usize,
    pub row: usize,
}

impl Cursor {
    pub fn up(&mut self, content: &[String]) {
        if self.row == 0 {
            return;
        }

        self.row -= 1;

        let row_length = content[self.row].len();
        if self.col > row_length {
            self.col = row_length;
        }
    }

    pub fn down(&mut self, content: &[String]) {
        if self.row >= content.len() {
            return;
        }

        self.row += 1;

        let row_length = content[self.row].len();
        if self.col > row_length {
            self.col = row_length;
        }
    }
}
