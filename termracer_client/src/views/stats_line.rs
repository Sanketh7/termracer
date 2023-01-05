use super::view::View;
use crate::{rect::Coord, window::Window};
use crossterm::style::Color;

macro_rules! STATS_LINE_FORMAT_STRING {
    () => {
        "WPM: {}    {}/{}"
    };
}

struct State {
    wpm: f32,
    progress: (usize, usize),
}

pub struct StatsLine {
    region_index: usize,
    state: State,
}

impl StatsLine {
    pub fn new(region_index: usize) -> Self {
        StatsLine {
            region_index,
            state: State {
                wpm: 0.0,
                progress: (0, 0),
            },
        }
    }

    pub fn set_wpm(&mut self, wpm: f32) {
        self.state.wpm = wpm;
    }

    pub fn set_progress(&mut self, progress: (usize, usize)) {
        self.state.progress = progress;
    }
}

impl View for StatsLine {
    fn draw(&mut self, window: &mut Window) {
        let s = format!(
            STATS_LINE_FORMAT_STRING!(),
            self.state.wpm as u32, self.state.progress.0, self.state.progress.1
        );
        window.clear_region(self.region_index);
        window.draw(
            &s,
            Color::White,
            Color::Reset,
            Coord { row: 0, col: 0 },
            self.region_index,
        );
    }

    fn get_region_index(&self) -> usize {
        self.region_index
    }
}
