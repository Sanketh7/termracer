use super::view::{KeyEventHandleable, View};
use crate::{window::Window, rect::Coord};
use crossterm::{
    event::{KeyCode, KeyEvent},
    style::Color,
};
use std::io::Write;

struct State {
    // index of current char to be inputted
    index: usize,
    correct: Vec<Option<bool>>,
}

pub struct Line {
    text: Vec<String>,
    region_index: usize,
    line_index: usize,
    state: State,
}

impl Line {
    pub fn new(text: Vec<String>, region_index: usize, line_index: usize) -> Self {
        let length = text.len();
        Line {
            text,
            region_index,
            line_index,
            state: State {
                index: 0,
                correct: vec![None; length],
            },
        }
    }

    pub fn reset_cursor(&self, window: &mut Window) {
        window.set_cursor(Coord { row: self.line_index as u16, col: self.state.index as u16}, self.region_index);
    }

    pub fn is_correct(&self) -> bool {
        self.state.correct.iter().all(|&x| x == Some(true))
    }

    fn process_character(&mut self, c: char) {
        if self.state.index < self.text.len() {
            self.state.correct[self.state.index] =
                Some(c.to_string() == self.text[self.state.index]);
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
    fn draw(&mut self, window: &mut Window) {
        for (i, c) in self.text.iter().enumerate() {
            let fg = if c.contains(char::is_whitespace) {
                Color::White
            } else {
                match self.state.correct[i] {
                    Some(true) => Color::Green,
                    Some(false) => Color::Red,
                    None => Color::White,
                }
            };

            let bg = if c.contains(char::is_whitespace) {
                match self.state.correct[i] {
                    Some(true) => Color::Green,
                    Some(false) => Color::Red,
                    None => Color::Reset,
                }
            } else {
                Color::Reset
            };

            window.draw(
                c,
                fg,
                bg,
                Coord {row: self.line_index as u16, col: i as u16 },
                self.region_index,
            )
        }
    }

    fn get_region_index(&self) -> usize {
        self.region_index
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
    use unicode_segmentation::UnicodeSegmentation;

    #[test]
    fn it_processes_characters() {
        let text = "text";
        let chars = text.graphemes(true).map(String::from).collect();
        let mut line = Line::new(chars, 0, 0);

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
        let chars = text.graphemes(true).map(String::from).collect();
        let mut line = Line::new(chars, 0, 0);

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

    #[test]
    fn it_checks_correctness() {
        let text = "text";
        let chars = text.graphemes(true).map(String::from).collect();
        let mut line = Line::new(chars, 0, 0);

        line.process_character('t');
        line.process_character('e');
        line.process_character('a');
        line.process_character('t');
        line.process_character('t');

        assert!(!line.is_correct());

        line.process_backspace();
        line.process_backspace();
        line.process_character('x');
        line.process_character('t');

        assert!(line.is_correct());

        line.process_character('z');

        assert!(line.is_correct());
    }
}
