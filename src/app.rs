use std::{cell::RefCell, io, rc::Rc};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame,
};

use crate::{mode::normal::Normal, tui};
use crate::{mode::Mode, state::State};

pub struct App {
    state: Rc<RefCell<State>>,
    file: Option<String>,
    mode: Box<dyn Mode>,
}

impl App {
    pub fn new() -> Self {
        let state = Rc::new(RefCell::new(State::new(String::new())));
        App {
            file: None,
            mode: Box::new(Normal::new(Rc::clone(&state))),
            state,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.state.borrow().exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;

            if let Some(mode) = self.mode.mode() {
                self.mode = mode;
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
