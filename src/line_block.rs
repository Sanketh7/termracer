use crossterm::event::KeyCode;
use std::io::{Error, Write};
use std::result::Result;

use super::line::Line;
use super::widget::{Widget, WidgetProps};

pub struct LineBlock {
    lines: Vec<Line>,
    current_line_index: Option<usize>,
    line_lengths_prefix_sums: Vec<usize>,
    widget_props: WidgetProps,
}

impl LineBlock {
    pub fn new(widget_props: WidgetProps) -> Self {
        LineBlock {
            lines: Vec::new(),
            current_line_index: None,
            line_lengths_prefix_sums: Vec::new(),
            widget_props,
        }
    }

    pub fn get_num_correct_characters(&self) -> usize {
        match self.current_line_index {
            Some(current_line_index) => (&self.lines[0..=current_line_index])
                .iter()
                .map(|x| x.get_num_correct())
                .sum(),
            None => 0,
        }
    }

    pub fn new_line(&mut self, text: String) {
        let row = self.lines.len();
        let line = Line::new(text, row, self.widget_props);

        let line_length = line.get_length();
        if !self.line_lengths_prefix_sums.is_empty() {
            let last_prefix_sum = *self
                .line_lengths_prefix_sums
                .get(self.line_lengths_prefix_sums.len() - 1)
                .unwrap();
            self.line_lengths_prefix_sums
                .push(last_prefix_sum + line_length);
        } else {
            self.line_lengths_prefix_sums.push(line_length);
        }
        self.lines.push(line);
        if self.current_line_index == None {
            self.current_line_index = Some(0);
        }
    }

    fn process_enter<T: Write>(&mut self, buf: &mut T) -> Result<(), Error> {
        match self.current_line_index {
            Some(current_line_index) => {
                if self.lines.get(current_line_index).unwrap().is_all_correct() {
                    if (self.current_line_index.unwrap() + 1) < self.lines.len() {
                        self.current_line_index = Some(self.current_line_index.unwrap() + 1);
                        self.lines
                            .get(self.current_line_index.unwrap())
                            .unwrap()
                            .move_to_user_column(buf)?;
                    }
                }
                Ok(())
            }
            None => Ok(()),
        }
    }
}

impl Widget for LineBlock {
    fn print<T: Write>(&self, buf: &mut T) -> Result<(), Error> {
        for line in self.lines.iter() {
            line.print(buf)?;
        }
        if self.current_line_index != None {
            self.lines
                .get(self.current_line_index.unwrap())
                .unwrap()
                .move_to_user_column(buf)?;
        }
        Ok(())
    }

    fn process_key_code<T: Write>(&mut self, key_code: KeyCode, buf: &mut T) -> Result<(), Error> {
        match self.current_line_index {
            Some(current_line_index) => match key_code {
                KeyCode::Enter => self.process_enter(buf),
                _ => self
                    .lines
                    .get_mut(current_line_index)
                    .unwrap()
                    .process_key_code(key_code, buf),
            },
            None => Ok(()),
        }
    }

    fn get_widget_props(&self) -> WidgetProps {
        self.widget_props
    }

    fn get_height(&self) -> usize {
        self.lines.iter().map(|x| x.get_height()).sum()
    }

    fn get_width(&self) -> usize {
        match self.lines.iter().map(|x| x.get_width()).max() {
            Some(width) => width,
            None => 0,
        }
    }
}
