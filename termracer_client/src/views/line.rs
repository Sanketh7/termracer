use super::view::{KeyEventHandleable, Rect, View};
use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    queue,
    style::{self, Color, Stylize},
};
use std::io::Write;

struct State {
    // index of current char to be inputted
    index: usize,
    correct: Vec<Option<bool>>,
    dirty: Vec<bool>,
}

pub struct Line {
    text: Vec<char>,
    bounds: Rect,
    state: State,
}

impl Line {
    pub fn new(text: Vec<char>, bounds: Rect) -> Self {
        let length = text.len();
        assert!(
            length <= bounds.width as usize,
            "ERROR: Line length exceeds bounds."
        );
        assert!(bounds.height == 1, "ERROR: Line height must be 1.");
        Line {
            text,
            bounds,
            state: State {
                index: 0,
                correct: vec![None; length],
                dirty: vec![true; length],
            },
        }
    }

    pub fn reset_cursor<T: Write>(&self, buf: &mut T) {
        queue!(
            buf,
            cursor::MoveTo(
                self.bounds.column + (self.state.index as u16),
                self.bounds.row
            )
        )
        .expect("ERROR: Failed to reset cursor position.");
    }

    fn process_character(&mut self, c: char) {
        if self.state.index < self.text.len() {
            self.state.correct[self.state.index] = Some(c == self.text[self.state.index]);
            self.state.dirty[self.state.index] = true;
            self.state.index += 1;
        }
    }

    fn process_backspace(&mut self) {
        if self.state.index > 0 {
            self.state.index -= 1;
            if let Some(correct) = self.state.correct.get_mut(self.state.index) {
                *correct = None;
            }
        }
    }
}

impl View for Line {
    fn display<T: Write>(&self, buf: &mut T) {
        let mut i = 0;
        while i < self.text.len() {
            if self.state.dirty[i] {
                let mut j = i;
                while j < self.text.len() && self.state.dirty[j] {
                    j += 1;
                }

                // print chars from [i, j)
                queue!(
                    buf,
                    cursor::MoveTo(self.bounds.column + (i as u16), self.bounds.row)
                )
                .expect("ERROR: Failed to move cursor position.");
                for (c, correct) in self.text[i..j].iter().zip(&self.state.correct[i..j]) {
                    let styled = match correct {
                        Some(true) => {
                            if c.is_whitespace() {
                                c.on(Color::Green)
                            } else {
                                c.with(Color::Green)
                            }
                        }
                        Some(false) => {
                            if c.is_whitespace() {
                                c.on(Color::Red)
                            } else {
                                c.with(Color::Red)
                            }
                        }
                        None => c.reset(),
                    };
                    queue!(buf, style::PrintStyledContent(styled))
                        .expect("ERROR: Failed to print styled character.");
                }

                i = j;
            } else {
                i += 1;
            }
        }
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}

impl KeyEventHandleable for Line {
    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(c) => self.process_character(c),
            KeyCode::Backspace => self.process_backspace(),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::views::view::Rect;

    #[test]
    fn it_processes_characters() {
        let text = "text";
        let mut line = Line::new(
            text.chars().collect(),
            Rect {
                row: 0,
                column: 0,
                width: 50,
                height: 1,
            },
        );

        line.process_character('t');
        line.process_character('a');
        line.process_character('x');
        line.process_character('t');

        assert_eq!(
            line.state.correct,
            vec![Some(true), Some(false), Some(true), Some(true)]
        );
        assert_eq!(line.state.index, line.text.len());

        line.process_character('t');

        assert_eq!(
            line.state.correct,
            vec![Some(true), Some(false), Some(true), Some(true)]
        );
        assert_eq!(line.state.index, line.text.len());
    }

    #[test]
    fn it_processes_backspaces() {
        let text = "text";
        let mut line = Line::new(
            text.chars().collect(),
            Rect {
                row: 0,
                column: 0,
                width: 50,
                height: 1,
            },
        );

        line.process_backspace();

        assert_eq!(line.state.correct, vec![None; line.text.len()]);
        assert_eq!(line.state.index, 0);

        line.process_character('t');
        line.process_character('a');
        line.process_character('x');
        line.process_character('t');
        line.process_character('t');
        line.process_backspace();
        line.process_backspace();

        assert_eq!(
            line.state.correct,
            vec![Some(true), Some(false), None, None]
        );
        assert_eq!(line.state.index, line.text.len() - 2);
    }
}
