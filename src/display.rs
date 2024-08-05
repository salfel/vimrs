use ratatui::widgets::{Paragraph, Widget};

pub struct Display {
    content: String,
}

impl Display {
    pub fn new(content: String) -> Self {
        Display { content }
    }
}

impl Widget for Display {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.content).render(area, buf)
    }
}
