use crossterm::{queue, cursor, style};
use super::view::{View};
use crate::rect::Rect;
use std::io::Write;

macro_rules! STATS_LINE_FORMAT_STRING {
    () => {
        "WPM: {}"
    };
}

struct State {
    wpm: f32,
    dirty: bool,
}

pub struct StatsLine {
    bounds: Rect,
    state: State,
}

impl StatsLine {
    pub fn new(bounds: Rect) -> Self {
        assert_eq!(bounds.height, 1, "ERROR: Status line height must be 1.");
        StatsLine {
            bounds,
            state: State {
                wpm: 0.0,
                dirty: true,
            },
        }
    }

    pub fn set_wpm(&mut self, wpm: f32) {
        self.state.wpm = wpm;
        self.state.dirty = true;
    }
}

impl View for StatsLine {
    fn display<T: Write>(&mut self, buf: &mut T) {
        if self.state.dirty {
            self.state.dirty = false;
            queue!(buf, 
                   cursor::MoveTo(self.bounds.column, self.bounds.row),
                   style::Print(" ".repeat(self.bounds.width as usize)),
                   cursor::MoveTo(self.bounds.column, self.bounds.row),
                   style::Print(format!(STATS_LINE_FORMAT_STRING!(), self.state.wpm as u32))
            ).expect("ERROR: Failed to draw stats line.");
        }
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}
