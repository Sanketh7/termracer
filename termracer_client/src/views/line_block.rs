use super::{
    line::Line,
    view::{KeyEventHandleable, View},
};
use crate::window::Window;
use crossterm::event::{KeyCode, KeyEvent};
use std::io::Write;

struct State {
    // index of current line
    index: usize,
    correct: Vec<bool>,
}

pub struct LineBlock {
    lines: Vec<Line>,
    region_index: usize,
    state: State,
}

impl LineBlock {
    pub fn new(text_lines: Vec<Vec<String>>, region_index: usize) -> Self {
        let length = text_lines.len();
        LineBlock {
            lines: text_lines
                .into_iter()
                .enumerate()
                .map(|(line_index, text)| Line::new(text, region_index, line_index))
                .collect(),
            region_index,
            state: State {
                index: 0,
                correct: vec![false; length],
            },
        }
    }

    pub fn reset_cursor<T: Write>(&self, buf: &mut T) {
        if let Some(line) = self.lines.get(self.state.index) {
            line.reset_cursor(buf);
        } else if let Some(line) = self.lines.get(self.state.index - 1) {
            line.reset_cursor(buf);
        }
    }

    pub fn is_correct(&self) -> bool {
        self.state.correct.iter().all(|&x| x)
    }

    fn process_enter(&mut self) {
        if let Some(line) = self.lines.get_mut(self.state.index) {
            if line.is_correct() {
                self.state.index += 1;
            }
        }
    }
}

impl View for LineBlock {
    fn draw(&mut self, window: &mut Window) {
        for line in &mut self.lines {
            line.draw(window);
        }
    }

    fn get_region_index(&self) -> usize {
        self.region_index
    }
}

impl KeyEventHandleable for LineBlock {
    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Enter => self.process_enter(),
            _ => {
                if let Some(line) = self.lines.get_mut(self.state.index) {
                    line.handle_key_event(event)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LineBlock;
    use crate::views::view::KeyEventHandleable;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use unicode_segmentation::UnicodeSegmentation;

    fn create_char_key_event(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }

    #[test]
    fn it_goes_to_next_line() {
        let text = "ab\ncd".to_owned();
        let text_lines = text
            .split('\n')
            .map(|line| line.graphemes(true).map(String::from).collect())
            .collect();
        let mut block = LineBlock::new(text_lines, 0);

        block.handle_key_event(create_char_key_event(KeyCode::Char('a')));
        block.handle_key_event(create_char_key_event(KeyCode::Char('c')));
        block.handle_key_event(create_char_key_event(KeyCode::Backspace));
        block.handle_key_event(create_char_key_event(KeyCode::Char('b')));
        block.handle_key_event(create_char_key_event(KeyCode::Enter));

        assert_eq!(block.state.index, 1);

        block.handle_key_event(create_char_key_event(KeyCode::Char('c')));
        block.handle_key_event(create_char_key_event(KeyCode::Char('d')));
        block.handle_key_event(create_char_key_event(KeyCode::Enter));

        assert_eq!(block.state.index, 2);
    }
}
