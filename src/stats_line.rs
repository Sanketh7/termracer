use crossterm::{cursor::MoveTo, event::KeyCode, style, QueueableCommand};
use std::io::{Error, Write};

use super::widget::{Widget, WidgetProps};

macro_rules! FORMAT_STRING {
    () => {
        "WPM: {}"
    };
}

pub struct StatsLine {
    widget_props: WidgetProps,
    wpm: f32,
}

impl StatsLine {
    pub fn new(widget_props: WidgetProps) -> StatsLine {
        StatsLine {
            widget_props,
            wpm: 0.0,
        }
    }

    pub fn set_wpm(&mut self, wpm: f32) {
        self.wpm = wpm;
    }
}

impl Widget for StatsLine {
    fn print<T: Write>(&self, buf: &mut T) -> Result<(), Error> {
        buf.queue(MoveTo(
            self.widget_props.column_offset as u16,
            self.widget_props.row_offset as u16,
        ))?
        // clear line to prevent artifacts from previous longer numbers
        .queue(style::Print(" ".repeat(self.get_width())))?
        .queue(MoveTo(
            self.widget_props.column_offset as u16,
            self.widget_props.row_offset as u16,
        ))?
        .queue(style::Print(format!(FORMAT_STRING!(), self.wpm as u16)))?;
        Ok(())
    }

    fn process_key_code<T: Write>(&mut self, _: KeyCode, _: &mut T) -> Result<(), Error> {
        Ok(())
    }

    fn get_widget_props(&self) -> WidgetProps {
        self.widget_props
    }

    fn get_height(&self) -> usize {
        1
    }

    fn get_width(&self) -> usize {
        format!(FORMAT_STRING!(), self.wpm).chars().count()
    }
}
