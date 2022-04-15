use crossterm::{
    cursor::MoveTo,
    event::KeyCode,
    style,
    style::{Color, Stylize},
    QueueableCommand,
};
use std::io::{Error, Write};
use std::result::Result;

use super::widget::{Widget, WidgetProps};

pub struct Line {
    text: String,
    length: usize,
    user_text: String,
    is_correct: Vec<Option<bool>>, // None if no input yet
    row: usize,
    user_column: usize,
    widget_props: WidgetProps,
}

impl Line {
    pub fn new(text: String, row: usize, widget_props: WidgetProps) -> Self {
        let length = text.chars().count();
        Line {
            text,
            length,
            user_text: String::new(),
            is_correct: vec![None; length],
            row,            // 0-indexed
            user_column: 0, // 0-indexed
            widget_props,
        }
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn get_num_correct(&self) -> usize {
        self.is_correct
            .iter()
            .map(|&x| if x == Some(true) { 1 } else { 0 })
            .sum()
    }

    pub fn move_to_user_column<T: Write>(&self, buf: &mut T) -> Result<(), Error> {
        buf.queue(MoveTo(
            (self.user_column + self.widget_props.column_offset) as u16,
            (self.row + self.widget_props.row_offset) as u16,
        ))?;
        Ok(())
    }

    fn process_character<T: Write>(&mut self, c: char, buf: &mut T) -> Result<(), Error> {
        // don't let the line overflow
        if self.user_column as usize >= self.text.len() {
            return Ok(());
        }

        self.move_to_user_column(buf)?;

        let is_correct = c == self.text.chars().nth(self.user_column).unwrap();
        buf.queue(style::PrintStyledContent(match c {
            ' ' => c.on(if is_correct { Color::Green } else { Color::Red }),
            _ => c.with(if is_correct { Color::Green } else { Color::Red }),
        }))?;
        self.is_correct[self.user_column] = Some(is_correct);
        self.user_text.push(c);
        self.user_column += 1;

        Ok(())
    }

    fn process_backspace<T: Write>(&mut self, buf: &mut T) -> Result<(), Error> {
        // only move backward if not already at the front
        if self.user_column > 0 {
            self.user_column -= 1;
        }
        self.move_to_user_column(buf)?;

        // only replace character if in-bounds
        if self.user_column < self.text.len() {
            self.move_to_user_column(buf)?;
            buf.queue(style::Print(
                self.text.chars().nth(self.user_column).unwrap(),
            ))?;
            self.move_to_user_column(buf)?;
        }

        self.user_text.pop();
        Ok(())
    }

    pub fn is_all_correct(&self) -> bool {
        (self.user_column == self.text.len()) && self.is_correct.iter().all(|&x| x == Some(true))
    }
}

impl Widget for Line {
    fn print<T: Write>(&self, buf: &mut T) -> Result<(), Error> {
        self.move_to_user_column(buf)?;
        buf.queue(style::Print(self.text.clone()))?;
        self.move_to_user_column(buf)?;
        Ok(())
    }

    fn process_key_code<T: Write>(&mut self, key_code: KeyCode, buf: &mut T) -> Result<(), Error> {
        match key_code {
            KeyCode::Char(c) => self.process_character(c, buf),
            KeyCode::Backspace => self.process_backspace(buf),
            _ => Ok(()),
        }
    }

    fn get_widget_props(&self) -> WidgetProps {
        self.widget_props
    }

    fn get_height(&self) -> usize {
        1
    }
    fn get_width(&self) -> usize {
        self.get_length()
    }
}
