use std::{io, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::Paragraph,
    Frame,
};

use crate::tui;

pub struct App {
    exit: bool,
}

impl App {
    pub fn new(_args: Vec<String>) -> Self {
        App { exit: false }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let paragraph = Paragraph::new("Hello world");
        frame.render_widget(paragraph, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                    KeyCode::Char('q') => self.exit = true,
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }
}
