use std::io::{Write, Error};
use std::result::Result;
use crossterm::{
    QueueableCommand, 
    cursor, 
    style, style::{Stylize, Color},
};

pub struct Line {
    text: String,
    user_text: String,
    is_correct: Vec<Option<bool>>, // None if no input yet
    row: usize,
    user_column: usize,
}

impl Line {
    pub fn new(text: String, row: usize) -> Self {
        let length = text.len();
        Line {
            text,
            user_text: String::new(),
            is_correct: vec![None; length],
            row, // 0-indexed
            user_column: 0, // 0-indexed
        }
    }

    pub fn move_to_user_column<T: Write>(&self, buf: &mut T) -> Result<(), Error> {
        buf.queue(cursor::MoveTo(self.user_column as u16, self.row as u16))?;
        Ok(())
    }

    pub fn print<T: Write>(&self, buf: &mut T) -> Result<(), Error> {
        self.move_to_user_column(buf)?;
        buf.queue(style::Print(self.text.clone()))?;
        self.move_to_user_column(buf)?;
        Ok(())
    }

    pub fn process_character<T: Write>(&mut self, c: char, buf: &mut T) -> Result<(), Error> {
        // don't let the line overflow
        if self.user_column as usize >= self.text.len() {
            return Ok(())
        }

        self.move_to_user_column(buf)?;

        let is_correct = c == self.text.chars().nth(self.user_column).unwrap();
        buf.queue(style::PrintStyledContent(
            match c {
                ' ' => c.on(if is_correct {Color::Green} else {Color::Red}),
                _ => c.with(if is_correct {Color::Green} else {Color::Red}),
            }
        ))?;
        self.is_correct[self.user_column] = Some(is_correct);
        self.user_text.push(c);
        self.user_column += 1;

        Ok(())
    }

    pub fn process_backspace<T: Write>(&mut self, buf: &mut T) -> Result<(), Error> {
        // only replace character if in-bounds 
        if self.user_column < self.text.len() {
            self.move_to_user_column(buf)?;
            buf.queue(style::Print(self.text.chars().nth(self.user_column).unwrap()))?;
        }

        // only move backward if not already at the front
        if self.user_column > 0 {
            self.user_column -= 1;
            self.move_to_user_column(buf)?;
        } else {
            self.move_to_user_column(buf)?;
        }

        self.user_text.pop();
        Ok(())
    }

    pub fn is_all_correct(&self) -> bool {
        (self.user_column == self.text.len()) && self.is_correct.iter().all(|&x| x == Some(true))
    }
}
