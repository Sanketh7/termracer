use std::io;
use std::io::Write;
use std::result::Result;

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

    pub fn print<T: Write>(&self, buf: &mut T) -> Result<(), io::Error> {
        for line in self.lines.iter() {
            line.print(buf)?;
        }
        if self.current_line_index != None {
            self.lines.get(self.current_line_index.unwrap()).unwrap().move_to_user_column(buf)?;
        }
        Ok(())
    }

    pub fn process_character<T: Write>(&mut self, c: char, buf: &mut T) -> Result<(), io::Error> {
        if self.current_line_index == None {
            return Ok(())
        }

        self.lines.get_mut(self.current_line_index.unwrap()).unwrap().process_character(c, buf)?;
        if self.lines.get(self.current_line_index.unwrap()).unwrap().is_all_correct() {
            if (self.current_line_index.unwrap() + 1) < self.lines.len() {
                self.current_line_index = Some(self.current_line_index.unwrap() + 1);
                self.lines.get(self.current_line_index.unwrap()).unwrap().move_to_user_column(buf)?;
            }
        }
        Ok(())
    }

    pub fn process_backspace<T: Write>(&mut self, buf: &mut T) -> Result<(), io::Error> {
        if self.current_line_index == None {
            return Ok(())
        }

        self.lines.get_mut(self.current_line_index.unwrap()).unwrap().process_backspace(buf)
    }
}