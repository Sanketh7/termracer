use std::io::Error;

mod app;
mod command_parser;
mod constants;
mod widgets;
mod word_generator;

use app::App;

fn main() -> Result<(), Error> {
    let mut app = App::new();
    app.event_loop()?;

    Ok(())
}
