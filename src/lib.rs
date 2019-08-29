pub use svg;

use svg::node::NodeClone;
use svg::Document;
use svg::Node;

use crate::canvas::Canvas;
use crate::shape::Shape;

pub mod canvas;
pub mod shape;
pub mod style;

pub struct Drawing {
    shape: Shape,
    children: Vec<Drawing>,
    //    style,
    //    position,
}

impl Drawing {
    pub fn new(shape: Shape) -> Drawing {
        Drawing {
            shape,
            children: vec![],
        }
    }
}

/// Children are stored in a vector; this `usize` is a handle to access the child
pub type ChildId = usize;

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::shape::Shape;
    use svg::node::element::path::Data;
    use svg::node::element::{Element, Path};
    use svg::{Document, Node};

    #[test]
    fn test_rect() {
        let rect = Shape::Rectangle {
            width: 50,
            height: 50,
        };

        let canvas = Canvas::new(100, 100);
    }

    #[test]
    fn svg() {
        let document = Document::new().set("viewBox", (0, 0, 100, 100));

        svg::save("image.svg", &document).unwrap();
    }
}
