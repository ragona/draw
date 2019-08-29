pub use svg;

use svg::node::NodeClone;
use svg::Document;
use svg::Node;

use crate::canvas::Canvas;
use crate::shape::Shape;

pub mod canvas;
pub mod render;
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

#[cfg(test)]
mod tests {
    use super::*;

    use svg::node::element::path::Data;
    use svg::node::element::{Element, Path};
    use svg::{Document, Node};
    use crate::render::svg::SvgRenderer;

    #[test]
    fn basic_end_to_end() {
        let mut canvas = Canvas::new(100, 100);
        let drawing = Drawing::new(Shape::Rectangle {
            width: 50,
            height: 50,
        });

        canvas.add(drawing);
        render::save(&canvas, "my_svg.svg", SvgRenderer{});
    }

    #[test]
    fn svg() {
        let document = Document::new().set("viewBox", (0, 0, 100, 100));

        svg::save("image.svg", &document).unwrap();
    }
}
