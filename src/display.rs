use ratatui::{
    prelude::{Buffer, Color, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::state::State;

pub struct Display<'a> {
    state: &'a State,
    after_cursor: bool,
}

impl<'a> Display<'a> {
    pub fn new(state: &'a State, after_cursor: bool) -> Self {
        Display {
            state,
            after_cursor,
        }
    }
}

impl Widget for Display<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = Vec::new();
        let content = self.state.get_content();
        let cursor = self.state.get_cursor();

        for (idx, row) in content.iter().enumerate() {
            let mut spans = vec![];
            if idx == cursor.row {
                if self.after_cursor {
                    if cursor.col == row.len() {
                        spans.push(Span::raw(&row[0..cursor.col]));
                        spans.push(Span::styled(" ", Style::default().bg(Color::Black)));
                    } else {
                        spans.push(Span::raw(&row[0..cursor.col]));
                        spans.push(Span::styled(
                            &row[cursor.col..cursor.col + 1],
                            Style::default().bg(Color::Black),
                        ));
                        spans.push(Span::raw(&row[cursor.col + 1..row.len()]));
                    }
                } else if row.is_empty() {
                    spans.push(Span::styled(" ", Style::default().on_black()))
                } else if cursor.col == 0 {
                    spans.push(Span::styled(&row[0..1], Style::default().on_black()));
                    spans.push(Span::raw(&row[1..]));
                } else {
                    spans.push(Span::raw(&row[0..cursor.col - 1]));
                    spans.push(Span::styled(
                        &row[cursor.col - 1..cursor.col],
                        Style::default().on_black(),
                    ));
                    spans.push(Span::raw(&row[cursor.col..]));
                }
            } else {
                spans.push(Span::raw(row));
            }
            lines.push(Line::from(spans));
        }

        Paragraph::new(lines).render(area, buf)
    }
}
