use std::{io, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::Paragraph,
    Frame,
};

use crate::{
    buffer::Buffer,
    tui,
};

pub struct App {
    exit: bool,
    buffers: Vec<Buffer>,
    active_buffer: usize,
}

impl App {
    pub fn new(_args: Vec<String>) -> Self {
        App {
            exit: false,
            buffers: vec![Buffer::new(String::new(), String::new())],
            active_buffer: 0,
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
        let paragraph = Paragraph::new(self.get_active_buffer().context.content.join("\n"));
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

    fn get_active_buffer(&mut self) -> &mut Buffer {
        &mut self.buffers[self.active_buffer]
    }
}
