use std::io::{self, Write};

use crossterm::style::{self, Color};
use crossterm::{cursor, queue, QueueableCommand};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::coord::Coord;
use super::layout::Layout;
use super::rect::Rect;
use super::split::{HorizontalSplitKind, VerticalSplitKind};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Cell {
  c: String,
  fg: Color,
  bg: Color,
}

impl Cell {
  fn new() -> Self {
    Cell {
      c: " ".to_owned(),
      fg: Color::Reset,
      bg: Color::Reset,
    }
  }
}

type Buffer = Vec<Vec<Cell>>;

pub struct Window {
  // bounds of entire window
  bounds: Rect,
  layout: Layout,
  buffer: Buffer,
  dirty: Vec<Vec<bool>>,
  cursor_pos: Coord,
}

impl Window {
  pub fn new(width: u16, height: u16) -> Self {
    let bounds = Rect {
      coord: Coord { row: 0, col: 0 },
      width,
      height,
    };
    Window {
      bounds,
      layout: Layout::new(bounds),
      buffer: vec![vec![Cell::new(); width as usize]; height as usize],
      dirty: vec![vec![false; width as usize]; height as usize],
      cursor_pos: Coord { row: 0, col: 0 },
    }
  }

  // row and column are relative to region
  pub fn draw(&mut self, s: &str, fg: Color, bg: Color, region_coord: Coord, region_index: usize) {
    let chars: Vec<&str> = s.graphemes(true).collect();
    assert!(
      chars.iter().all(|c| c.width() == 1),
      "ERROR: Window only supports drawing characters with width == 1."
    );
    let region_bounds = self
      .layout
      .region(region_index)
      .expect("ERROR: Failed to draw -- invalid region index.");

    for (dcol, c) in chars.into_iter().enumerate() {
      if self.check_coord(
        region_coord.row,
        region_coord.col + (dcol as u16),
        region_index,
      ) {
        let window_row = region_bounds.coord.row + region_coord.row;
        let window_column = region_bounds.coord.col + region_coord.col + (dcol as u16);

        let cell = &mut self.buffer[window_row as usize][window_column as usize];
        let new_cell = Cell {
          c: c.to_string(),
          fg,
          bg,
        };

        if cell != &new_cell {
          *cell = new_cell;
          self.dirty[window_row as usize][window_column as usize] = true;
        }
      }
    }
  }

  pub fn set_cursor(&mut self, region_coord: Coord, region_index: usize) {
    let region = self
      .layout
      .region(region_index)
      .expect("ERROR: Failed to set cursor position -- invalid region index.");
    self.cursor_pos = region.coord + region_coord;
  }

  #[allow(dead_code)] 
  pub fn vertical_split(
    &mut self,
    split: VerticalSplitKind,
    region_index: usize,
  ) -> (usize, usize) {
    self.layout.vertical_split(split, region_index)
  }

  pub fn horizontal_split(
    &mut self,
    split: HorizontalSplitKind,
    region_index: usize,
  ) -> (usize, usize) {
    self.layout.horizontal_split(split, region_index)
  }

  pub fn resize(&mut self, new_width: u16, new_height: u16) {
    let new_bounds = Rect {
      coord: Coord { row: 0, col: 0 },
      width: new_width,
      height: new_height,
    };
    self.layout.resize(new_bounds);

    self.bounds = new_bounds;
    self.buffer = vec![vec![Cell::new(); new_width as usize]; new_height as usize];
    self.dirty = vec![vec![true; new_width as usize]; new_height as usize];
  }

  pub fn clear(&mut self) {
    for i in 0..self.layout.regions().len() {
      self.clear_region(i);
    }
  }

  pub fn clear_region(&mut self, region_index: usize) {
    let region_bounds = self
      .layout
      .region(region_index)
      .expect("ERROR: Failed to clear region -- invalid region index.");
    let clear_text = " ".repeat(region_bounds.width as usize);
    for row in 0..region_bounds.height {
      self.draw(
        &clear_text,
        Color::Reset,
        Color::Reset,
        Coord { row, col: 0 },
        region_index,
      );
    }
  }

  pub fn region(&self, region_index: usize) -> Option<&Rect> {
    self.layout.region(region_index)
  }

