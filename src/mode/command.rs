use super::{normal::NormalMode, EditorMode, Mode};
use crate::{display::Display, state::State};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    widgets::Paragraph,
    Frame,
};

pub struct CommandMode {
    state: State,
    command: String,
    exit: bool,
}

impl CommandMode {
    pub fn new(state: State) -> Self {
        CommandMode {
            state,
            command: String::new(),
            exit: false,
        }
    }

    fn execute_command(&mut self) {
        match self.command.as_str() {
            "q" => self.state.exit = true,
            _ => {}
        }
        self.exit_to_normal();
    }

    fn remove_char(&mut self) {
        if self.command.pop().is_none() {
            self.exit_to_normal();
        }
    }

    fn exit_to_normal(&mut self) {
        self.exit = true;
    }
}

impl EditorMode for CommandMode {
    fn label(&self) -> String {
        String::from("Command")
    }

    fn should_change_mode(&self) -> bool {
        self.exit
    }

    fn mode(self) -> Option<Box<Mode>> {
        if self.exit {
            Some(Box::new(Mode::Normal(NormalMode::new(self.state))))
        } else {
            None
        }
    }

    fn handle_key(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => self.exit_to_normal(),
            KeyCode::Char(char) => self.command.push(char),
            KeyCode::Backspace => self.remove_char(),
            KeyCode::Enter => self.execute_command(),
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(rect);

        let display = Display::new(self.state.get_content());
        frame.render_widget(display, layout[0]);

        let command = Paragraph::new(format!(":{}", self.command));
        frame.render_widget(command, layout[1]);
    }

    fn get_state(&self) -> &State {
        &self.state
    }
}
