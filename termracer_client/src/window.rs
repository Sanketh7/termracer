use crate::rect::Rect;
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
    // bounds in world space
    bounds: Rect,
    buffer: Buffer,
    dirty: Vec<Vec<bool>>,
}

impl Window {
    pub fn new(bounds: Rect) -> Self {
        let width = bounds.width;
        let height = bounds.height;
        Window {
            bounds,
            buffer: vec![vec![Cell::new(); width as usize]; height as usize],
            dirty: vec![vec![false; width as usize]; height as usize],
        }
    }

    // row, column, bounds are in window space
    pub fn draw(&mut self, s: &str, fg: Color, bg: Color, row: u16, column: u16, bounds: Rect) {
        let chars: Vec<&str> = s.graphemes(true).collect();
        assert!(
            chars.iter().all(|c| c.width() == 1),
            "ERROR: Window only supports drawing characters with width == 1."
        );

        for (dcol, c) in chars.into_iter().enumerate() {
            if self.check_coord(row, column + (dcol as u16), bounds) {
                let buffer_row = row;
                let buffer_column = column + (dcol as u16);
                self.buffer[buffer_row as usize][buffer_column as usize] = Cell {
                    c: c.to_string(),
                    fg,
                    bg,
                }
            }
        }
    }

    fn check_coord(&self, row: u16, column: u16, bounds: Rect) -> bool {
        let inside_window = row < self.bounds.height && column < self.bounds.width;
        let inside_bounds = row >= bounds.row && row < bounds.height && column >= bounds.column && column < bounds.width;
        inside_window && inside_bounds
    }

    pub fn display<T: Write>(&mut self, buf: &mut T) {
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
    use crossterm::style::Color;
    use crate::rect::Rect;
    use super::Window;

    #[test]
    fn it_draws() {
        let mut window = Window::new(Rect { row: 0, column: 0, width: 3, height: 2});
        window.draw("abcd", Color::Reset, Color::Reset, 0, 0, Rect {row: 0, column: 0, width: 3, height: 2});
        window.draw("ef", Color::Reset, Color::Reset, 1, 1, Rect {row: 0, column: 0, width: 3, height: 2});

        assert_eq!(window.buffer[0][0].c, "a");
        assert_eq!(window.buffer[0][1].c, "b");
        assert_eq!(window.buffer[0][2].c, "c");

        assert_eq!(window.buffer[1][0].c, " ");
        assert_eq!(window.buffer[1][1].c, "e");
        assert_eq!(window.buffer[1][2].c, "f");
    }

    #[test]
    fn it_draws_overlap() {
        let mut window = Window::new(Rect { row: 0, column: 0, width: 3, height: 2});
        window.draw("abcd", Color::Reset, Color::Reset, 0, 0, Rect {row: 0, column: 0, width: 3, height: 2});
        window.draw("ef", Color::Reset, Color::Reset, 0, 1, Rect {row: 0, column: 0, width: 3, height: 2});

        assert_eq!(window.buffer[0][0].c, "a");
        assert_eq!(window.buffer[0][1].c, "e");
        assert_eq!(window.buffer[0][2].c, "f");

        assert_eq!(window.buffer[1][0].c, " ");
        assert_eq!(window.buffer[1][1].c, " ");
        assert_eq!(window.buffer[1][2].c, " ");
    }

    #[test]
    fn it_draws_within_bounds() {
        let mut window = Window::new(Rect { row: 0, column: 0, width: 3, height: 2});
        // outside bounds rect
        window.draw("abcd", Color::Reset, Color::Reset, 0, 0, Rect {row: 0, column: 1, width: 3, height: 2});
        // outside window
        window.draw("ef", Color::Reset, Color::Reset, 1, 2, Rect {row: 0, column: 0, width: 5, height: 2});
        window.draw("gh", Color::Reset, Color::Reset, 2, 0, Rect {row: 0, column: 0, width: 3, height: 5});

        assert_eq!(window.buffer[0][0].c, " ");
        assert_eq!(window.buffer[0][1].c, "b");
        assert_eq!(window.buffer[0][2].c, "c");

        assert_eq!(window.buffer[1][0].c, " ");
        assert_eq!(window.buffer[1][1].c, " ");
        assert_eq!(window.buffer[1][2].c, "e");
    }
}
