pub use svg::node::element::Path;

use crate::Drawing;
use svg::node::element::Element;
use svg::node::NodeClone;
use svg::{Document, Node};

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

    pub fn add(&mut self) {}
}
