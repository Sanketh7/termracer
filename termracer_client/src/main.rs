use crossterm::{
    event::{self, Event, KeyCode},
    execute, terminal,
};
use rect::{Coord, HorizontalSplit, Rect};
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

    let mut window = Window::new(Rect {
        coord: Coord { row: 0, col: 0 },
        width: 50,
        height: 50,
    });
    let (block_region, stats_region) =
        window.horizontal_split(HorizontalSplit::CellsInBottom(1), 0);

    let text = "A lot of sample text oh boy\n".repeat(49).to_owned();
    let text_lines = text
        .split('\n')
        .map(|line| line.graphemes(true).map(String::from).collect())
        .collect();
    let mut block = LineBlock::new(text_lines, block_region);
    let mut stats = StatsLine::new(stats_region);
    let mut wpm = 0.0;

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
            block.draw(&mut window);
            stats.draw(&mut window);
            window.display(&mut buf);
            // block.reset_cursor(&mut buf);
            buf.flush().expect("ERROR: Failed to flush buffer.");
            wpm += 10.0;
        }
    }

    terminal::disable_raw_mode().expect("ERROR: Failed to disable raw mode.");
    execute!(buf, terminal::LeaveAlternateScreen)
        .expect("ERROR: Failed to leave alternate screen.");
}
