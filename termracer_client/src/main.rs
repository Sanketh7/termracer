use crate::views::{line::Line, view::Rect};
use crossterm::{
    event::{self, Event, KeyCode},
    execute, terminal,
};
use std::io::{self, Write};
use std::time::Duration;
use views::view::{KeyEventHandleable, View};

mod views;

fn main() {
    let mut buf = io::stdout();
    execute!(buf, terminal::EnterAlternateScreen)
        .expect("ERROR: Failed to enter alternate screen.");
    terminal::enable_raw_mode().expect("ERROR: Failed to enable raw mode.");

    let text = "Sample text".to_owned();
    let mut line = Line::new(
        text.chars().collect(),
        Rect {
            row: 0,
            column: 0,
            width: 50,
            height: 1,
        },
    );

    line.display(&mut buf);

    loop {
        if event::poll(Duration::from_millis(30)).expect("ERROR: Failed to poll event.") {
            match event::read().expect("ERROR: Failed to read event.") {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Esc => break,
                    _ => line.handle_key_event(key_event),
                },
                _ => (),
            }
        } else {
            line.display(&mut buf);
            line.reset_cursor(&mut buf);
            buf.flush().expect("ERROR: Failed to flush buffer.")
        }
    }

    terminal::disable_raw_mode().expect("ERROR: Failed to disable raw mode.");
    execute!(buf, terminal::LeaveAlternateScreen)
        .expect("ERROR: Failed to leave alternate screen.");
}
