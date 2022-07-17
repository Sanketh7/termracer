use ascii::AsciiString;
use crossterm::{
    cursor::{RestorePosition, SavePosition},
    event::KeyCode,
    terminal, QueueableCommand,
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

fn generate_text_vec(num_words: usize, max_line_length: usize) -> Vec<AsciiString> {
    let words = WordGenerator::new().get_random_words(num_words);
    let mut lines: Vec<Vec<AsciiString>> = vec![vec![]];
    let mut curr_line_length = 0;
    for word in words {
        // +1 to account for space
        if curr_line_length + 1 + word.len() <= max_line_length {
            curr_line_length += 1 + word.len();
            lines.last_mut().unwrap().push(word);
        } else {
            curr_line_length = word.len();
            lines.push(vec![word]);
        }
    }
    // combine each line into a space-separated string
    lines
        .iter()
        .map(|line| {
            AsciiString::from_ascii(
                line.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            )
            .unwrap()
        })
        .collect::<Vec<AsciiString>>()
}

impl SessionWidget {
    pub fn new(viewable_widget_props: ViewableWidgetProps, num_words: usize) -> Self {
        // TODO: make getting the width more robust (e.g. widgets have a max size)
        let text_vec = generate_text_vec(num_words, terminal::size().unwrap().0 as usize);
        let mut line_block = LineBlockWidget::new(ViewableWidgetProps {
            offset: viewable_widget_props.offset,
        });
        for text in text_vec.iter() {
            line_block.new_line(text.to_ascii_string());
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
