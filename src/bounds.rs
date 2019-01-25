use crate::Point;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Bounds {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Bounds {
        Bounds {
            x,
            y,
            width,
            height,
        }
    }

    pub fn top_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn top_right(&self) -> Point {
        Point {
            x: self.x + self.width,
            y: self.y,
        }
    }

    pub fn bot_right(&self) -> Point {
        Point {
            x: self.x + self.width,
            y: self.y + self.height,
        }
    }

    pub fn bot_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + self.height,
        }
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        (x >= self.x) && (x < self.x + self.width) && (y >= self.y) && (y < self.y + self.height)
    }

    pub fn overlaps(&self, other: &Bounds) -> bool {
        if self.top_left() >= other.bot_right() || other.top_left() >= self.bot_right() {
            return false;
        }

        true
    }

    ///
    /// Makes sure that self at least contains other. Will not get smaller to match other, but
    /// will grow if it is currently too small. Used in setting the child bounding boxes that
    /// speed up rendering empty space in Sprite.
    ///
    pub fn surround(&mut self, other: &Bounds) {
        if other.x < self.x {
            let dx = other.x - self.x;
            self.x = other.x;
            self.width += dx;
        }

        if other.y < self.y {
            let dy = other.y - self.y;
            self.y = other.y;
            self.height += dy;
        }

        if other.x + other.width > self.x + self.width {
            self.width += (other.x + other.width) - (self.x + self.width);
        }

        if other.y + other.height > self.y + self.height {
            self.height += (other.y + other.height) - (self.y + self.height);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let bounds = Bounds::new(0, 0, 50, 50);

        assert!(bounds.contains(0, 0)); // tl
        assert!(bounds.contains(49, 49)); // br
        assert!(bounds.contains(49, 0)); // tr
        assert!(bounds.contains(0, 49)); // bl
        assert!(bounds.contains(25, 25)); // middle

        assert_eq!(false, bounds.contains(0, 50));
        assert_eq!(false, bounds.contains(50, 50));

        let bounds = Bounds::new(25, 25, 50, 50);

        assert!(bounds.contains(50, 50));
        assert!(bounds.contains(25, 74));
        assert_eq!(false, bounds.contains(0, 75));
    }

    #[test]
    fn overlaps() {
        let a = Bounds::new(0, 0, 50, 50);
        let b = Bounds::new(0, 0, 50, 50);
        let c = Bounds::new(25, 25, 50, 50);
        let d = Bounds::new(-25, -25, 50, 50);
        let f = Bounds::new(50, 50, 50, 50);

        assert!(a.overlaps(&b));
        assert!(a.overlaps(&c));
        assert!(a.overlaps(&d));
        assert_eq!(false, a.overlaps(&f));
    }

    #[test]
    fn surround() {
        let mut a = Bounds::new(0, 0, 10, 10);
        let b = Bounds::new(20, 20, 5, 5);

        a.surround(&b);

        assert_eq!(a, Bounds::new(0, 0, 25, 25))
    }
}
