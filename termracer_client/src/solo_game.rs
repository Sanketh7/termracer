use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use crate::{
    layout::HorizontalSplitKind,
    views::{
        line_block::LineBlock,
        progress_bar::ProgressBar,
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
    progress_bar: ProgressBar,
}

pub struct SoloGame {
    ui: UI,
}

impl SoloGame {
    pub fn new() -> Self {
        let text = "A lot of sample text oh boy\n".repeat(10).to_owned();
        let text_lines = text
            .split('\n')
            .map(|line| line.graphemes(true).map(String::from).collect())
            .collect();

        let (term_width, term_height) =
            terminal::size().expect("ERROR: Failed to get terminal size.");
        let mut window = Window::new(term_width, term_height);

        let (line_block_region, bottom_region) =
            window.horizontal_split(HorizontalSplitKind::CellsInBottom(2), 0);
        let (stats_line_region, progress_bar_region) =
            window.horizontal_split(HorizontalSplitKind::CellsInBottom(1), bottom_region);

        let line_block = LineBlock::new(text_lines, line_block_region);
        let stats_line = StatsLine::new(stats_line_region);
        let progress_bar = ProgressBar::new(progress_bar_region);

        SoloGame {
            ui: UI {
                window,
                line_block,
                stats_line,
                progress_bar,
            },
        }
    }

    pub fn game_loop<T: Write>(&mut self, buf: &mut T, poll_duration: Duration) {
        let start_instant = Instant::now();

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
                let progress = self.ui.line_block.progress();
                let wpm = (progress.0 as f32) / 5.0 / (start_instant.elapsed().as_secs_f32() / 60.0);
                self.ui.stats_line.set_wpm(wpm);
                self.ui
                    .progress_bar
                    .set_progress(progress);

                // draw to window
                self.ui.line_block.draw(&mut self.ui.window);
                self.ui.stats_line.draw(&mut self.ui.window);
                self.ui.progress_bar.draw(&mut self.ui.window);
                self.ui.line_block.reset_cursor(&mut self.ui.window);

                // display window on screen
                self.ui.window.display(buf);
                buf.flush().expect("ERROR: Failed to flush buffer.");
            }
        }
    }
}
