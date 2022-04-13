use std::io;
use std::io::Write;
use std::result::Result;
use crossterm::{
    ExecutableCommand, 
    event, event::{Event, KeyCode},
    terminal,
};

mod line;
mod line_block;

use line_block::LineBlock;

fn main() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;

    let mut buf = io::stdout();
    buf.execute(terminal::EnterAlternateScreen)?;

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
                    KeyCode::Backspace => block.process_backspace(&mut buf)?,
                    KeyCode::Char(c) => block.process_character(c, &mut buf)?,
                    _ => (),
                }
                buf.flush()?;
            }
            _ => ()
        }
    }

    terminal::disable_raw_mode()?;
    buf.execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}
