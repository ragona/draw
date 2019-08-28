#![allow(dead_code, unused_imports)]

//!
//! # Summary
//! Simple bitmap drawing library. Display lists, transparency, lines and simple vector shapes.
//!
//! # Examples
//!
//! ```
//! use draw::{self, Sprite, SpriteId, Rectangle, Pixel};
//!
//! let mut parent = Sprite::new(100, 100);
//! let mut child = Rectangle::new(50, 50, 25, 25, Pixel::red());
//!
//! parent.add_child(child);
//!
//! draw::save(&parent, "tests/img/example.png");
//! ```
//!#![allow(dead_code, unused_imports)]

//!
//! # Summary
//! Simple bitmap drawing library. Display lists, transparency, lines and simple vector shapes.
//!
//! # Examples
//!
//! ```
//! use draw::{self, Sprite, SpriteId, Rectangle, Pixel};
//!
//! let mut parent = Sprite::new(100, 100);
//! let mut child = Rectangle::new(50, 50, 25, 25, Pixel::red());
//!
//! parent.add_child(child);
//!
//! draw::save(&parent, "tests/img/example.png");
//! ```
//!

use std::env;
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufWriter;
use std::ops::Add;
use std::path::Path;

use itertools::{self, Itertools, iproduct};
use png::{self, Encoder, HasParameters};
use rayon::prelude::*;

pub use crate::bitmap::Bitmap;
pub use crate::bounds::Bounds;
pub use crate::pixel::Pixel;
pub use crate::shapes::{Circle, Rectangle};
pub use crate::sprite::{Sprite, SpriteId};
pub use crate::transform::Transform;

pub mod bitmap;
pub mod bounds;
pub mod error;
pub mod pixel;
pub mod samples;
pub mod shapes;
pub mod sprite;
pub mod transform;

pub type DrawError = error::Error;
pub type ErrorKind = error::ErrorKind;
pub type Result<T> = std::result::Result<T, DrawError>;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {

    pub fn new(x: i32, y: i32) -> Point {
        Point{x, y}
    }

    pub fn dist(self, other: Point) -> u32 {
        /*
        https://en.wikipedia.org/wiki/Euclidean_distance
        */
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f32).sqrt() as u32
    }
}

impl From<Point> for (isize, isize) {
    fn from(point: Point) -> (isize, isize) {
        (point.x as isize, point.y as isize)
    }
}

impl From<[i32;2]> for Point {
    fn from(arr: [i32;2]) -> Point {
        Point::new(arr[0], arr[1])
    }
}

// todo: Split this into multiple traits
pub trait Drawable {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn transform(&self) -> Transform;
    fn set_transform(&mut self, transform: Transform);
    fn get_pixel(&self, x: i32, y: i32) -> Pixel;
    fn rotate(&mut self, degrees: u32);

    fn area(&self) -> usize {
        self.width() as usize * self.height() as usize
    }

    fn bounds(&self) -> Bounds {
        let t = self.transform();
        Bounds {
            x: t.x,
            y: t.y,
            width: self.width(),
            height: self.height(),
        }
    }

    /// Used to arrange the
    fn points(&self) -> Vec<Point> {
        let mut points:Vec<Point> = Vec::with_capacity(self.area());

        for y in 0..self.height() {
            for x in 0..self.width() {
                points.push(Point{y, x})
            }
        }

        points
    }

    fn pixels(&self) -> Vec<Pixel>
    where
        Self: Sync + Sized,
    {
        self.points()
            .into_par_iter()
            .map(|point| self.get_pixel(point.x, point.y))
            .collect()
    }

    fn as_bytes(&self) -> Vec<u8>
    where
        Self: Sync + Sized,
    {
        let pixels = self.pixels();
        let mut bytes = Vec::with_capacity(pixels.len() * 4);

        for pixel in pixels {
            bytes.extend(pixel.as_bytes());
        }

        bytes
    }

    fn into_box(self) -> Box<dyn Drawable>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

/// Save Drawable to the specified location
pub fn save(d: &(impl Drawable + Sync), path: &str) -> Result<()> {
    let width = d.width();
    let height = d.height();
    let path = format!("{}/{}", env::current_dir()?.display(), path);
    let path = Path::new(&path);

    // create file
    let file = File::create(path)?;

    // create image
    let mut encoder = png::Encoder::new(BufWriter::new(file), width as u32, height as u32);

    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);

