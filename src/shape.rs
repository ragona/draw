//! Shape data
use crate::Position;

/// Enum containing all supported shapes
#[derive(Debug, Clone)]
pub enum Shape {
    Rectangle {
        width: u32,
        height: u32,
    },
    Circle {
        radius: u32,
    },
    Line {
        start: Position,
        points: Vec<LinePoint>,
    },
}

/// Enum that describes the various types of lines
#[derive(Debug, Copy, Clone)]
pub enum LinePoint {
    /// A hard corner
    /// `point` - end point of the line segment
    Straight { point: Position },
    /// Curve with single control point at `curve`
    /// `point` - end point of the line segment
    /// `curve` - position the line will curve towards
    QuadraticBezierCurve { point: Position, curve: Position },
    /// Curve with two control points
    /// `point` - end point of the line segment
    /// `curve_a` - control point that influences beginning of line
    /// `curve_a` - control point that influences end of line
    CubicBezierCurve {
        point: Position,
        curve_a: Position,
        curve_b: Position,
    },
}

/// Helper object for building lines; provides a better syntax than building a `Shape::Line`
/// from scratch.
///
/// # Example
///
/// ```
/// use draw::shape::LineBuilder;
/// use draw::{Position, Shape};
/// let mut line = LineBuilder::new(Position::new(0.0, 0.0));
///
/// line.line_to(Position{x: 50.0 ,y: 50.0 });
/// line.line_to(Position{x: 50.0 ,y: 100.0 });
///
/// // Consume the builder, turn the line into a shape for use with the display list
/// let shape: Shape = line.into();
/// ```
pub struct LineBuilder {
    start: Position,
    points: Vec<LinePoint>,
}

impl LineBuilder {
    pub fn new(start: Position) -> LineBuilder {
        LineBuilder {
            start,
            points: vec![],
        }
    }

    pub fn line_to(&mut self, point: Position) {
        self.points.push(LinePoint::Straight { point });
    }

    pub fn curve_to(&mut self, point: Position, curve: Position) {
        self.points
            .push(LinePoint::QuadraticBezierCurve { point, curve })
    }

    pub fn to_shape(self) -> Shape {
        Shape::Line {
            start: self.start,
            points: self.points,
        }
    }
}

impl From<LineBuilder> for Shape {
    fn from(line: LineBuilder) -> Shape {
        line.to_shape()
    }
}
