use crate::rect::{HorizontalSplit, Rect, VerticalSplit};
use crossterm::{
    cursor,
    style::{self, Color, Stylize},
    QueueableCommand,
};
use std::io::Write;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Eq, PartialEq)]
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
    regions: Vec<Rect>,
    buffer: Buffer,
    dirty: Vec<Vec<bool>>,
}

impl Window {
    pub fn new(bounds: Rect) -> Self {
        let width = bounds.width;
        let height = bounds.height;
        Window {
            bounds,
            // default region covers entire window
            regions: vec![bounds],
            buffer: vec![vec![Cell::new(); width as usize]; height as usize],
            dirty: vec![vec![false; width as usize]; height as usize],
        }
    }

    pub fn vertical_split(&mut self, split: VerticalSplit, region_index: usize) -> (usize, usize) {
        let region = self
            .regions
            .get_mut(region_index)
            .expect("ERROR: Failed to split region -- invalid region index.");
        let (left, right) = region.vertical_split(split);
        *region = left;
        self.regions.push(right);
        (region_index, self.regions.len() - 1)
    }

    pub fn horizontal_split(
        &mut self,
        split: HorizontalSplit,
        region_index: usize,
    ) -> (usize, usize) {
        let region = self
            .regions
            .get_mut(region_index)
            .expect("ERROR: Failed to split region -- invalid region index.");
        let (top, bottom) = region.horizontal_split(split);
        *region = top;
        self.regions.push(bottom);
        (region_index, self.regions.len() - 1)
    }

