use crossterm::style::Color;

use super::view::View;
use crate::framework::coord::Coord;
use crate::framework::window::Window;
use crate::models::progress::Progress;

const BAR_SYMBOL: &str = "░";

pub struct ProgressBar {
  region_index: usize,
  progress: Progress,
}

impl ProgressBar {
  pub fn new(region_index: usize) -> Self {
    ProgressBar {
      region_index,
      progress: Progress {
        correct: 0,
        incorrect: 0,
        total: 0,
      },
    }
  }

  pub fn set_progress(&mut self, progress: Progress) {
    assert!(
      progress.correct + progress.incorrect <= progress.total,
      "ERROR: Invalid progress."
    );
    self.progress = progress;
  }
}

impl View for ProgressBar {
  fn draw(&self, window: &mut Window) {
    let total_width = (window
      .region(self.region_index)
      .expect("ERROR: Failed to draw progress bar -- invalid region.")
      .width) as usize;

    let correct_width = if self.progress.correct == self.progress.total {
      total_width
    } else {
      (((self.progress.correct as f32) / (self.progress.total as f32)) * (total_width as f32))
        as usize
    };
    let incorrect_width = if self.progress.incorrect == self.progress.total {
      total_width
    } else {
      (((self.progress.incorrect as f32) / (self.progress.total as f32)) * (total_width as f32))
        as usize
    };

    let total_string = BAR_SYMBOL.repeat(total_width);
    let correct_string = BAR_SYMBOL.repeat(correct_width);
    let incorrect_string = BAR_SYMBOL.repeat(incorrect_width);

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
    window.draw(
      &incorrect_string,
      Color::Red,
      Color::Red,
      Coord {
        row: 0,
        col: correct_width as u16,
      },
      self.region_index,
    );
  }

  fn get_region_index(&self) -> usize {
    self.region_index
  }
}
