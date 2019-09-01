use cgmath::Point2;
use rgb;

pub use crate::canvas::Canvas;
pub use crate::render::svg::SvgRenderer;
pub use crate::shape::Shape;
pub use crate::style::Style;

pub mod canvas;
pub mod render;
pub mod shape;
pub mod style;

/// Drawings are stored in a vector; this `usize` is a handle to access the child
pub type DrawingId = usize;

pub type Position = Point2<f32>;
pub type RGB = rgb::RGB<u8>;

pub struct Drawing {
    pub shape: Shape,
    pub display_list: DisplayList,
    pub position: Point2<f32>,
    pub style: Style,
}

impl Drawing {
    pub fn new(shape: Shape) -> Drawing {
        Drawing {
            shape,
            style: Style::default(),
            display_list: DisplayList::new(),
            position: Position::new(0.0, 0.0),
        }
    }
}

pub struct DisplayList {
    drawings: Vec<Drawing>,
}

impl DisplayList {
    fn new() -> DisplayList {
        DisplayList { drawings: vec![] }
    }

    /// Adds a drawing to the top of the display list.
    /// Returns a DrawingId handle that can be used to refer to the drawing in the future.
    pub fn add(&mut self, drawing: Drawing) -> DrawingId {
        let child_id = self.drawings.len();
        self.drawings.push(drawing);
        child_id
    }

    pub fn remove(&mut self, _drawing_id: DrawingId) {
        unimplemented!()
    }
}
