use crossterm::{
    event::KeyCode,
    terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::error;
use std::io;
use std::io::{Stdout, Write};
use std::mem;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

use super::command_parser;
use super::command_parser::Command;
use super::session::Session;
use super::session_event_handler;
use super::widget::{Widget, WidgetProps};

pub struct App {
    buf: Stdout,
    session: Option<Session>, // None when no session is active
    session_event_handler_receiver: Option<Receiver<KeyCode>>,
    session_event_handler_handle: Option<JoinHandle<()>>,
}

impl App {
    pub fn new() -> App {
        App {
            buf: io::stdout(),
            session: None,
            session_event_handler_receiver: None,
            session_event_handler_handle: None,
        }
    }

    fn start_new_session(&mut self) -> Result<(), Box<dyn error::Error>> {
        let (transmitter, receiver) = mpsc::channel::<KeyCode>();

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
        self.session_event_handler_receiver = Some(receiver);
        self.session_event_handler_handle = Some(session_event_handler::spawn_thread(transmitter)?);

        // set up terminal
        terminal::enable_raw_mode()?;
        self.buf.execute(EnterAlternateScreen)?;

        // print and start session
        self.session.as_mut().unwrap().print(&mut self.buf)?;
        self.buf.flush()?;
        self.session.as_mut().unwrap().start();
        Ok(())
    }

    pub fn event_loop(&mut self) -> Result<(), Box<dyn error::Error>> {
        loop {
            if let Some(session_event_handler_receiver) = &self.session_event_handler_receiver {
                // there MUST exist an active session
                assert!(self.session.is_some());
                if let Ok(msg) = session_event_handler_receiver.try_recv() {
                    // tr_recv() is non-blocking
                    match msg {
                        KeyCode::Esc => {
                            // note that the thread loop will break on
                            // KeyCode::Esc
                            let handle = mem::replace(&mut self.session_event_handler_handle, None);
                            handle.unwrap().join().unwrap();
                            assert!(self.session_event_handler_handle.is_none());

                            self.session = None;
                            self.session_event_handler_receiver = None;

                            self.buf.execute(LeaveAlternateScreen)?;
                            terminal::disable_raw_mode()?;
                            break;
                        }
                        _ => {
                            self.session
                                .as_mut()
                                .unwrap()
                                .process_key_code(msg, &mut self.buf)?;
                            self.session.as_mut().unwrap().refresh(&mut self.buf)?;
                            self.buf.flush()?;
                        }
                    }
                }
            } else {
                // there must not be a session active
                assert!(self.session.is_none());

                print!("> ");
                io::stdout().flush()?;

                let mut line = String::new();
                io::stdin().read_line(&mut line)?;
                line = line.trim_end().to_string();

                match command_parser::parse_string(line) {
                    Some(Command::StartSession) => {
                        self.start_new_session()?;
                        continue;
                    }
                    None => println!("Invalid command."),
                }
            }
        }
        Ok(())
    }
}
