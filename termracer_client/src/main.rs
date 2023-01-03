use crossterm::{
    event::{self, Event, KeyCode},
    execute, terminal,
};
use rect::Rect;
use std::io::{self, Write};
use std::time::Duration;
use unicode_segmentation::UnicodeSegmentation;
use views::{
    line_block::LineBlock,
    stats_line::StatsLine,
    view::{KeyEventHandleable, View},
};
use window::Window;

mod rect;
mod views;
mod window;

fn main() {
    let mut buf = io::stdout();
    execute!(buf, terminal::EnterAlternateScreen)
        .expect("ERROR: Failed to enter alternate screen.");
    terminal::enable_raw_mode().expect("ERROR: Failed to enable raw mode.");

    let text = "A lot of sample text oh boy\n".repeat(49).to_owned();
    let text_lines = text
        .split('\n')
        .map(|line| line.graphemes(true).map(String::from).collect())
        .collect();
    let mut block = LineBlock::new(
        text_lines,
        Rect {
            row: 0,
            column: 0,
            width: 50,
            height: 50,
        },
    );
    let mut stats = StatsLine::new(Rect {
        row: 0,
        column: 0,
        width: 50,
        height: 1,
    });
    let mut wpm = 0.0;

    let mut block_window = Window::new(Rect {
        row: 0,
        column: 0,
        width: 50,
        height: 50,
    });

    let mut stats_window = Window::new(Rect {
        row: 50,
        column: 0,
        width: 50,
        height: 1,
    });

    loop {
        if event::poll(Duration::from_millis(10)).expect("ERROR: Failed to poll event.") {
            match event::read().expect("ERROR: Failed to read event.") {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Esc => break,
                    _ => block.handle_key_event(key_event),
                },
                _ => (),
            }
        } else {
            stats.set_wpm(wpm);
            block.draw(&mut block_window);
            stats.draw(&mut stats_window);
            block_window.display(&mut buf);
            stats_window.display(&mut buf);
            block.reset_cursor(&mut buf);
            buf.flush().expect("ERROR: Failed to flush buffer.");
            wpm += 10.0;
        }
    }

    terminal::disable_raw_mode().expect("ERROR: Failed to disable raw mode.");
    execute!(buf, terminal::LeaveAlternateScreen)
        .expect("ERROR: Failed to leave alternate screen.");
}
