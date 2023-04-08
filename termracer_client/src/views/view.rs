use crate::window::Window;
use crossterm::event::KeyEvent;

pub trait View {
  fn draw(&self, window: &mut Window);
  fn get_region_index(&self) -> usize;
}

pub trait KeyEventHandleable {
  fn handle_key_event(&mut self, event: KeyEvent);
}
