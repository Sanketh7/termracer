use crossterm::style::Color;

use super::view::View;
use crate::framework::coord::Coord;
use crate::framework::window::Window;

const BAR_SYMBOL: &str = "░";

struct State {
  // (correct, total)
  progress: (usize, usize),
}

pub struct ProgressBar {
  region_index: usize,
  state: State,
}

impl ProgressBar {
  pub fn new(region_index: usize) -> Self {
    ProgressBar {
      region_index,
      state: State { progress: (0, 0) },
    }
  }

  pub fn set_progress(&mut self, progress: (usize, usize)) {
    assert!(progress.0 <= progress.1, "ERROR: Invalid progress.");
    self.state.progress = progress;
  }
}

impl View for ProgressBar {
  fn draw(&self, window: &mut Window) {
    let total_width = (window
      .region(self.region_index)
      .expect("ERROR: Failed to draw progress bar -- invalid region.")
      .width) as usize;

    let correct_width = if self.state.progress.0 == self.state.progress.1 {
      total_width
    } else {
      (((self.state.progress.0 as f32) / (self.state.progress.1 as f32)) * (total_width as f32))
        as usize
    };

    let total_string = BAR_SYMBOL.repeat(total_width);
    let correct_string = BAR_SYMBOL.repeat(correct_width);

    window.clear_region(self.region_index);
    window.draw(
      &total_string,
      Color::White,
      Color::White,
      Coord { row: 0, col: 0 },
      self.region_index,
    );
    window.draw(
      &correct_string,
      Color::Green,
      Color::Green,
      Coord { row: 0, col: 0 },
      self.region_index,
    );
  }

  fn get_region_index(&self) -> usize {
    self.region_index
  }
}
