#![allow(dead_code, unused_imports)]

use bresenham::Bresenham;
use draw::{self, Bitmap, Point, Pixel};


fn main() {
    let mut canvas = Bitmap::new(1000, 1000, Pixel::rgb(215, 240, 200));

    let a = Point::new(50, 400);
    let b = Point::new(900, 50);
    let c = Point::new(700, 700);

    let a_to_b = Bresenham::new(b.into(), a.into());
    let b_to_c = Bresenham::new(b.into(), c.into());

    for (start, end) in a_to_b.zip(b_to_c) {
        let (sx, sy) = start;
        let (ex, ey) = end;
        if sx % 10 == 0 {
            canvas.line(
                Point::new(sx as i32, sy as i32),
                Point::new(ex as i32, ey as i32),
                Pixel::black(),
            )
        }
    }


    draw::save(&canvas, "tmp/triangle.png");
}
