use std::{io, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    widgets::Paragraph,
    Frame,
};

use crate::{buffer::Buffer, mode::Mode, tui};

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
        let mut content = self.get_active_buffer().get_content().to_string();
        content.push_str(match self.get_active_buffer().context.mode {
            Mode::Normal => "normal",
            Mode::Insert => "insert",
        });
        let paragraph = Paragraph::new(content);
        frame.render_widget(paragraph, frame.size());
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
