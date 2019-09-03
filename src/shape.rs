//! Shape data
use crate::Point;

/// Enum containing all supported shapes
/// todo: Make this Shape<T> to support float sizes
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
        start: Point,
        points: Vec<LinePoint>,
    },
}

/// Enum that describes the various types of lines
#[derive(Debug, Copy, Clone)]
pub enum LinePoint {
    /// A hard corner
    /// `point` - end point of the line segment
    Straight { point: Point },
    /// Curve with single control point at `curve`
    /// `point` - end point of the line segment
    /// `curve` - position the line will curve towards
    QuadraticBezierCurve { point: Point, curve: Point },
    /// Curve with two control points
    /// `point` - end point of the line segment
    /// `curve_a` - control point that influences beginning of line
    /// `curve_a` - control point that influences end of line
    CubicBezierCurve {
        point: Point,
        curve_a: Point,
        curve_b: Point,
    },
}

/// Helper object for building lines; provides a better syntax than building a `Shape::Line`
/// from scratch.
///
/// # Example
///
/// ```
/// use draw::shape::LineBuilder;
/// use draw::{Point, Shape};
/// let mut line = LineBuilder::new(Point::new(0.0, 0.0));
///
/// line.line_to(Point{x: 50.0 ,y: 50.0 });
/// line.curve_to(Point::new(50.0, 50.0), Point::new(20.0, 30.0));
///
/// // Consume the builder, turn the line into a shape for use with the display list
/// let shape: Shape = line.into();
/// ```
pub struct LineBuilder {
    start: Point,
    points: Vec<LinePoint>,
}

impl LineBuilder {
    /// Create a new LineBuilder with `start` as the origin
    pub fn new(start: Point) -> LineBuilder {
        LineBuilder {
            start,
            points: vec![],
        }
    }

    /// Draw a straight line to `point`
    pub fn line_to(&mut self, point: Point) {
        self.points.push(LinePoint::Straight { point });
    }

    /// Draw a curve to `point`, with a single control point at `curve`
    pub fn curve_to(&mut self, point: Point, curve: Point) {
        self.points
            .push(LinePoint::QuadraticBezierCurve { point, curve })
    }

    /// Consume the LineBuilder, return a `Shape::Line`
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
