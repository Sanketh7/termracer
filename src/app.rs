use crossterm::{
    event,
    event::{Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io;
use std::io::{Error, Stdout, Write};

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
                "a a a a a a a a a a a a a a a a a a a a a a a a a a a ".to_string(),
                "Ut eget suscipit lectus, id egestas neque. Mauris at metus ipsum.".to_string(),
            ],
            WidgetProps {
                row_offset: 0,
                column_offset: 0,
            },
        ));
        self.buf.execute(EnterAlternateScreen)?;
        self.session.as_mut().unwrap().start();
        self.session.as_mut().unwrap().print(&mut self.buf)?;
        self.buf.flush()?;
        Ok(())
    }

    // used when a session terminates forcefully
    pub fn terminate_session(&mut self) -> Result<(), Error> {
        self.session = None;
        self.buf.execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn event_loop(&mut self) -> Result<(), Error> {
        match self.session.as_mut() {
            Some(session) => loop {
                match event::read()? {
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Esc => {
                            self.terminate_session()?;
                            break;
                        }
                        key_code => session.process_key_code(key_code, &mut self.buf)?,
                    },
                    _ => (),
                }
                session.refresh(&mut self.buf)?;
                self.buf.flush()?
            },
            None => (),
        }
        Ok(())
    }
}
