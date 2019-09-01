//! Top level container for a drawing.
use crate::shape::Shape;
use crate::{DisplayList, Drawing};

/// Container for a drawing
pub struct Canvas {
    /// Width of the canvas in pixels
    pub width: u32,
    /// Height of the canvas in pixels
    pub height: u32,
    /// Optional background shape
    pub background: Option<Shape>,
    /// Display list; contains drawings ordered from bottom to top
    pub display_list: DisplayList,
}

impl Canvas {
    /// New empty Canvas with no background
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            background: None,
            display_list: DisplayList::new(),
        }
    }

    /// New canvas with a default background shape
    pub fn with_background(width: u32, height: u32, background: Shape) -> Canvas {
        Canvas {
            width,
            height,
            background: Some(background),
            display_list: DisplayList::new(),
        }
    }

    /// All top level drawings contained in the Canvas
    pub fn drawings(&self) -> &Vec<Drawing> {
        &self.display_list.drawings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_background() {
        let canvas = Canvas::with_background(
            50,
            50,
            Shape::Rectangle {
                width: 50,
                height: 50,
            },
        );

        assert!(canvas.background.is_some());
    }
}
