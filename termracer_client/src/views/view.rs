use crossterm::event::KeyEvent;

use crate::framework::window::Window;

pub trait View {
  fn draw(&self, window: &mut Window);
  fn get_region_index(&self) -> usize;
}

pub trait KeyEventHandleable {
  fn handle_key_event(&mut self, event: KeyEvent);
}
