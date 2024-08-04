use super::{normal::Normal, Mode};
use crate::{display::Display, state::State};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    widgets::Paragraph,
    Frame,
};
use std::{cell::RefCell, rc::Rc};

pub struct Command {
    mode: Option<Box<dyn Mode>>,
    state: Rc<RefCell<State>>,
    command: String,
}

impl Command {
    pub fn new(state: Rc<RefCell<State>>) -> Self {
        Command {
            mode: None,
            state,
            command: String::new(),
        }
    }

    fn execute_command(&mut self) {
        match self.command.as_str() {
            "q" => (*self.state).borrow_mut().exit = true,
            _ => {}
        }
        self.exit_to_normal();
    }

    fn exit_to_normal(&mut self) {
        self.mode = Some(Box::new(Normal::new(Rc::clone(&self.state))));
    }
}

impl Mode for Command {
    fn label(&self) -> String {
        String::from("Command")
    }

    fn mode(&mut self) -> Option<Box<dyn Mode>> {
        self.mode.take()
    }

    fn handle_key(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => self.exit_to_normal(),
            KeyCode::Char(char) => self.command.push(char),
            KeyCode::Backspace => {
                self.command.pop();
            }
            KeyCode::Enter => self.execute_command(),
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(rect);

        let state = (*self.state).borrow();
        let display = Display::new(state.get_content());
        frame.render_widget(display, layout[0]);

        let command = Paragraph::new(format!(":{}", self.command));
        frame.render_widget(command, layout[1]);
    }
}
