use std::borrow::Borrow;

use ratatui::{
    prelude::Color,
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::state::State;

pub struct Display<'a> {
    state: &'a State,
}

impl<'a> Display<'a> {
    pub fn new(state: &'a State) -> Self {
        Display { state }
    }
}

impl Widget for Display<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut lines = Vec::new();
        let content = self.state.get_content();
        let cursor = self.state.get_cursor();

        for (idx, row) in content.iter().enumerate() {
            let mut spans = vec![];
            if idx == cursor.row {
                if row.is_empty() {
                    spans.push(Span::styled(" ", Style::default().bg(Color::Black)))
                } else if cursor.col == 0 {
                    spans.push(Span::styled(&row[0..1], Style::default().bg(Color::Black)));
                    spans.push(Span::raw(&row[cursor.col + 1..row.len()]));
                } else {
                    spans.push(Span::raw(&row[0..cursor.col - 1]));
                    spans.push(Span::styled(
                        &row[cursor.col - 1..cursor.col],
                        Style::default().bg(Color::Black),
                    ));
                    spans.push(Span::raw(&row[cursor.col..row.len()]));
                }
            } else {
                spans.push(Span::raw(row));
            }
            lines.push(Line::from(spans));
        }

        Paragraph::new(lines).render(area, buf)
    }
}
