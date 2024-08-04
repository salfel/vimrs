mod app;
mod display;
mod tui;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::new().run(&mut terminal);
    tui::restore()?;
    app_result
}
