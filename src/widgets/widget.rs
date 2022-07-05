use crossterm::event::KeyCode;
use std::io::{Error, Write};

#[derive(Clone, Copy)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone, Copy)]
pub struct ViewableWidgetProps {
    pub offset: Coord,
}

pub trait ViewableWidget {
    fn print<'a, T: Write>(&self, buf: &'a mut T) -> Result<&'a mut T, Error>;

    fn get_dimensions(&self) -> Coord;

    fn get_viewable_widget_props(&self) -> ViewableWidgetProps;
    fn get_offset(&self) -> Coord;
}

pub trait EventHandleableWidget {
    fn process_key_code<'a, T: Write>(
        &mut self,
        key_code: KeyCode,
        buf: &'a mut T,
    ) -> Result<&'a mut T, Error>;
}
