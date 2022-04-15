use crossterm::{
    event,
    event::{Event, KeyCode},
    terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io;
use std::io::{Error, Write};

mod line;
mod line_block;
mod session;
mod stats_line;
mod widget;

use session::Session;
use widget::{Widget, WidgetProps};

fn main() -> Result<(), Error> {
    terminal::enable_raw_mode()?;

    let mut buf = io::stdout();
    buf.execute(EnterAlternateScreen)?;

    let mut session = Session::new(
        &vec![
            "a a a a a a a a a a a a a a a a a a a a a a a a a a a ".to_string(),
            "Ut eget suscipit lectus, id egestas neque. Mauris at metus ipsum.".to_string(),
        ],
        WidgetProps {
            row_offset: 0,
            column_offset: 0,
        },
    );
    session.print(&mut buf)?;
    buf.flush()?;
    session.start();

    loop {
        match event::read()? {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Esc => break,
                    key_code => session.process_key_code(key_code, &mut buf)?,
                }
                session.refresh(&mut buf)?;
                buf.flush()?;
            }
            _ => (),
        }
    }

    terminal::disable_raw_mode()?;
    buf.execute(LeaveAlternateScreen)?;
    Ok(())
}
