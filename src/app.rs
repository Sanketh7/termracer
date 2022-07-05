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

use crate::command_parser;
use crate::command_parser::Command;
use crate::session::Session;
use crate::widget::{Coord, EventHandleableWidget, ViewableWidget, ViewableWidgetProps};

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
        self.session = Some(Session::new(ViewableWidgetProps {
            offset: Coord { row: 0, col: 0 },
        }));

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

        println!("Session aborted!");
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
                                continue;
                            }
                            key_code => {
                                session.process_key_code(key_code, &mut self.buf)?;
                            }
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
                        Some(Command::Help) => {
                            println!("{}", command_parser::HELP_STRING);
                            continue;
                        }
                        None => {
                            println!("Invalid command.\nUse \"help\" to see a list of commands.")
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
