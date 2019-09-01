use crate::Position;

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

impl Shape {
    pub fn new_line(start: Position) -> Self {
        Shape::Line {
            start,
            points: vec![],
        }
    }
}
pub enum LinePoint {
    Straight {
        point: Position,
    },
    QuadraticBezierCurve {
        point: Position,
        curve: Position,
    },
    CubicBezierCurve {
        point: Position,
        curve_a: Position,
        curve_b: Position,
    },
}

pub struct StraightLine {
    start: Position,
    points: Vec<Position>,
}

impl StraightLine {
    pub fn new(start: Position) -> StraightLine {
        StraightLine {
            start,
            points: vec![start],
        }
    }

    pub fn line_to(&mut self, position: Position) {
        self.points.push(position);
    }
}

impl From<StraightLine> for Shape {
    fn from(line: StraightLine) -> Shape {
        Shape::Line {
            start: line.start,
            points: line
                .points
                .iter()
                .map(|p| LinePoint::Straight { point: *p })
                .collect(),
        }
    }
}
