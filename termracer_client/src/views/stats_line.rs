use super::view::View;
use crate::{rect::Rect, window::Window};
use crossterm::style::Color;

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
    fn draw(&mut self, window: &mut Window) {
        if self.state.dirty {
            self.state.dirty = false;

            let s = format!(STATS_LINE_FORMAT_STRING!(), self.state.wpm as u32);

            window.draw(
                &" ".repeat(s.len() + 5),
                Color::Reset,
                Color::Reset,
                self.bounds.row,
                self.bounds.column,
                self.bounds,
            );
            window.draw(
                &s,
                Color::White,
                Color::Reset,
                self.bounds.row,
                self.bounds.column,
                self.bounds,
            );
        }
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}
