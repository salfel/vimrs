mod app;
mod buffer;
mod filesystem;
mod mode;
mod motion;
mod navigation;
mod tui;
mod words;

#[cfg(test)]
mod test;

use app::App;
use std::{env, io};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    let mut terminal = tui::init()?;
    let app_result = App::new(args).run(&mut terminal);
    tui::restore()?;
    app_result
}
