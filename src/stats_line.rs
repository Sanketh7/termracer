use crossterm::{cursor::MoveTo, style, QueueableCommand};
use std::io::{Error, Write};

use super::widget::{Coord, ViewableWidget, ViewableWidgetProps};

macro_rules! STATS_LINE_FORMAT_STRING {
    () => {
        "WPM: {}"
    };
}

pub struct StatsLine {
    viewable_widget_props: ViewableWidgetProps,
    wpm: f32,
}

impl StatsLine {
    pub fn new(viewable_widget_props: ViewableWidgetProps) -> StatsLine {
        StatsLine {
            viewable_widget_props,
            wpm: 0.0,
        }
    }

    pub fn set_wpm(&mut self, wpm: f32) {
        self.wpm = wpm;
    }
}

impl ViewableWidget for StatsLine {
    fn print<'a, T: Write>(&self, buf: &'a mut T) -> Result<&'a mut T, Error> {
        buf.queue(MoveTo(
            self.get_offset().col as u16,
            self.get_offset().row as u16,
        ))?
        // clear line to prevent artifacts from previous longer numbers
        .queue(style::Print(" ".repeat(self.get_dimensions().col)))?
        .queue(MoveTo(
            self.get_offset().col as u16,
            self.get_offset().row as u16,
        ))?
        .queue(style::Print(format!(
            STATS_LINE_FORMAT_STRING!(),
            self.wpm as u16
        )))
    }

    fn get_dimensions(&self) -> Coord {
        Coord {
            row: 1,
            col: format!(STATS_LINE_FORMAT_STRING!(), self.wpm)
                .chars()
                .count(),
        }
    }

    fn get_viewable_widget_props(&self) -> ViewableWidgetProps {
        self.viewable_widget_props
    }

    fn get_offset(&self) -> Coord {
        self.viewable_widget_props.offset
    }
}
