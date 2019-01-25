use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::*;

pub fn red_square() -> Sprite {
    let mut scene = Sprite::new(100, 100);

    scene.add_child(
        Rectangle::new(50, 50, 25, 25, Pixel::red())
    );

    scene
}

pub fn bitmap_child() -> Sprite {
    let mut scene = Sprite::new(100, 100);
    let mut bitmap = Bitmap::new(50, 50, Pixel::red());

    bitmap.transform.x = 25;
    bitmap.transform.y = 25;
    scene.add_child(bitmap);

    scene
}

pub fn four_corner_rectangles() -> Sprite {
    let positions: Vec<(i32, i32, Pixel)> = vec![
        (0, 0, Pixel::blue()),     // top left
        (50, 0, Pixel::magenta()), // top right
        (0, 50, Pixel::red()),     // bot left
        (50, 50, Pixel::yellow()), // bot right
    ];
    let mut scene = Sprite::new(100, 100);

    for (x, y, color) in positions {
        scene.add_child(
            Rectangle::new(50, 50, x, y,color)
        );
    }

    scene
}

///
/// Determinstically random pile of rectangles; shouldn't change between runs.
///
pub fn many_rectangles(n: usize) -> Sprite {
    const WIDTH: i32 = 100;
    const HEIGHT: i32 = 100;
    let mut scene = Sprite::new_container(WIDTH, HEIGHT, 8);
    let mut rng = SmallRng::from_seed([0; 16]);

    for _ in 0..n {
        let color = Pixel {
            r: rng.gen_range(0, 255),
            g: rng.gen_range(0, 255),
            b: rng.gen_range(0, 255),
            a: rng.gen_range(0, 255),
        };
        let mut rect = Rectangle::new(10, 10, 0, 0, color);

        rect.transform.x = rng.gen_range(0, WIDTH);
        rect.transform.y = rng.gen_range(0, HEIGHT);

        scene.add_child(rect);
    }

    scene
}

pub fn circle_from_rotated_point() -> Bitmap {
    let mut bitmap = Bitmap::new(100, 100, Pixel::white());
    let point = Point { x: 50, y: 0 };
    let center = Point { x: 50, y: 50 };

    for i in 0..=360 {
        let p = rotate_point(point, center, i);
        bitmap.set_pixel(p.x, p.y, Pixel::red());
    }

    bitmap
}

pub fn rotate_bitmap() -> Sprite {
    let mut parent = Sprite::new(100, 100);
    let mut child = Bitmap::new(50, 50, Pixel::white());

    child.transform.x = 25;
    child.transform.y = 25;
    child.rotate(170);
    parent.add_child(child);

    parent
}

pub fn transparent() -> Sprite {
    let mut black = Pixel::black();
    black.a = 127;

    let mut parent = Sprite::new(100, 100);

    parent.add_child(
        Rectangle::new(50, 50, 25, 25, black)
    );

    parent
}

pub fn circle() -> Circle {
    Circle::new(250, 0, 0, Pixel::red())
}

pub fn circles() -> Sprite {
    let mut parent = Sprite::new(900, 900);
    let points = vec![(450, 450), (300, 300), (600, 300), (600, 600), (300, 600)];

    for point in points {
        parent.add_child(Circle::new(
            250,
            point.0 - 250,
            point.1 - 250,
            Pixel::rgba(255, 0, 0, 50)));
    }

    parent
}

pub fn spiral() -> Sprite {
    let mut parent = Sprite::new(1000, 1000);
    let mut point = Point { x: 500, y: 150 };
    let center = Point { x: 500, y: 500 };
    parent.background(Pixel::black());
    for i in 0..=255 {
        parent.add_child(Circle::new(250, point.x - 250, point.y - 250, Pixel::rgba(255, 0, 0, 1)));
        point = rotate_point(point, center, i);
    }

    parent
}
