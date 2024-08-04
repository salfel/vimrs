use ratatui::widgets::{Paragraph, Widget};

pub struct Display<'a> {
    content: &'a str,
}

impl<'a> Display<'a> {
    pub fn new(content: &'a str) -> Self {
        Display { content }
    }
}

impl Widget for Display<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.content).render(area, buf)
    }
}
