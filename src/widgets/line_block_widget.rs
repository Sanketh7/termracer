use crossterm::event::KeyCode;
use std::io::{Error, Write};
use std::result::Result;

use crate::widgets::line_widget::{LineWidget, LineWidgetBuf};
use crate::widgets::widget::{Coord, EventHandleableWidget, ViewableWidget, ViewableWidgetProps};

pub struct LineBlockWidget {
    lines: Vec<LineWidget>,
    current_line_index: Option<usize>,
    line_lengths_prefix_sums: Vec<usize>,
    is_all_correct: bool,
    viewable_widget_props: ViewableWidgetProps,
}

impl LineBlockWidget {
    pub fn new(viewable_widget_props: ViewableWidgetProps) -> Self {
        LineBlockWidget {
            lines: Vec::new(),
            current_line_index: None,
            line_lengths_prefix_sums: Vec::new(), // for get_num_correct_characters()
            viewable_widget_props,
            is_all_correct: false,
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
        let line = LineWidget::new(
            text,
            ViewableWidgetProps {
                offset: Coord {
                    row: self.get_offset().row + row,
                    col: self.get_offset().col,
                },
            },
        );

        let line_length = line.get_dimensions().col;
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

    pub fn is_all_correct(&self) -> bool {
        self.is_all_correct
    }

    fn process_enter<'a, T: Write>(&mut self, buf: &'a mut T) -> Result<&'a mut T, Error> {
        match self.current_line_index {
            Some(current_line_index) => {
                if self.lines.get(current_line_index).unwrap().is_all_correct() {
                    if (self.current_line_index.unwrap() + 1) < self.lines.len() {
                        self.current_line_index = Some(self.current_line_index.unwrap() + 1);
                        buf.move_to_user_column(
                            self.lines.get(self.current_line_index.unwrap()).unwrap(),
                        )?;
                    } else {
                        // last line => check if everything is correct
                        self.is_all_correct = self.get_num_correct_characters()
                            == *self.line_lengths_prefix_sums.last().unwrap()
                    }
                }
                Ok(buf)
            }
            None => Ok(buf),
        }
    }
}

impl ViewableWidget for LineBlockWidget {
    fn print<'a, T: Write>(&self, buf: &'a mut T) -> Result<&'a mut T, Error> {
        for line in self.lines.iter() {
            line.print(buf)?;
        }
        if self.current_line_index != None {
            buf.move_to_user_column(self.lines.get(self.current_line_index.unwrap()).unwrap())
        } else {
            Ok(buf)
        }
    }

    fn get_dimensions(&self) -> Coord {
        Coord {
            row: self.lines.iter().map(|x| x.get_dimensions().row).sum(),
            col: self
                .lines
                .iter()
                .map(|x| x.get_dimensions().col)
                .max()
                .unwrap_or(0),
        }
    }

    fn get_viewable_widget_props(&self) -> ViewableWidgetProps {
        self.viewable_widget_props
    }

    fn get_offset(&self) -> Coord {
        self.viewable_widget_props.offset
    }
}

impl EventHandleableWidget for LineBlockWidget {
    fn process_key_code<'a, T: Write>(
        &mut self,
        key_code: KeyCode,
        buf: &'a mut T,
    ) -> Result<&'a mut T, Error> {
        if !self.is_all_correct() {
            if let Some(current_line_index) = self.current_line_index {
                match key_code {
                    KeyCode::Enter => return self.process_enter(buf),
                    _ => {
                        return self
                            .lines
                            .get_mut(current_line_index)
                            .unwrap()
                            .process_key_code(key_code, buf)
                    }
                }
            }
        }
        Ok(buf)
    }
}
