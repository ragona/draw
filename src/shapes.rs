use super::*;

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
    pub color: Pixel,
    pub transform: Transform,
}

impl Rectangle {
    pub fn new(width: u32, height: u32, x: i32, y: i32, color: Pixel) -> Rectangle {
        Rectangle {
            width,
            height,
            color,
            transform: Transform{x, y, rotation: 0},
        }
    }
}

impl Drawable for Rectangle {
    fn width(&self) -> i32 {
        self.width as i32
    }

    fn height(&self) -> i32 {
        self.height as i32
    }

    fn transform(&self) -> Transform {
        self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn get_pixel(&self, _: i32, _: i32) -> Pixel {
        self.color
    }

    fn rotate(&mut self, _degrees: u32) {
        unimplemented!() // todo
    }
}

pub struct Circle {
    pub radius: u32,
    pub color: Pixel,
    pub transform: Transform,
}

impl Circle {
    pub fn new(radius: u32, x: i32, y: i32, color: Pixel) -> Circle {
        Circle {
            radius,
            color,
            transform: Transform{x, y, rotation: 0},
        }
    }
}

impl Drawable for Circle {
    fn width(&self) -> i32 {
        (self.radius * 2) as i32
    }

    fn height(&self) -> i32 {
        (self.radius * 2) as i32
    }

    fn transform(&self) -> Transform {
        self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        let center = Point {
            x: self.radius as i32,
            y: self.radius as i32,
        };

        if center.dist(Point { x, y }) < self.radius {
            self.color
        } else {
            Pixel::new()
        }
    }

    fn rotate(&mut self, _degrees: u32) {
        // noop
    }
}
