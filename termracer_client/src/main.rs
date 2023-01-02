use crossterm::{
    event::{self, Event, KeyCode},
    execute, terminal,
};
use std::io::{self, Write};
use std::time::Duration;
use views::{
    line_block::LineBlock,
    view::{KeyEventHandleable, Rect, View}, stats_line::StatsLine,
};

mod views;

fn main() {
    let mut buf = io::stdout();
    execute!(buf, terminal::EnterAlternateScreen)
        .expect("ERROR: Failed to enter alternate screen.");
    terminal::enable_raw_mode().expect("ERROR: Failed to enable raw mode.");

    let text = "Sample text\nNext line!".to_owned();
    let text_lines = text
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();
    let mut block = LineBlock::new(
        text_lines,
        Rect {
            row: 0,
            column: 0,
            width: 50,
            height: 2,
        },
    );
    let mut stats = StatsLine::new(Rect {
        row: 2,
        column: 0,
        width: 50,
        height: 1
    });
    let mut wpm = 0.0;

    loop {
        if event::poll(Duration::from_millis(30)).expect("ERROR: Failed to poll event.") {
            match event::read().expect("ERROR: Failed to read event.") {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Esc => break,
                    _ => block.handle_key_event(key_event),
                },
                _ => (),
            }
        } else {
            stats.set_wpm(wpm);
            block.display(&mut buf);
            stats.display(&mut buf);
            block.reset_cursor(&mut buf);
            buf.flush().expect("ERROR: Failed to flush buffer.");
            wpm += 0.1;
        }
    }

    terminal::disable_raw_mode().expect("ERROR: Failed to disable raw mode.");
    execute!(buf, terminal::LeaveAlternateScreen)
        .expect("ERROR: Failed to leave alternate screen.");
}
