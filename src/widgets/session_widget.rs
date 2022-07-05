use crossterm::{
    cursor::{RestorePosition, SavePosition},
    event::KeyCode,
    QueueableCommand,
};
use std::cmp;
use std::io::{Error, Write};
use std::time::Instant;

use crate::constants::CHARACTERS_PER_WORD;
use crate::widgets::line_block_widget::LineBlockWidget;
use crate::widgets::stats_line_widget::StatsLineWidget;
use crate::widgets::widget::{Coord, EventHandleableWidget, ViewableWidget, ViewableWidgetProps};
use crate::word_generator::WordGenerator;

pub struct SessionWidget {
    line_block: LineBlockWidget,
    stats_line: StatsLineWidget,
    started_at: Option<Instant>,
    wpm: Option<f32>,
    viewable_widget_props: ViewableWidgetProps,
}

impl SessionWidget {
    pub fn new(viewable_widget_props: ViewableWidgetProps) -> Self {
        let word_generator = WordGenerator::new();

        let text_vec: Vec<String> = (0..5)
            .map(|_| word_generator.get_random_words(10).join(" "))
            .collect();
        let mut line_block = LineBlockWidget::new(ViewableWidgetProps {
            offset: viewable_widget_props.offset,
        });
        for text in text_vec.iter() {
            line_block.new_line(text.to_string());
        }
        let line_block_height = line_block.get_dimensions().row;

        SessionWidget {
            line_block,
            stats_line: StatsLineWidget::new(ViewableWidgetProps {
                offset: Coord {
                    row: viewable_widget_props.offset.row + line_block_height,
                    col: viewable_widget_props.offset.col,
                },
            }),
            started_at: None,
            wpm: None,
            viewable_widget_props,
        }
    }

    pub fn start(&mut self) {
        self.started_at = Some(Instant::now());
    }

    pub fn refresh<'a, T: Write>(&mut self, buf: &'a mut T) -> Result<&'a mut T, Error> {
        match self.get_elapsed_seconds() {
            Some(seconds) => {
                let wpm = (self.line_block.get_num_correct_characters() as f32
                    / CHARACTERS_PER_WORD as f32)
                    / (seconds / 60.0);
                self.wpm = Some(wpm);

                self.stats_line.set_wpm(wpm);

                buf.queue(SavePosition)?;
                self.stats_line.print(buf)?.queue(RestorePosition)
            }
            None => Ok(buf),
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

impl ViewableWidget for SessionWidget {
    fn print<'a, T: Write>(&self, buf: &'a mut T) -> Result<&'a mut T, Error> {
        self.stats_line.print(buf)?;
        self.line_block.print(buf)
    }

    fn get_dimensions(&self) -> Coord {
        Coord {
            row: self.line_block.get_dimensions().row + self.stats_line.get_dimensions().row,
            col: cmp::max(
                self.line_block.get_dimensions().col,
                self.stats_line.get_dimensions().col,
            ),
        }
    }

    fn get_viewable_widget_props(&self) -> ViewableWidgetProps {
        self.viewable_widget_props
    }

    fn get_offset(&self) -> Coord {
        self.viewable_widget_props.offset
    }
}

impl EventHandleableWidget for SessionWidget {
    fn process_key_code<'a, T: Write>(
        &mut self,
        key_code: KeyCode,
        buf: &'a mut T,
    ) -> Result<&'a mut T, Error> {
        match self.started_at {
            Some(_) => self.line_block.process_key_code(key_code, buf),
            None => Ok(buf),
        }
    }
}
