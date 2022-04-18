use crossterm::{
    cursor::MoveTo,
    event,
    event::{Event, KeyCode},
    terminal,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::io;
use std::io::{Error, Stdout, Write};

use super::command_parser;
use super::command_parser::Command;
use super::session::Session;
use super::widget::{Widget, WidgetProps};

pub struct App {
    buf: Stdout,
    session: Option<Session>, // None when no session is active
}

impl App {
    pub fn new() -> App {
        App {
            buf: io::stdout(),
            session: None,
        }
    }

    pub fn start_new_session(&mut self) -> Result<(), Error> {
        self.session = Some(Session::new(
            &vec![
                "little between not small those here go high use world with out they".to_string(),
                "small get such so course right few one hold much when never late".to_string(),
                "old each end help what well off will high come possible we ask who".to_string(),
            ],
            WidgetProps {
                row_offset: 0,
                column_offset: 0,
            },
        ));
        terminal::enable_raw_mode()?;
        self.buf.execute(EnterAlternateScreen)?;
        self.session.as_mut().unwrap().start();
        self.session.as_mut().unwrap().print(&mut self.buf)?;
        self.buf.flush()?;
        Ok(())
    }

    // used when a session terminates forcefully
    fn force_end_session(&mut self) -> Result<(), Error> {
        self.session = None;
        self.buf.execute(LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    // used when a session is ended normally (line block is done)
    fn end_session(&mut self) -> Result<(), Error> {
        let wpm = self.session.as_ref().unwrap().get_wpm().unwrap();

        self.session = None;
        self.buf.execute(LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        println!("Session finished! WPM: {}.", wpm as u16);
        Ok(())
    }

    pub fn event_loop(&mut self) -> Result<(), Error> {
        self.buf.queue(Clear(ClearType::All))?.queue(MoveTo(0, 0))?;
        self.buf.flush()?;

        loop {
            match self.session.as_mut() {
                Some(session) => {
                    // exit if session is done
                    if session.is_done() {
                        self.end_session()?;
                        continue;
                    }

                    match event::read()? {
                        Event::Key(key_event) => match key_event.code {
                            KeyCode::Esc => {
                                self.force_end_session()?;
                                break;
                            }
                            key_code => session.process_key_code(key_code, &mut self.buf)?,
                        },
                        _ => (),
                    }
                    session.refresh(&mut self.buf)?;
                    self.buf.flush()?
                }
                None => {
                    print!("> ");
                    io::stdout().flush()?;

                    let mut line = String::new();
                    io::stdin().read_line(&mut line)?;
                    line = line.trim_end().to_string();

                    match command_parser::parse_string(line) {
                        Some(Command::Start) => {
                            self.start_new_session()?;
                            continue;
                        }
                        Some(Command::Quit) => {
                            break;
                        }
                        None => println!("Invalid command."),
                    }
                }
            }
        }
        Ok(())
    }
}
