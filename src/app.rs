use std::{io, mem};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame,
};

use crate::{mode::EditorMode, state::State};
use crate::{mode::Mode, tui};

pub struct App {
    file: Option<String>,
    mode: Mode,
}

impl App {
    pub fn new() -> Self {
        let state = State::new(String::new());

        App {
            file: None,
            mode: Mode::new(state),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.mode.get_state().exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;

            if self.mode.should_change_mode() {
                let mut tmp = Mode::new(State::new(String::new()));

                mem::swap(&mut tmp, &mut self.mode);

                self.mode = *tmp.mode().expect("mode was not some");
            }
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(frame.size());

        self.mode.render(frame, layout[0]);

        let show_mode = Paragraph::new(self.mode.label());
        frame.render_widget(show_mode, layout[1]);
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
