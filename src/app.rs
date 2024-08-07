use std::{io, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    widgets::Paragraph,
    Frame,
};

use crate::buffer::Buffer;
use crate::tui;

pub struct App {
    exit: bool,
    buffers: Vec<Buffer>,
    errors: Vec<String>,
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        let mut errors = Vec::new();

        let filename = args.get(1).map_or(String::new(), |arg| arg.to_string());
        let buffer = Buffer::new(filename, &mut errors);

        App {
            exit: false,
            buffers: vec![buffer],
            errors,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let content = self.buffers.first().map_or("", |buffer| &buffer.content);
        let paragraph = Paragraph::new(content);
        frame.render_widget(paragraph, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => {}
                _ => {}
            }
        }

        Ok(())
    }

    fn get_error(&mut self) -> &str {
        match self.errors.first() {
            Some(error) => error,
            None => "",
        }
    }
}
