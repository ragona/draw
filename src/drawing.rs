use crate::*;

/// A drawing composes a `Shape`, a `DisplayList`, a `Position`, and a `Style`. It's a complete
/// sub-tree of an overall piece of art.
pub struct Drawing {
    /// The actual background shape for the drawing
    pub shape: Option<Shape>,
    /// The drawing's children, layered from bottom to top
    pub display_list: DisplayList,
    /// The top left location for the drawing
    pub position: Point,
    /// Fill and stroke information for the drawing
    pub style: Style,
}

impl Drawing {
    /// Create a new drawing with default values.
    pub fn new() -> Drawing {
        Drawing {
            shape: None,
            style: Style::default(),
            display_list: DisplayList::new(),
            position: Point::origin(),
        }
    }

    pub fn with_shape(mut self, shape: Shape) -> Drawing {
        self.shape = Some(shape);
        self
    }

    pub fn with_style(mut self, style: Style) -> Drawing {
        self.style = style;
        self
    }

    pub fn with_position(mut self, position: Point) -> Drawing {
        self.position = position;
        self
    }

    pub fn with_xy(mut self, x: f32, y: f32) -> Drawing {
        self.position = Point { x, y };
        self
    }
}

/// A sorted Vec of Drawings, ordered from bottom to top.
pub struct DisplayList {
    pub drawings: Vec<Drawing>,
}

impl DisplayList {
    /// Create a new empty display list
    pub fn new() -> DisplayList {
        DisplayList { drawings: vec![] }
    }

    /// Adds a drawing to the top of the display list.
    /// Returns a DrawingId handle that can be used to refer to the drawing in the future.
    pub fn add(&mut self, drawing: Drawing) -> DrawingId {
        let child_id = self.drawings.len();
        self.drawings.push(drawing);
        child_id
    }

    /// Remove a drawing from the display list
    /// todo: Implement
    pub fn remove(&mut self, _drawing_id: DrawingId) {
        unimplemented!()
    }
}
