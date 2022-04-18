use crossterm::{
    cursor::{RestorePosition, SavePosition},
    event::KeyCode,
    QueueableCommand,
};
use std::io::{Error, Write};
use std::time::Instant;

use super::constants::CHARACTERS_PER_WORD;
use super::line_block::LineBlock;
use super::stats_line::StatsLine;
use super::widget::{Widget, WidgetProps};

pub struct Session {
    line_block: LineBlock,
    stats_line: StatsLine,
    started_at: Option<Instant>,
    wpm: Option<f32>,
    widget_props: WidgetProps,
}

impl Session {
    pub fn new(text_vec: &Vec<String>, widget_props: WidgetProps) -> Self {
        let mut line_block = LineBlock::new(widget_props);
        for text in text_vec.iter() {
            line_block.new_line(text.to_string());
        }
        let line_block_height = line_block.get_height();

        Session {
            line_block,
            stats_line: StatsLine::new(WidgetProps {
                row_offset: widget_props.row_offset + line_block_height,
                column_offset: widget_props.column_offset,
            }),
            started_at: None,
            wpm: None,
            widget_props,
        }
    }

    pub fn start(&mut self) {
        self.started_at = Some(Instant::now());
    }

    pub fn refresh<T: Write>(&mut self, buf: &mut T) -> Result<(), Error> {
        match self.get_elapsed_seconds() {
            Some(seconds) => {
                let wpm = (self.line_block.get_num_correct_characters() as f32
                    / CHARACTERS_PER_WORD as f32)
                    / (seconds / 60.0);
                self.wpm = Some(wpm);

                self.stats_line.set_wpm(wpm);

                buf.queue(SavePosition)?; // save and restore position to keep cursor inside line block
                self.stats_line.print(buf)?;
                // self.print(buf)?;
                buf.queue(RestorePosition)?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn is_done(&self) -> bool {
        self.line_block.is_all_correct()
    }

    pub fn get_wpm(&self) -> Option<f32> {
        self.wpm
    }

    fn get_elapsed_seconds(&self) -> Option<f32> {
        match self.started_at {
            Some(started_at) => Some(started_at.elapsed().as_secs_f32()),
            None => None,
        }
    }
}

impl Widget for Session {
    fn print<T: Write>(&self, buf: &mut T) -> Result<(), Error> {
        self.stats_line.print(buf)?;
        self.line_block.print(buf)
    }

    fn process_key_code<T: Write>(&mut self, key_code: KeyCode, buf: &mut T) -> Result<(), Error> {
        match self.started_at {
            Some(_) => self.line_block.process_key_code(key_code, buf),
            None => Ok(()),
        }
    }

    fn get_widget_props(&self) -> WidgetProps {
        self.widget_props
    }

    fn get_height(&self) -> usize {
        let line_block_height = self.line_block.get_height();
        line_block_height
    }

    fn get_width(&self) -> usize {
        let line_block_width = self.line_block.get_width();
        line_block_width
    }
}