  fn check_coord(&self, region_row: u16, region_column: u16, region_index: usize) -> bool {
    let region_bounds = self
      .layout
      .region(region_index)
      .expect("ERROR: Invalid region index.");
    let window_row = region_row + region_bounds.coord.row;
    let window_column = region_column + region_bounds.coord.col;

    let inside_window = window_row < self.bounds.height && window_column < self.bounds.width;
    let inside_region = region_row < region_bounds.height && region_column < region_bounds.width;

    inside_window && inside_region
  }

  pub fn display<T: Write>(&mut self, buf: &mut T) {
    let prev_coord: Option<Coord> = None;
    let prev_fg: Option<Color> = None;
    let prev_bg: Option<Color> = None;

    let handle_error = |res: Result<(), io::Error>| res.expect("ERROR: Failed to display cells.");

    for row in 0..self.bounds.height {
      for col in 0..self.bounds.width {
        if self.dirty[row as usize][col as usize] {
          // clear dirty bit
          self.dirty[row as usize][col as usize] = false;

          let cell = &self.buffer[row as usize][col as usize];

          if prev_coord.is_none()
            || row != prev_coord.unwrap().row
            || col != prev_coord.unwrap().col + 1
          {
            handle_error(queue!(
              buf,
              cursor::MoveTo(col + self.bounds.coord.col, row + self.bounds.coord.row)
            ));
          }
          if prev_fg.is_none() || cell.fg != prev_fg.unwrap() {
            handle_error(queue!(buf, style::SetForegroundColor(cell.fg)));
          }
          if prev_bg.is_none() || cell.bg != prev_bg.unwrap() {
            handle_error(queue!(buf, style::SetBackgroundColor(cell.bg)));
          }
          handle_error(queue!(buf, style::Print(&cell.c)));
        }
      }
    }
    buf
      .queue(cursor::MoveTo(self.cursor_pos.col, self.cursor_pos.row))
      .expect("ERROR: Failed to move cursor.");
  }
}

#[cfg(test)]
mod tests {
  use crossterm::style::Color;

  use super::super::coord::Coord;
  use super::super::split::{HorizontalSplitKind, VerticalSplitKind};
  use super::super::window::Window;

  #[test]
  fn it_draws_within_window() {
    let mut window = Window::new(3, 2);
    window.draw(
      "abcd",
      Color::Reset,
      Color::Reset,
      Coord { row: 0, col: 0 },
      0,
    );
    window.draw(
      "ef",
      Color::Reset,
      Color::Reset,
      Coord { row: 1, col: 1 },
      0,
    );

    assert_eq!(window.buffer[0][0].c, "a");
    assert_eq!(window.buffer[0][1].c, "b");
    assert_eq!(window.buffer[0][2].c, "c");

    assert_eq!(window.buffer[1][0].c, " ");
    assert_eq!(window.buffer[1][1].c, "e");
    assert_eq!(window.buffer[1][2].c, "f");
  }

  #[test]
  fn it_draws_overlap() {
    let mut window = Window::new(3, 2);
    window.draw(
      "abcd",
      Color::Reset,
      Color::Reset,
      Coord { row: 0, col: 0 },
      0,
    );
    window.draw(
      "ef",
      Color::Reset,
      Color::Reset,
      Coord { row: 0, col: 1 },
      0,
    );

    assert_eq!(window.buffer[0][0].c, "a");
    assert_eq!(window.buffer[0][1].c, "e");
    assert_eq!(window.buffer[0][2].c, "f");

    assert_eq!(window.buffer[1][0].c, " ");
    assert_eq!(window.buffer[1][1].c, " ");
    assert_eq!(window.buffer[1][2].c, " ");
  }

