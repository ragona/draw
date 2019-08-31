use crate::shape::Shape;
use crate::{DisplayList, Drawing};

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub background: Option<Shape>,
    pub display_list: DisplayList,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            background: None,
            display_list: DisplayList::new(),
        }
    }

    pub fn drawings(&self) -> &Vec<Drawing> {
        &self.display_list.drawings
    }
}
