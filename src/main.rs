mod actions;
mod app;
mod buffer;
mod filesystem;
mod mode;
mod motion;
mod navigation;
mod utils;

use app::App;
use std::{env, io};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    let mut terminal = ratatui::init();
    let app_result = App::new(args).run(&mut terminal);
    ratatui::restore();
    app_result
}
