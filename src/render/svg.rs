use svg::{Document, Node};

use crate::canvas::Canvas;
use crate::render::Renderer;
use crate::shape::Shape;
use crate::style::Style;
use crate::{Drawing, Position, RGB};
use svg::node::element::{tag, Element};

/// Renders the canvas as an SVG
pub struct SvgRenderer {}

impl SvgRenderer {
    pub fn new() -> SvgRenderer {
        SvgRenderer {}
    }
}

impl Renderer for SvgRenderer {
    fn render(&self, canvas: &Canvas) -> Vec<u8> {
        // create a new svg document
        let mut document = Document::new().set("viewBox", (0, 0, canvas.width, canvas.height));
        // first render the background
        if let Some(shape) = &canvas.background {
            let origin = Position::new(0.0, 0.0);
            document = render_shape(shape, &origin, &Style::default(), document);
        }
        // render all drawings from the bottom up
        for drawing in canvas.drawings() {
            document = render_drawing(drawing, document)
        }
        // return a byte array
        document.to_string().into_bytes()
    }
}

/// Recursively render the drawing from the bottom up
fn render_drawing(drawing: &Drawing, mut document: Document) -> Document {
    // first, render this drawing's shape
    document = render_shape(&drawing.shape, &drawing.position, &drawing.style, document);
    // next, render each drawing from the bottom up
    for drawing in &drawing.display_list.drawings {
        document = render_drawing(drawing, document);
    }
    // finally, return the composed document
    document
}

fn render_shape(shape: &Shape, position: &Position, style: &Style, document: Document) -> Document {
    let mut element;
    // start by setting the shape of the element
    match shape {
        Shape::Rectangle { width, height } => {
            element = Element::new(tag::Rectangle);
            element.assign("width", *width);
            element.assign("height", *height);
        }
    }
    // set the style of the element
    if let Some(fill) = &style.fill {
        element.assign("fill", rgb_to_str(&fill.color));
    }
    if let Some(stroke) = &style.stroke {
        element.assign("stroke", rgb_to_str(&stroke.color));
        element.assign("stroke-width", format!("{}", stroke.width));
    }
    // set the position of the element
    element.assign("x", position.x);
    element.assign("y", position.y);
    //add the element to the document
    document.add(element)
}

fn rgb_to_str(color: &RGB) -> String {
    format!("rgb({},{},{})", color.r, color.g, color.b)
}
