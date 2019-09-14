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

pub enum ShapeBuilder {}

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
///
/// let line = LineBuilder::new(0.0, 0.0)
///     .curve_to(50.0, 50.0, 20.0, 30.0)
///     .line_to(50.0, 75.0)
///     .build();
/// ```
pub struct Line {
    start: Point,
    points: Vec<LinePoint>,
}

impl Line {
    /// Create a new Line with `start` as the origin
    pub fn new(start: Point) -> Line {
        Line {
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

    /// Consume the Line, return a `Shape::Line`
    pub fn to_shape(self) -> Shape {
        Shape::Line {
            start: self.start,
            points: self.points,
        }
    }
}

pub struct LineBuilder {
    line: Line,
}

impl LineBuilder {
    pub fn new(x: f32, y: f32) -> LineBuilder {
        LineBuilder {
            line: Line::new(Point::new(x, y)),
        }
    }
    pub fn line_to(mut self, x: f32, y: f32) -> LineBuilder {
        self.line.line_to(Point::new(x, y));
        self
    }

    pub fn curve_to(mut self, x: f32, y: f32, curve_x: f32, curve_y: f32) -> LineBuilder {
        self.line
            .curve_to(Point::new(x, y), Point::new(curve_x, curve_y));
        self
    }

    pub fn build(self) -> Shape {
        self.line.to_shape()
    }
}

impl From<Line> for Shape {
    fn from(line: Line) -> Shape {
        line.to_shape()
    }
}
