use crate::Position;

pub enum Shape {
    Rectangle { width: u32, height: u32 },
    Circle { radius: u32 },
    StraightLine { points: Vec<Position> },
}

pub struct StraightLineBuilder {
    points: Vec<Position>,
}

impl StraightLineBuilder {
    pub fn new(start: Position) -> StraightLineBuilder {
        StraightLineBuilder {
            points: vec![start],
        }
    }

    pub fn line_to(&mut self, point: Position) {
        self.points.push(point);
    }
}
