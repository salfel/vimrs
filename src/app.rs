use std::{io, mem, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Stylize},
    widgets::Paragraph,
    Frame,
};

use crate::{mode::EditorMode, state::State};
use crate::{mode::Mode, tui};


pub struct App {
    //file: Option<String>,
    mode: Mode,
}

impl App {
    pub fn new() -> Self {
        let state = State::new(vec![String::new()]);

        App {
            //file: None,
            mode: Mode::new(state),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.mode.get_state().exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;

            self.mode.get_state().clear_error();

            if self.mode.should_change_mode() {
                let mut tmp = Mode::new(State::new(Vec::new()));

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

        let bottom_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        let show_mode = Paragraph::new(self.mode.label());
        frame.render_widget(show_mode, bottom_layout[0]);

        let error = self.mode.get_state().get_error();
        let show_error = Paragraph::new(error).fg(Color::Red).right_aligned();
        frame.render_widget(show_error, bottom_layout[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    self.mode.handle_key(event);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
