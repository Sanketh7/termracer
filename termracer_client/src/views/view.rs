use crossterm::event::KeyEvent;
use std::io::Write;

#[derive(Clone, Copy)]
pub struct Rect {
    pub row: u16,
    pub column: u16,
    pub width: u16,
    pub height: u16,
}

pub trait View {
    fn display<T: Write>(&mut self, buf: &mut T);
    fn get_bounds(&self) -> Rect;
}

pub trait KeyEventHandleable {
    fn handle_key_event(&mut self, event: KeyEvent);
}
