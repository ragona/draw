use crate::{Drawing, DisplayList};
use crate::shape::Shape;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub background: Option<Shape>,
    pub display_list: DisplayList,
}

impl Canvas {
    pub fn new(width: u32, height: u32, background: Option<Shape>) -> Canvas {
        Canvas {
            width,
            height,
            background,
            display_list: DisplayList::new(),
        }
    }

    pub fn drawings(&self) -> &Vec<Drawing> {
        &self.display_list.drawings
    }
}
