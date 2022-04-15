use crossterm::event::KeyCode;
use std::io::{Error, Write};

#[derive(Clone, Copy)]
pub struct WidgetProps {
    pub row_offset: usize,
    pub column_offset: usize,
}

pub trait Widget {
    fn print<T: Write>(&self, buf: &mut T) -> Result<(), Error>;
    fn process_key_code<T: Write>(&mut self, key_code: KeyCode, buf: &mut T) -> Result<(), Error>;
    fn get_widget_props(&self) -> WidgetProps;
    fn get_height(&self) -> usize;
    fn get_width(&self) -> usize;
}
