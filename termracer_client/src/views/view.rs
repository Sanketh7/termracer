use crate::{rect::Rect, window::Window};
use crossterm::event::KeyEvent;

pub trait View {
    fn draw(&mut self, window: &mut Window);
    fn get_bounds(&self) -> Rect;
}

pub trait KeyEventHandleable {
    fn handle_key_event(&mut self, event: KeyEvent);
}
