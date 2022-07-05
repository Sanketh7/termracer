use crossterm::{
    cursor::MoveTo,
    event::KeyCode,
    style,
    style::{Color, Stylize},
    QueueableCommand,
};
use std::io::{Error, Write};
use std::result::Result;

use crate::widgets::widget::{Coord, EventHandleableWidget, ViewableWidget, ViewableWidgetProps};

pub trait LineWidgetBuf {
    fn move_to_user_column(&mut self, line: &LineWidget) -> Result<&mut Self, Error>;
}

impl<T: Write> LineWidgetBuf for T {
    fn move_to_user_column(&mut self, line: &LineWidget) -> Result<&mut Self, Error> {
        self.queue(MoveTo(
            (line.user_column + line.get_offset().col) as u16,
            line.get_offset().row as u16,
        ))
    }
}

pub struct LineWidget {
    text: String,
    length: usize,
    is_correct: Vec<Option<bool>>, // None if no input yet
    user_column: usize,            // 0-indexed
    viewable_widget_props: ViewableWidgetProps,
}

impl LineWidget {
    pub fn new(text: String, viewable_widget_props: ViewableWidgetProps) -> Self {
        let length = text.chars().count();
        LineWidget {
            text,
            length,
            is_correct: vec![None; length],
            user_column: 0,
            viewable_widget_props,
        }
    }

    pub fn get_num_correct(&self) -> usize {
        self.is_correct
            .iter()
            .map(|&x| if x == Some(true) { 1 } else { 0 })
            .sum()
    }

    fn process_character<'a, T: Write>(
        &mut self,
        c: char,
        buf: &'a mut T,
    ) -> Result<&'a mut T, Error> {
        // don't let the line overflow
        if self.user_column as usize >= self.length {
            return Ok(buf);
        }

        let correct_char = self.text.chars().nth(self.user_column).unwrap();
        self.is_correct[self.user_column] = Some(c == correct_char);
        let ret = buf
            .move_to_user_column(self)?
            .queue(style::PrintStyledContent(match correct_char {
                ' ' => correct_char.on(if c == correct_char {
                    Color::Green
                } else {
                    Color::Red
                }),
                _ => correct_char.with(if c == correct_char {
                    Color::Green
                } else {
                    Color::Red
                }),
            }));
        self.user_column += 1;
        ret
    }

    fn process_backspace<'a, T: Write>(&mut self, buf: &'a mut T) -> Result<&'a mut T, Error> {
        // only move backward if not already at the front
        if self.user_column > 0 {
            self.user_column -= 1;
        }

        // only replace character if in-bounds
        if self.user_column < self.length {
            self.is_correct[self.user_column] = None;
            buf.move_to_user_column(self)?
                .queue(style::Print(
                    self.text.chars().nth(self.user_column).unwrap(),
                ))?
                .move_to_user_column(self)
        } else {
            Ok(buf)
        }
    }

    pub fn is_all_correct(&self) -> bool {
        (self.user_column == self.length) && self.is_correct.iter().all(|&x| x == Some(true))
    }
}

impl ViewableWidget for LineWidget {
    fn print<'a, T: Write>(&self, buf: &'a mut T) -> Result<&'a mut T, Error> {
        buf.move_to_user_column(self)?;

        let mut index = 0;
        for c in self.text.chars() {
            match self.is_correct.get(index) {
                Some(Some(b)) => {
                    buf.queue(style::PrintStyledContent(match c {
                        ' ' => c.on(if *b { Color::Green } else { Color::Red }),
                        _ => c.with(if *b { Color::Green } else { Color::Red }),
                    }))?;
                }
                Some(None) => {
                    buf.queue(style::Print(c))?;
                }
                _ => (),
            };
            index += 1;
        }

        Ok(buf)
    }

    fn get_dimensions(&self) -> Coord {
        Coord {
            row: 1,
            col: self.length,
        }
    }

    fn get_viewable_widget_props(&self) -> ViewableWidgetProps {
        self.viewable_widget_props
    }

    fn get_offset(&self) -> Coord {
        self.viewable_widget_props.offset
    }
}

impl EventHandleableWidget for LineWidget {
    fn process_key_code<'a, T: Write>(
        &mut self,
        key_code: KeyCode,
        buf: &'a mut T,
    ) -> Result<&'a mut T, Error> {
        match key_code {
            KeyCode::Char(c) => self.process_character(c, buf),
            KeyCode::Backspace => self.process_backspace(buf),
            _ => Ok(buf),
        }
    }
}
