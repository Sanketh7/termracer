use std::cmp;
use std::io::Write;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode};
use crossterm::{execute, terminal};
use termracer_word_generator::word_generator;
use unicode_segmentation::UnicodeSegmentation;

use crate::framework::split::HorizontalSplitKind;
use crate::framework::window::Window;
use crate::models::game_result::GameResult;
use crate::util::throttler::Throttler;
use crate::views::line_block::LineBlock;
use crate::views::progress_bar::ProgressBar;
use crate::views::stats_line::StatsLine;
use crate::views::view::{KeyEventHandleable, View};

const AVERAGE_WORD_LENGTH: usize = 5;

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
  pub fn new(word_count: usize) -> Self {
    let (term_width, term_height) = terminal::size().expect("ERROR: Failed to get terminal size.");

    // line block takes up the entire terminal width
    // scale down to give breathing room
    // add 1 to account for whitespace
    let words_per_line = ((term_width / (AVERAGE_WORD_LENGTH + 1) as u16) as f32 * 0.6) as usize;
    let all_words = word_generator::generate_words(word_count);
    let text_lines: Vec<Vec<String>> = (0..word_count)
      .step_by(words_per_line)
      .map(|i| {
        all_words[i..cmp::min(i + words_per_line, word_count)]
          .join(" ")
          .graphemes(true)
          .map(String::from)
          .collect()
      })
      .collect();

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

  pub fn run<T: Write>(&mut self, buf: &mut T, poll_duration: Duration) -> GameResult {
    execute!(buf, terminal::EnterAlternateScreen)
      .expect("ERROR: Failed to enter alternate screen.");
    terminal::enable_raw_mode().expect("ERROR: Failed to enable raw mode.");

    let game_results = self.game_loop(buf, poll_duration);

    terminal::disable_raw_mode().expect("ERROR: Failed to disable raw mode.");
    execute!(buf, terminal::LeaveAlternateScreen)
      .expect("ERROR: Failed to leave alternate screen.");

    game_results
  }

  fn game_loop<T: Write>(&mut self, buf: &mut T, poll_duration: Duration) -> GameResult {
    let start_instant = Instant::now();

    let mut throttler = Throttler::new(20);

    loop {
      if event::poll(poll_duration).expect("ERROR: Failed to poll event.") {
        match event::read().expect("ERROR: Failed to read event.") {
          Event::Key(key_event) => match key_event.code {
            KeyCode::Esc => return GameResult::Aborted,
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
        let wpm = (progress.correct as f32)
          / (AVERAGE_WORD_LENGTH as f32)
          / (start_instant.elapsed().as_secs_f32() / 60.0);
        if self.ui.line_block.done() {
          return GameResult::Completed { wpm };
        }
        self.ui.stats_line.set_wpm(wpm);
        self.ui.progress_bar.set_progress(progress);

        // draw to window
        self.ui.line_block.draw(&mut self.ui.window);
        throttler.try_run(|| {
          self.ui.stats_line.draw(&mut self.ui.window);
          self.ui.progress_bar.draw(&mut self.ui.window);
        });
        self.ui.line_block.reset_cursor(&mut self.ui.window);

        // display window on screen
        self.ui.window.display(buf);
        buf.flush().expect("ERROR: Failed to flush buffer.");
      }
    }
  }
}
