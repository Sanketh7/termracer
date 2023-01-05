use std::{
    io::{self, Write},
    time::Duration,
};

use crate::{
    layout::HorizontalSplitKind,
    views::{
        line_block::LineBlock,
        stats_line::StatsLine,
        view::{KeyEventHandleable, View},
    },
    window::Window,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use unicode_segmentation::UnicodeSegmentation;

struct UI {
    window: Window,
    // views
    line_block: LineBlock,
    stats_line: StatsLine,
}

pub struct SoloGame {
    ui: UI,
}

impl SoloGame {
    pub fn new() -> Self {
        let text = "A lot of sample text oh boy\n".repeat(49).to_owned();
        let text_lines = text
            .split('\n')
            .map(|line| line.graphemes(true).map(String::from).collect())
            .collect();
        let (term_width, term_height) =
            terminal::size().expect("ERROR: Failed to get terminal size.");
        let mut window = Window::new(term_width, term_height);
        let (line_block_region, stats_line_region) =
            window.horizontal_split(HorizontalSplitKind::CellsInBottom(1), 0);
        let line_block = LineBlock::new(text_lines, line_block_region);
        let stats_line = StatsLine::new(stats_line_region);

        SoloGame {
            ui: UI {
                window,
                line_block,
                stats_line,
            },
        }
    }

    pub fn game_loop<T: Write>(&mut self, buf: &mut T, poll_duration: Duration) {
        let mut wpm = 0.0;

        loop {
            if event::poll(poll_duration).expect("ERROR: Failed to poll event.") {
                match event::read().expect("ERROR: Failed to read event.") {
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Esc => break,
                        _ => self.ui.line_block.handle_key_event(key_event),
                    },
                    Event::Resize(width, height) => {
                        self.ui.window.resize(width, height);
                        self.ui.window.clear();
                    }
                    _ => (),
                }
            } else {
                // update state
                wpm += 10.0;
                self.ui.stats_line.set_wpm(wpm);
                self.ui
                    .stats_line
                    .set_progress(self.ui.line_block.progress());

                // draw to window
                self.ui.line_block.draw(&mut self.ui.window);
                self.ui.stats_line.draw(&mut self.ui.window);
                self.ui.line_block.reset_cursor(&mut self.ui.window);

                // display window on screen
                self.ui.window.display(buf);
                buf.flush().expect("ERROR: Failed to flush buffer.");
            }
        }
    }
}
