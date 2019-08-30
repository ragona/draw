use crate::shape::Shape;

pub mod canvas;
pub mod render;
pub mod shape;
pub mod style;

/// Drawings are stored in a vector; this `usize` is a handle to access the child
pub type DrawingId = usize;

pub struct Drawing {
    pub shape: Shape,
    pub display_list: DisplayList,
    //    style,
    //    position,
}

impl Drawing {
    pub fn new(shape: Shape) -> Drawing {
        Drawing {
            shape,
            display_list: DisplayList::new(),
        }
    }
}

pub struct DisplayList {
    drawings: Vec<Drawing>,
}

impl DisplayList {
    fn new() -> DisplayList {
        DisplayList {
            drawings: vec![]
        }
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


#[cfg(test)]
mod tests {
    use super::*;

    use crate::canvas::Canvas;
    use crate::render::svg::SvgRenderer;

    #[test]
    fn basic_end_to_end() {
        // create a canvas to draw on
        let mut canvas = Canvas::new(100, 100, None);

        // create some drawings of rectangles
        let a = Drawing::new(Shape::Rectangle {
            width: 50,
            height: 50,
        });

        let b = Drawing::new(Shape::Rectangle {
            width: 10,
            height: 10,
        });

        // add those drawings to the canvas
        canvas.display_list.add(a);
        canvas.display_list.add(b);

        // save the canvas as an svg
        render::save(&canvas, "tests/svg/basic_end_to_end.svg", SvgRenderer::new())
            .expect("Failed to save");
    }
}