    // row and column are relative to region
    pub fn draw(
        &mut self,
        s: &str,
        fg: Color,
        bg: Color,
        region_row: u16,
        region_column: u16,
        region_index: usize,
    ) {
        let chars: Vec<&str> = s.graphemes(true).collect();
        assert!(
            chars.iter().all(|c| c.width() == 1),
            "ERROR: Window only supports drawing characters with width == 1."
        );
        let region_bounds = self
            .regions
            .get(region_index)
            .expect("ERROR: Failed to draw -- invalid region index.");

        for (dcol, c) in chars.into_iter().enumerate() {
            if self.check_coord(region_row, region_column + (dcol as u16), region_index) {
                let window_row = region_bounds.row + region_row;
                let window_column = region_bounds.column + region_column + (dcol as u16);

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

    fn check_coord(&self, region_row: u16, region_column: u16, region_index: usize) -> bool {
        let region_bounds = self
            .regions
            .get(region_index)
            .expect("ERROR: Invalid region index.");
        let window_row = region_row + region_bounds.row;
        let window_column = region_column + region_bounds.column;

        let inside_window = window_row < self.bounds.height && window_column < self.bounds.width;
        let inside_region =
            region_row < region_bounds.height && region_column < region_bounds.width;

        inside_window && inside_region
    }

    pub fn display<T: Write>(&mut self, buf: &mut T) {
        buf.queue(cursor::Hide)
            .expect("ERROR: Failed to hide cursor.");
        for row in 0..self.bounds.height {
            for col in 0..self.bounds.width {
                if self.dirty[row as usize][col as usize] {
                    // clear dirty bit
                    self.dirty[row as usize][col as usize] = false;

                    let cell = &self.buffer[row as usize][col as usize];
                    buf.queue(cursor::MoveTo(
                        col + self.bounds.column,
                        row + self.bounds.row,
                    ))
                    .and_then(|buf| {
                        buf.queue(style::PrintStyledContent(
                            (&cell.c[..]).with(cell.fg).on(cell.bg),
                        ))
                    })
                    .expect("ERROR: Failed to draw cell.");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Window;
    use crate::rect::{HorizontalSplit, Rect, VerticalSplit};
    use crossterm::style::Color;

    #[test]
    fn it_splits_vertically() {
        let mut window = Window::new(Rect {
            row: 0,
            column: 0,
            width: 100,
            height: 50,
        });
        let (left, right) = window.vertical_split(VerticalSplit::CellsinLeft(30), 0);

        assert_eq!((left, right), (0, 1));
        assert_eq!(window.regions[left].width, 30);
        assert_eq!(window.regions[right].width, 70);
    }

    #[test]
    fn it_splits_horizontally() {
        let mut window = Window::new(Rect {
            row: 0,
            column: 0,
            width: 100,
            height: 50,
        });
        let (top, bottom) = window.horizontal_split(HorizontalSplit::CellsinTop(30), 0);

        assert_eq!((top, bottom), (0, 1));
        assert_eq!(window.regions[top].height, 30);
        assert_eq!(window.regions[bottom].height, 20);
    }

    #[test]
    fn it_draws_within_window() {
        let mut window = Window::new(Rect {
            row: 0,
            column: 0,
            width: 3,
            height: 2,
        });
        window.draw("abcd", Color::Reset, Color::Reset, 0, 0, 0);
        window.draw("ef", Color::Reset, Color::Reset, 1, 1, 0);

        assert_eq!(window.buffer[0][0].c, "a");
        assert_eq!(window.buffer[0][1].c, "b");
        assert_eq!(window.buffer[0][2].c, "c");

        assert_eq!(window.buffer[1][0].c, " ");
        assert_eq!(window.buffer[1][1].c, "e");
        assert_eq!(window.buffer[1][2].c, "f");
    }

    #[test]
    fn it_draws_overlap() {
        let mut window = Window::new(Rect {
            row: 0,
            column: 0,
            width: 3,
            height: 2,
        });
        window.draw("abcd", Color::Reset, Color::Reset, 0, 0, 0);
        window.draw("ef", Color::Reset, Color::Reset, 0, 1, 0);

        assert_eq!(window.buffer[0][0].c, "a");
        assert_eq!(window.buffer[0][1].c, "e");
        assert_eq!(window.buffer[0][2].c, "f");

        assert_eq!(window.buffer[1][0].c, " ");
        assert_eq!(window.buffer[1][1].c, " ");
        assert_eq!(window.buffer[1][2].c, " ");
    }

    #[test]
    fn it_draws_within_region() {
        let mut window = Window::new(Rect {
            row: 0,
            column: 0,
            width: 3,
            height: 3,
        });
        let (left, right) = window.vertical_split(VerticalSplit::CellsinLeft(1), 0);
        let (right_top, right_bottom) =
            window.horizontal_split(HorizontalSplit::CellsinTop(1), right);
        /*
        +---+---+---+
        | a | x   y |
        +   +---+---+
        |   |       |
        +   +       +
        |   | d   e |
        +---+---+---+
        */

        window.draw("abc", Color::Reset, Color::Reset, 0, 0, left);
        window.draw("def", Color::Reset, Color::Reset, 1, 0, right_bottom);
        window.draw("xyz", Color::Reset, Color::Reset, 0, 0, right_top);

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
        let mut window = Window::new(Rect {
            row: 0,
            column: 0,
            width: 3,
            height: 2,
        });

        window.draw("abcd", Color::Reset, Color::Reset, 0, 0, 0);

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
        let mut window = Window::new(Rect {
            row: 0,
            column: 0,
            width: 3,
            height: 2,
        });

        window.draw("abcd", Color::Reset, Color::Reset, 0, 0, 0);

        assert_eq!(window.buffer[0][0].c, "a");
        assert_eq!(window.buffer[0][1].c, "b");
        assert_eq!(window.buffer[0][2].c, "c");

        assert_eq!(window.dirty[0][0], true);
        assert_eq!(window.dirty[0][1], true);
        assert_eq!(window.dirty[0][2], true);

        window.display(&mut mock_stdout);

        window.draw("abd", Color::Reset, Color::Reset, 0, 0, 0);

        assert_eq!(window.dirty[0][0], false);
        assert_eq!(window.dirty[0][1], false);
        assert_eq!(window.dirty[0][2], true);
    }
}
