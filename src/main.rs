use std::io;
use std::io::{Write, Error};
use std::result::Result;
use crossterm::{
    ExecutableCommand, 
    event, event::{Event, KeyCode},
    terminal, terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

mod line;
mod line_block;
mod session;

use line_block::LineBlock;

fn main() -> Result<(), Error> {
    terminal::enable_raw_mode()?;

    let mut buf = io::stdout();
    buf.execute(EnterAlternateScreen)?;

    let mut block = LineBlock::new();
    block.new_line("Hello, World!".to_string());
    block.new_line("Bye, Moon!".to_string());
    block.print(&mut buf)?;
    buf.flush()?;

    loop {
        match event::read()? {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Esc => break,
                    key_code => block.process_key_code(key_code, &mut buf)?,
                }
                buf.flush()?;
            }
            _ => ()
        }
    }

    terminal::disable_raw_mode()?;
    buf.execute(LeaveAlternateScreen)?;
    Ok(())
}
