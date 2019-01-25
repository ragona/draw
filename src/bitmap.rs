use std::cmp;
use std::f32::consts::PI;
use std::mem;

use bresenham::Bresenham;

use super::*;

#[derive(Debug)]
pub struct Bitmap {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Vec<Pixel>>,
    pub transform: Transform,
}

impl Bitmap {
    pub fn new(width: i32, height: i32, color: Pixel) -> Bitmap {
        let mut bitmap = Bitmap {
            width,
            height,
            pixels: vec![],
            transform: Transform::new(),
        };

        bitmap.fill(color);
        bitmap
    }

    pub fn fill(&mut self, pixel: Pixel) {
        let mut pixels: Vec<Vec<Pixel>> = Vec::with_capacity(self.width as usize);
        for _ in 0..self.width {
            pixels.push(vec![pixel; self.height as usize]);
        }
        self.pixels = pixels;

    }

    //    pub fn shear_x(&mut self, v: usize) {
    //        for y in 0..self.height {
    //            for x in 0..self.width {

    //            }
    //        }
    //    }

    pub fn line(&mut self, start: Point, end: Point, color: Pixel) {
        for (x, y) in Bresenham::new(start.into(), end.into()) {
            self.set_pixel(x as i32, y as i32, color);
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Pixel) {
        if (x >= 0) && (y >= 0) && (x < self.width) && (y < self.height) {
            self.pixels[x as usize][y as usize] = color;
        }
    }
}

impl Drawable for Bitmap {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn transform(&self) -> Transform {
        self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        if x < self.width && y < self.height {
            return self.pixels[x as usize][y as usize];
        }

        Pixel::new()
    }

    /// Rotating a bitmap is a destructive operation
    fn rotate(&mut self, degrees: u32) {
        // get the corners
        let bounds = self.bounds();
        let mut top_left = bounds.top_left();
        let mut top_right = bounds.top_right();
        let mut bot_right = bounds.bot_right();
        let mut bot_left = bounds.bot_left();

        // Rotate from the middle of the bitmap
        let cx: f32 = self.width as f32 / 2f32;
        let cy: f32 = self.height as f32 / 2f32;
        let center = Point {
            x: cx as i32,
            y: cy as i32,
        };

        // rotate the corners
        top_left = rotate_point(top_left, center, degrees);
        top_right = rotate_point(top_right, center, degrees);
        bot_left = rotate_point(bot_left, center, degrees);
        bot_right = rotate_point(bot_right, center, degrees);

        // make a new bounding box based on the rotated corners
        let mut min_x = self.width as i32;
        let mut min_y = self.height as i32;
        let mut max_x = -self.width as i32;
        let mut max_y = -self.height as i32;

        for corner in [top_left, top_right, bot_right, bot_left].iter() {
            max_x = cmp::max(corner.x, max_x);
            max_y = cmp::max(corner.y, max_y);
            min_x = cmp::min(corner.x, min_x);
            min_y = cmp::min(corner.y, min_y);
        }

        // todo when transparency is in -- clamp to actual pixels, otherwise this just grows

        // normalize corners so that 0, 0 is still the top left
        for corner in [&mut top_left, &mut top_right, &mut bot_right, &mut bot_left].iter_mut() {
            corner.x -= min_x;
            corner.y -= min_y;
        }

        // recreate our pixel data with the new rotation
        let width_diff = (max_x - min_x) - self.width;
        let height_diff = (max_y - min_y) - self.height;

        let mut new_bitmap = Bitmap::new(
            self.width + width_diff + 2,
            self.height + height_diff + 2,
            Pixel::new(),
        );

        new_bitmap.transform.x = self.transform.x - width_diff / 2;
        new_bitmap.transform.y = self.transform.y - height_diff / 2;

        // Note: This is a cheap method that creates holes. Todo: Try 3 shear method.
        let top = Bresenham::new(top_left.into(), top_right.into());
        let bot = Bresenham::new(bot_left.into(), bot_right.into());
        for (y, (start, end)) in top.zip(bot).enumerate() {
            let line = Bresenham::new(start, end);
            for (x, point) in line.enumerate() {
                new_bitmap.set_pixel(
                    point.0 as i32,
                    point.1 as i32,
                    self.get_pixel(x as i32, y as i32),
                );
            }
        }

        mem::replace(self, new_bitmap);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate() -> Result<()> {
        let mut bitmap = Bitmap::new(100, 100, Pixel::red());

        let x = 0;
        let y = 50;
        let center = Point { x: 50, y: 50 };

        bitmap.set_pixel(x, y, Pixel::white());
        for i in 0..360 {
            let nx = rotate_point(Point { x, y }, center, i);
            bitmap.set_pixel(nx.x, nx.y, Pixel::white());
        }
        bitmap.set_pixel(center.x, center.y, Pixel::black());

        save(&bitmap, "tests/img/bitmap_rotate.png")?;

        Ok(())
    }

    #[test]
    fn line() -> Result<()> {
        let mut bitmap = Bitmap::new(500, 500, Pixel::red());
        for x in 0..20 {
            for y in 0..20 {
                bitmap.line(
                    Point { x: 0, y: x * 10 },
                    Point {
                        x: x * 50,
                        y: y * 50,
                    },
                    Pixel::white(),
                );
            }
        }

        save(&bitmap, "tests/img/bitmap_lines.png")?;

        Ok(())
    }

    /// There's currently a bug with non-square bitmaps. This test is meant to catch it.
    #[test]
    fn test_rectangle_line() -> Result<()> {
        let mut bitmap = Bitmap::new(4, 2, Pixel::new());

        bitmap.line(Point::new(0, 0), Point::new(4, 2), Pixel::black());

        save(&bitmap, "tests/img/rect_line.png")?;

        Ok(())
    }
}
