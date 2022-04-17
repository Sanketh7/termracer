use crossterm::terminal;
use std::io::Error;

mod app;
mod line;
mod line_block;
mod session;
mod stats_line;
mod widget;

use app::App;

fn main() -> Result<(), Error> {
    terminal::enable_raw_mode()?;

    let mut app = App::new();
    app.start_new_session()?;
    app.event_loop()?;

    terminal::disable_raw_mode()?;
    Ok(())
}
