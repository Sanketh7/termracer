use std::error;

mod app;
mod command_parser;
mod line;
mod line_block;
mod session;
mod session_event_handler;
mod stats_line;
mod widget;

use app::App;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut app = App::new();
    app.event_loop()?;

    Ok(())
}
