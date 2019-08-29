pub use svg::node::element::Path;

use crate::Drawing;
use svg::node::element::Element;
use svg::node::NodeClone;
use svg::{Document, Node};

/// Children are stored in a vector; this `usize` is a handle to access the child
pub type DrawingId = usize;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    drawings: Vec<Drawing>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            drawings: vec![],
        }
    }

    /// Adds a drawing to the top of the canvas display list.
    /// Returns a DrawingId handle that can be used to refer to the drawing in the future.
    pub fn add(&mut self, drawing: Drawing) -> DrawingId {
        let child_id = self.drawings.len();
        self.drawings.push(drawing);
        child_id
    }
}
