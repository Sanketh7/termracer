use std::io::Error;
use std::thread;

mod app;
mod command_parser;
mod line;
mod line_block;
mod session;
mod stats_line;
mod widget;

use app::App;

fn main() -> Result<(), Error> {
    let mut app = App::new();
    // app.start_new_session()?;
    app.event_loop()?;

    Ok(())
}
