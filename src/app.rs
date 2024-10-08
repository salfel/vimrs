use std::{
    io::{self, Stdout},
    time::Duration,
};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    prelude::{Buffer as TBuffer, CrosstermBackend, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    Frame, Terminal,
};

use crate::buffer::{Buffer, Register};

#[allow(dead_code)]
pub struct App {
    buffers: Vec<Buffer>,
    active_buffer: usize,
    register: Register,
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        let filename = args.get(1).map_or(String::new(), |value| value.to_string());
        let register = Register::new();

        App {
            buffers: vec![Buffer::new(filename, &register)],
            register,
            active_buffer: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        while !self.get_active_buffer().exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        self.render(frame.area(), frame.buffer_mut());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    self.get_active_buffer().handle_keys(event)
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn get_active_buffer(&mut self) -> &mut Buffer {
        &mut self.buffers[self.active_buffer]
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut TBuffer) {
        let active_buffer = &self.buffers[self.active_buffer];
        let layout = Layout::default()
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        Paragraph::new(active_buffer.content.join("\n").to_string()).render(layout[0], buf);

        Paragraph::new(Line::from(vec![
            Span::raw(format!("-- {} --     ", active_buffer.mode)),
            active_buffer.message(),
        ]))
        .render(layout[1], buf);

        active_buffer.render_cursor(buf);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::{Style, Stylize};

    use super::*;

    #[test]
    fn renders_blank_with_mode() {
        let app = App::new(Vec::new());
        let mut buf = TBuffer::empty(Rect::new(0, 0, 20, 5));

        app.render(buf.area, &mut buf);

        let mut expected = TBuffer::with_lines(vec![
            "                    ",
            "                    ",
            "                    ",
            "                    ",
            "-- Normal --        ",
        ]);
        expected.set_style(Rect::new(0, 0, 1, 1), Style::new().on_white().black());

        assert_eq!(buf, expected);
    }
}
