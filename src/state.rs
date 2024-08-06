use std::time::SystemTime;

pub struct State {
    pub message: String,
    error: Option<Error>,
    content: Vec<String>,
    pub exit: bool,
    pub cursor: Cursor,
}

impl State {
    pub fn new(content: Vec<String>) -> Self {
        Self {
            message: String::new(),
            content,
            exit: false,
            cursor: Cursor::default(),
            error: None,
        }
    }

    pub fn pop_char(&mut self) {
        if let Some(line) = self.content.get_mut(self.cursor.row) {
            if self.cursor.col == 0 {
                if self.cursor.row != 0 {
                    let content = self.content.remove(self.cursor.row);
                    self.left();

                    if let Some(prev_line) = self.content.get_mut(self.cursor.row) {
                        prev_line.push_str(&content);
                    }
                }

                return;
            }

            line.remove(self.cursor.col - 1);
            self.left();
        }
    }

    pub fn write_char(&mut self, char: char) {
        let row = self
            .content
            .get_mut(self.cursor.row)
            .expect("row was empty");
        row.insert(self.cursor.col, char);
        self.right();
    }

    pub fn remove_char(&mut self) {
        if let Some(row) = self.content.get_mut(self.cursor.row) {
            if self.cursor.col > row.len() - 1 {
                return;
            }

            row.remove(self.cursor.col);
            if self.cursor.col <= row.len() {
                self.left();
            }
        }
    }

    pub fn new_row(&mut self) {
        let after = if self.cursor.col == 0 {
            String::new()
        } else {
            self.content[self.cursor.row]
                .drain(self.cursor.col..)
                .collect()
        };
        self.content.insert(self.cursor.row + 1, after);

        self.down();
        self.cursor.col = 0;
    }

    pub fn trim_row(&mut self) {
        if let Some(row) = self.content.get_mut(self.cursor.row) {
            *row = row.trim().to_string();

            if self.cursor.col == row.len() {
                self.left();
            }
        }
    }

    pub fn get_error(&self) -> &str {
        match &self.error {
            Some(error) => &error.message,
            None => "",
        }
    }

    pub fn add_error(&mut self, message: String) {
        self.error = Some(Error::new(message));
    }

    pub fn clear_error(&mut self) {
        if let Some(error) = &mut self.error {
            let now = SystemTime::now();
            if let Ok(duration) = now.duration_since(error.time) {
                if duration.as_secs() >= 5 {
                    self.error = None;
                }
            }
        }
    }

    pub fn get_content(&self) -> &Vec<String> {
        &self.content
    }

    pub fn get_cursor(&self) -> Cursor {
        self.cursor.clone()
    }

    pub fn down(&mut self) {
        self.cursor.down(&self.content)
    }

    pub fn up(&mut self) {
        self.cursor.up(&self.content)
    }

    pub fn left(&mut self) {
        self.cursor.left(&self.content)
    }

    pub fn right(&mut self) {
        self.cursor.right(&self.content)
    }

    pub fn print(&mut self, message: String) {
        self.message = message
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
        if self.row >= content.len() - 1 {
            return;
        }

        self.row += 1;

        let row_length = content[self.row].len();
        if self.col > row_length {
            self.col = row_length;
        }
    }

    pub fn left(&mut self, content: &[String]) {
        if self.col == 0 {
            if self.row != 0 {
                self.row -= 1;

                let row = &content[self.row];
                self.col = if row.is_empty() { 0 } else { row.len() };
            }
        } else {
            self.col -= 1;
        }
    }

    pub fn right(&mut self, content: &[String]) {
        if self.col == content[self.row].len() {
            if self.row != content.len() - 1 {
                self.row += 1;
                self.col = 0;
            }
        } else {
            self.col += 1;
        }
    }
}

pub struct Error {
    time: SystemTime,
    message: String,
}

impl Error {
    fn new(message: String) -> Self {
        Self {
            time: SystemTime::now(),
            message,
        }
    }
}
