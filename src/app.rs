use std::{fmt::write, io, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame,
};

use crate::tui;
use crate::{buffer::Buffer, mode::EditorMode};

pub struct App {
    buffers: Vec<Buffer>,
    errors: Vec<String>,
    active_buffer: usize,
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        let mut errors = Vec::new();

        let filename = args.get(1).map_or(String::new(), |arg| arg.to_string());
        let buffer = Buffer::new(filename, &mut errors);

        App {
            buffers: vec![buffer],
            errors,
            active_buffer: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.should_exit() {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;

            self.buffers[self.active_buffer].change_mode();
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let current_buffer = &self.buffers[self.active_buffer];

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(frame.size());

        let content = self.buffers.first().map_or("", |buffer| &buffer.content);
        let paragraph = Paragraph::new(content);
        frame.render_widget(paragraph, layout[0]);

        let bottom_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50); 2])
            .split(layout[1]);

        let label = format!("-- {} --", current_buffer.mode.label());

        let paragraph = Paragraph::new(label);
        frame.render_widget(paragraph, bottom_layout[0]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    let current_buffer = &mut self.buffers[self.active_buffer];
                    current_buffer.mode.handle_events(event);
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn should_exit(&self) -> bool {
        if let Some(buffer) = self.buffers.get(self.active_buffer) {
            buffer.should_exit()
        } else {
            true
        }
    }

    fn get_error(&mut self) -> &str {
        match self.errors.first() {
            Some(error) => error,
            None => "",
        }
    }
}
