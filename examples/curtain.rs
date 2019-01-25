#![allow(dead_code, unused_imports)]

use bresenham::Bresenham;
use rand::{self, Rng};

use draw::{self, Bitmap, Circle, Pixel, Point, Result, Sprite};

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut canvas = Bitmap::new(1500, 500, Pixel::rgb(10, 20, 20));
    let mut points:Vec<Vec<Point>> = vec![];

    let width = 350;
    let height = 10;
    let start_x = 50;
    let start_y = 0;
    let noise = 20;
    let x_size = (canvas.width - start_x) / width;
    let y_size = (canvas.height - start_y * 2) / height;

    // build a grid of points spaced around the canvas
    for x in 0..width + 5 {
        let mut column: Vec<Point> = vec![];

        for y in 0..height + 5 {
            let r = rng.gen_range(-noise, noise);
            let point= Point::new(
                start_x + x * x_size + r,
                start_y + y * y_size,
            );

            column.push(point);
        }
        points.push(column);
    }

    for column in points.iter() {
        let mut points_iter = column.iter();
        let mut prev = *points_iter.next().unwrap();
        for point in points_iter {
            canvas.line(prev, *point, Pixel::white());
            prev = *point;
        }
    }

    draw::save(&canvas, "tmp/curtain.png")?;

    Ok(())
}
