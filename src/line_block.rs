use std::io::{Write, Error};
use std::result::Result;
use crossterm::event::KeyCode;

use super::line::Line;

pub struct LineBlock {
    lines: Vec<Line>,
    current_line_index: Option<usize>,
}

impl LineBlock {
    pub fn new() -> Self {
        LineBlock {
            lines: Vec::new(),
            current_line_index: None
        }
    }

    pub fn new_line(&mut self, text: String) {
        let row = self.lines.len();
        let line = Line::new(text, row);
        self.lines.push(line);

        if self.current_line_index == None {
            self.current_line_index = Some(0);
        }
    }

    pub fn print<T: Write>(&self, buf: &mut T) -> Result<(), Error> {
        for line in self.lines.iter() {
            line.print(buf)?;
        }
        if self.current_line_index != None {
            self.lines.get(self.current_line_index.unwrap()).unwrap().move_to_user_column(buf)?;
        }
        Ok(())
    }

    pub fn process_key_code<T: Write>(&mut self, key_code: KeyCode, buf: &mut T) -> Result<(), Error> {
        match self.current_line_index {
            Some(current_line_index) => {
                match key_code {
                    KeyCode::Enter => self.process_enter(buf),
                    _ => self.lines.get_mut(current_line_index).unwrap().process_key_code(key_code, buf)
                }
            },
            None => Ok(())
        }
    }

    fn process_enter<T: Write>(&mut self, buf: &mut T) -> Result<(), Error> {
        match self.current_line_index {
            Some(current_line_index) => {
                if self.lines.get(current_line_index).unwrap().is_all_correct() {
                    if (self.current_line_index.unwrap() + 1) < self.lines.len() {
                        self.current_line_index = Some(self.current_line_index.unwrap() + 1);
                        self.lines.get(self.current_line_index.unwrap()).unwrap().move_to_user_column(buf)?;
                    }
                }
                Ok(())
            },
            None => Ok(())
        }
    }
}