    /*
    Note: I'm not quite sure what to do here with the errors that can be thrown by the PNG
    encoder. There is an aliased Result type, but it's private to the module, so I can't handle
    it in my own Error type. I've chosen to just panic in the even that the call fails, which
    in this particular case seems okay, but I'm really not sure what the right answer is.
    */

    // write the file
    let mut writer = encoder.write_header().expect("Failed to write header");

    writer
        .write_image_data(&d.as_bytes())
        .expect("Failed to write image data");

    Ok(())
}

///
/// Rotate `point` around `center` by `degrees`.
///
pub fn rotate_point(point: Point, center: Point, degrees: u32) -> Point {
    /*
    https://gamedev.stackexchange.com/questions/86755/how-to-calculate-corner-\
        positions-marks-of-a-rotated-tilted-rectangle

    X = x*cos(θ) - y*sin(θ)
    Y = x*sin(θ) + y*cos(θ)

    This will give you the location of a point rotated θ degrees around the origin. Since the
    corners of the square are rotated around the center of the square and not the origin, a couple
    of steps need to be added to be able to use this formula. First you need to set the point
    relative to the origin. Then you can use the rotation formula. After the rotation you need to
    move it back relative to the center of the square.

    // cx, cy - center of square coordinates
    // x, y - coordinates of a corner point of the square
    // theta is the angle of rotation

    // translate point to origin
    float tempX = x - cx;
    float tempY = y - cy;

    // now apply rotation
    float rotatedX = tempX*cos(theta) - tempY*sin(theta);
    float rotatedY = tempX*sin(theta) + tempY*cos(theta);

    // translate back
    x = rotatedX + cx;
    y = rotatedY + cy;
    */
    let x = point.x as f32;
    let y = point.y as f32;
    let cx = center.x as f32;
    let cy = center.y as f32;
    let theta = degrees as f32 * PI / 180f32;

    let tx = x - cx;
    let ty = y - cy;
    let rx = tx * theta.cos() - ty * theta.sin();
    let ry = tx * theta.sin() + ty * theta.cos();

    Point {
        x: (rx + cx) as i32,
        y: (ry + cy) as i32,
    }
}

// todo add tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_dist() {
        assert_eq!(Point { x: -1, y: -1 }.dist(Point { x: 1, y: 1 }), 2)
    }

    #[test]
    fn test_from_array() {
        let a = [5, 5];
        let b: Point = a.into();

        assert_eq!(Point::new(5, 5), b);
    }
}


use std::env;
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufWriter;
use std::ops::Add;
use std::path::Path;

use itertools::{self, Itertools, iproduct};
use png::{self, Encoder, HasParameters};
use rayon::prelude::*;

pub use crate::bitmap::Bitmap;
pub use crate::bounds::Bounds;
pub use crate::pixel::Pixel;
pub use crate::shapes::{Circle, Rectangle};
pub use crate::sprite::{Sprite, SpriteId};
pub use crate::transform::Transform;

pub mod bitmap;
pub mod bounds;
pub mod error;
pub mod pixel;
pub mod samples;
pub mod shapes;
pub mod sprite;
pub mod transform;

pub type DrawError = error::Error;
pub type ErrorKind = error::ErrorKind;
pub type Result<T> = std::result::Result<T, DrawError>;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {

    pub fn new(x: i32, y: i32) -> Point {
        Point{x, y}
    }

    pub fn dist(self, other: Point) -> u32 {
        /*
        https://en.wikipedia.org/wiki/Euclidean_distance
        */
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f32).sqrt() as u32
    }
}

impl From<Point> for (isize, isize) {
    fn from(point: Point) -> (isize, isize) {
        (point.x as isize, point.y as isize)
    }
}

impl From<[i32;2]> for Point {
    fn from(arr: [i32;2]) -> Point {
        Point::new(arr[0], arr[1])
    }
}

// todo: Split this into multiple traits
pub trait Drawable {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn transform(&self) -> Transform;
    fn set_transform(&mut self, transform: Transform);
    fn get_pixel(&self, x: i32, y: i32) -> Pixel;
    fn rotate(&mut self, degrees: u32);

