mod app;
mod display;
mod filesystem;
mod mode;
mod state;
mod tui;

use app::App;
use std::{env, io};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    let mut terminal = tui::init()?;
    let app_result = App::new(args).run(&mut terminal);
    tui::restore()?;
    app_result
}
