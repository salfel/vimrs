use std::{io, rc::Rc};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    prelude::{Buffer, Rect, Widget},
    widgets::{block::Title, Block},
    Frame,
};

use crate::{display::Display, mode::Mode, state::State};
use crate::{mode::normal::Normal, tui};

pub struct App {
    exit: bool,
    state: Rc<State>,
    file: Option<String>,
    mode: Box<dyn Mode>,
}

impl App {
    pub fn new() -> Self {
        let state = Rc::new(State::new(String::new()));
        App {
            exit: false,
            file: None,
            mode: Box::new(Normal::new(Rc::clone(&state))),
            state,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;

            if let Some(mode) = self.mode.mode() {
                self.mode = mode;
            }
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let display = Display::new(self.state.get_content());

        frame.render_widget(display, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                self.mode.handle_key(event);
            }
            _ => {}
        }

        Ok(())
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