    fn area(&self) -> usize {
        self.width() as usize * self.height() as usize
    }

    fn bounds(&self) -> Bounds {
        let t = self.transform();
        Bounds {
            x: t.x,
            y: t.y,
            width: self.width(),
            height: self.height(),
        }
    }

    /// Used to arrange the
    fn points(&self) -> Vec<Point> {
        let mut points:Vec<Point> = Vec::with_capacity(self.area());

        for y in 0..self.height() {
            for x in 0..self.width() {
                points.push(Point{y, x})
            }
        }

        points
    }

    fn pixels(&self) -> Vec<Pixel>
    where
        Self: Sync + Sized,
    {
        self.points()
            .into_par_iter()
            .map(|point| self.get_pixel(point.x, point.y))
            .collect()
    }

    fn as_bytes(&self) -> Vec<u8>
    where
        Self: Sync + Sized,
    {
        let pixels = self.pixels();
        let mut bytes = Vec::with_capacity(pixels.len() * 4);

        for pixel in pixels {
            bytes.extend(pixel.as_bytes());
        }

        bytes
    }

    fn into_box(self) -> Box<dyn Drawable>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

/// Save Drawable to the specified location
pub fn save(d: &(impl Drawable + Sync), path: &str) -> Result<()> {
    let width = d.width();
    let height = d.height();
    let path = format!("{}/{}", env::current_dir()?.display(), path);
    let path = Path::new(&path);

    // create file
    let file = File::create(path)?;

    // create image
    let mut encoder = png::Encoder::new(BufWriter::new(file), width as u32, height as u32);

    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);

    /*
    Note: I'm not quite sure what to do here with the errors that can be thrown by the PNG
    encoder. There is an aliased Result type, but it's private to the module, so I can't handle
    it in my own Error type. I've chosen to just panic in the even that the call fails, which
    in this particular case seems okay, but I'm really not sure what the right answer is.
    */

    // write the file
    let mut writer = encoder.write_header().expect("Failed to write header");

    writer
        .write_image_data(&d.as_bytes())
        .expect("Failed to write image data");

    Ok(())
}

///
/// Rotate `point` around `center` by `degrees`.
///
pub fn rotate_point(point: Point, center: Point, degrees: u32) -> Point {
    /*
    https://gamedev.stackexchange.com/questions/86755/how-to-calculate-corner-\
        positions-marks-of-a-rotated-tilted-rectangle

    X = x*cos(θ) - y*sin(θ)
    Y = x*sin(θ) + y*cos(θ)

    This will give you the location of a point rotated θ degrees around the origin. Since the
    corners of the square are rotated around the center of the square and not the origin, a couple
    of steps need to be added to be able to use this formula. First you need to set the point
    relative to the origin. Then you can use the rotation formula. After the rotation you need to
    move it back relative to the center of the square.

    // cx, cy - center of square coordinates
    // x, y - coordinates of a corner point of the square
    // theta is the angle of rotation

    // translate point to origin
    float tempX = x - cx;
    float tempY = y - cy;

    // now apply rotation
    float rotatedX = tempX*cos(theta) - tempY*sin(theta);
    float rotatedY = tempX*sin(theta) + tempY*cos(theta);

    // translate back
    x = rotatedX + cx;
    y = rotatedY + cy;
    */
    let x = point.x as f32;
    let y = point.y as f32;
    let cx = center.x as f32;
    let cy = center.y as f32;
    let theta = degrees as f32 * PI / 180f32;

    let tx = x - cx;
    let ty = y - cy;
    let rx = tx * theta.cos() - ty * theta.sin();
    let ry = tx * theta.sin() + ty * theta.cos();

    Point {
        x: (rx + cx) as i32,
        y: (ry + cy) as i32,
    }
}

// todo add tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_dist() {
        assert_eq!(Point { x: -1, y: -1 }.dist(Point { x: 1, y: 1 }), 2)
    }

    #[test]
    fn test_from_array() {
        let a = [5, 5];
        let b: Point = a.into();

        assert_eq!(Point::new(5, 5), b);
    }
}
