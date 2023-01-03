use crossterm::event::KeyEvent;
use std::io::Write;
use crate::rect::Rect;

pub trait View {
    fn display<T: Write>(&mut self, buf: &mut T);
    fn get_bounds(&self) -> Rect;
}

pub trait KeyEventHandleable {
    fn handle_key_event(&mut self, event: KeyEvent);
}