  #[test]
  fn it_draws_within_region() {
    let mut window = Window::new(3, 3);
    let (left, right) = window.vertical_split(VerticalSplitKind::CellsInLeft(1), 0);
    let (right_top, right_bottom) =
      window.horizontal_split(HorizontalSplitKind::CellsInTop(1), right);
    /*
    +---+---+---+
    | a | x   y |
    +   +---+---+
    |   |       |
    +   +       +
    |   | d   e |
    +---+---+---+
    */

    window.draw(
      "abc",
      Color::Reset,
      Color::Reset,
      Coord { row: 0, col: 0 },
      left,
    );
    window.draw(
      "def",
      Color::Reset,
      Color::Reset,
      Coord { row: 1, col: 0 },
      right_bottom,
    );
    window.draw(
      "xyz",
      Color::Reset,
      Color::Reset,
      Coord { row: 0, col: 0 },
      right_top,
    );

    assert_eq!(window.buffer[0][0].c, "a");
    assert_eq!(window.buffer[0][1].c, "x");
    assert_eq!(window.buffer[0][2].c, "y");

    assert_eq!(window.buffer[1][0].c, " ");
    assert_eq!(window.buffer[1][1].c, " ");
    assert_eq!(window.buffer[1][2].c, " ");

    assert_eq!(window.buffer[2][0].c, " ");
    assert_eq!(window.buffer[2][1].c, "d");
    assert_eq!(window.buffer[2][2].c, "e");
  }

  #[test]
  fn it_sets_dirty_bit() {
    let mut mock_stdout = Vec::new();
    let mut window = Window::new(3, 2);

    window.draw(
      "abcd",
      Color::Reset,
      Color::Reset,
      Coord { row: 0, col: 0 },
      0,
    );

    assert_eq!(window.buffer[0][0].c, "a");
    assert_eq!(window.buffer[0][1].c, "b");
    assert_eq!(window.buffer[0][2].c, "c");

    assert_eq!(window.dirty[0][0], true);
    assert_eq!(window.dirty[0][1], true);
    assert_eq!(window.dirty[0][2], true);

    window.display(&mut mock_stdout);

    assert_eq!(window.dirty[0][0], false);
    assert_eq!(window.dirty[0][1], false);
    assert_eq!(window.dirty[0][2], false);
  }

  #[test]
  fn it_doesnt_set_dirty_bit() {
    let mut mock_stdout = Vec::new();
    let mut window = Window::new(3, 2);

    window.draw(
      "abcd",
      Color::Reset,
      Color::Reset,
      Coord { row: 0, col: 0 },
      0,
    );

    assert_eq!(window.buffer[0][0].c, "a");
    assert_eq!(window.buffer[0][1].c, "b");
    assert_eq!(window.buffer[0][2].c, "c");

    assert_eq!(window.dirty[0][0], true);
    assert_eq!(window.dirty[0][1], true);
    assert_eq!(window.dirty[0][2], true);

    window.display(&mut mock_stdout);

    window.draw(
      "abd",
      Color::Reset,
      Color::Reset,
      Coord { row: 0, col: 0 },
      0,
    );

    assert_eq!(window.dirty[0][0], false);
    assert_eq!(window.dirty[0][1], false);
    assert_eq!(window.dirty[0][2], true);
  }

  #[test]
  fn it_clears_region() {
    let mut window = Window::new(3, 3);
    let (left, right) = window.vertical_split(VerticalSplitKind::CellsInLeft(1), 0);
    let (right_top, right_bottom) =
      window.horizontal_split(HorizontalSplitKind::CellsInTop(1), right);
    /*
    +---+---+---+
    | a | x   y |
    +   +---+---+
    |   |       |
    +   +       +
    |   |       |
    +---+---+---+
    */

    window.draw(
      "abc",
      Color::Reset,
      Color::Reset,
      Coord { row: 0, col: 0 },
      left,
    );
    window.draw(
      "def",
      Color::Reset,
      Color::Reset,
      Coord { row: 1, col: 1 },
      right_bottom,
    );
    window.draw(
      "xyz",
      Color::Reset,
      Color::Reset,
      Coord { row: 0, col: 0 },
      right_top,
    );

    window.clear_region(right_bottom);

    assert_eq!(window.buffer[0][0].c, "a");
    assert_eq!(window.buffer[0][1].c, "x");
    assert_eq!(window.buffer[0][2].c, "y");

    assert_eq!(window.buffer[1][0].c, " ");
    assert_eq!(window.buffer[1][1].c, " ");
    assert_eq!(window.buffer[1][2].c, " ");

    assert_eq!(window.buffer[2][0].c, " ");
    assert_eq!(window.buffer[2][1].c, " ");
    assert_eq!(window.buffer[2][2].c, " ");
  }
}
