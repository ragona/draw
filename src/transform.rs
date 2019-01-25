#[derive(Debug, Default, Copy, Clone)]
pub struct Transform {
    pub x: i32,
    pub y: i32,
    pub rotation: u32,
}

impl Transform {
    ///
    /// Returns a new Transform
    ///
    pub fn new() -> Transform {
        Transform {
            x: 0,
            y: 0,
            rotation: 0,
        }
    }

    pub fn xy(x: i32, y: i32) -> Transform {
        Transform {x, y, rotation: 0}
    }
}
