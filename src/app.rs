use std::{io, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Layout},
    widgets::Paragraph,
    Frame,
};

use crate::{buffer::Buffer, tui};

pub struct App {
    buffers: Vec<Buffer>,
    active_buffer: usize,
}

impl App {
    pub fn new(_args: Vec<String>) -> Self {
        App {
            buffers: vec![Buffer::new(String::new(), String::new())],
            active_buffer: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.get_active_buffer().should_exit() {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(frame.size());

        let paragraph = Paragraph::new(self.get_active_buffer().get_content().to_string());
        frame.render_widget(paragraph, layout[0]);

        let paragraph = Paragraph::new(format!(
            "-- {} --   {}",
            self.get_active_buffer().get_mode(),
            self.get_active_buffer().print()
        ));
        frame.render_widget(paragraph, layout[1]);

        self.get_active_buffer().render_cursor(frame);
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
