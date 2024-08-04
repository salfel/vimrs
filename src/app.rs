use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::{Buffer, Rect, Widget},
    widgets::{block::Title, Block},
    Frame,
};

use crate::display::Display;
use crate::tui;

pub struct App {
    exit: bool,
    content: String,
    file: Option<String>,
}

impl App {
    pub fn new() -> Self {
        App {
            exit: false,
            content: String::new(),
            file: None,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let display = Display::new(&self.content);

        frame.render_widget(display, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                self.handle_key_events(event);
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_events(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("test");

        Block::bordered()
            .title(title.alignment(ratatui::layout::Alignment::Center))
            .render(area, buf);
    }
}
