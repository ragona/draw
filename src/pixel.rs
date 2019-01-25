use rand::Rng;
use std::cmp;
use std::f32;
use std::ops::{self, Add, AddAssign};

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Pixel> for (f32, f32, f32, f32) {
    fn from(pixel: Pixel) -> (f32, f32, f32, f32) {
        (
            f32::from(pixel.r),
            f32::from(pixel.g),
            f32::from(pixel.b),
            f32::from(pixel.a),
        )
    }
}

impl Pixel {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b, a: 255 }
    }

    pub fn grey(value: u8) -> Pixel {
        Pixel {r: value, g: value, b: value, a: 255}
    }

    pub fn new() -> Pixel {
        Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    pub fn as_bytes(self) -> Vec<u8> {
        vec![self.r, self.g, self.b, self.a]
    }

    pub fn black() -> Pixel {
        Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
    pub fn white() -> Pixel {
        Pixel {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
    pub fn red() -> Pixel {
        Pixel {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }
    pub fn green() -> Pixel {
        Pixel {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }
    }
    pub fn blue() -> Pixel {
        Pixel {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }
    pub fn brown() -> Pixel {
        Pixel {
            r: 255,
            g: 255,
            b: 0,
            a: 255,
        }
    }
    pub fn yellow() -> Pixel {
        Pixel {
            r: 0,
            g: 255,
            b: 255,
            a: 255,
        }
    }
    pub fn magenta() -> Pixel {
        Pixel {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    pub fn rand() -> Pixel {
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen_range(0, 255);
        let g: u8 = rng.gen_range(0, 255);
        let b: u8 = rng.gen_range(0, 255);
        let a: u8 = rng.gen_range(0, 255);

        Pixel { r, g, b, a }
    }

    pub fn add(overlay: Pixel, background: Pixel) -> Pixel {
        /*
        Blending transparent pixels is significantly more interesting than I initially guessed!
        Check out the video[1] linked below; it makes a compelling case for the alpha blending
        mode that I've implemented.

        1: Computer Color is Broken: https://www.youtube.com/watch?v=LKnqECcg6Gw
        2: Alpha Compositing: https://en.wikipedia.org/wiki/Alpha_compositing

        todo: Blend modes. This method is fairly expensive, and should be optional.
        */

        // don't try to blend with a fully transparent overlay
        if overlay.a == 0 {
            return background;
        }

        // don't blend with a transparent background
        if background.a == 0 {
            return overlay;
        }

        let overlay: (f32, f32, f32, f32) = overlay.into();
        let background: (f32, f32, f32, f32) = background.into();
        let (r1, g1, b1, a1) = overlay;
        let (r2, g2, b2, a2) = background;
        let opacity = a1 / 255f32;

        let blend = |a: f32, b: f32| {
            let a = a * opacity;
            let b = b * (1f32 - opacity);
            ((a.powf(2.2) + b.powf(2.2)) / 2f32).sqrt()
        };

        let r = cmp::min(blend(r1, r2) as i32, 255) as u8;
        let g = cmp::min(blend(g1, g2) as i32, 255) as u8;
        let b = cmp::min(blend(b1, b2) as i32, 255) as u8;
        let a = cmp::min((a1 + a2) as i32, 255) as u8;

        Pixel { r, g, b, a }
    }
}

fn safe_add(a: u8, b: u8, max: u8) -> u8 {
    let max = cmp::min(255 - a, max);
    if max == 0 {
        return a;
    }

    a + cmp::min(b, max)
}

impl Add for Pixel {
    type Output = Pixel;

    fn add(self, other: Pixel) -> Pixel {
        if self.a == 255 {
            return self;
        }

        Pixel::add(self, other)
    }
}

impl AddAssign for Pixel {
    fn add_assign(&mut self, other: Pixel) {
        *self = Pixel::add(*self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_darker() {
        let black = Pixel::rgba(0, 0, 0, 128);
        let white = Pixel::white();
        let combined = black + white;

        dbg!(combined);
    }

    #[test]
    fn add_lighter() {
        let white = Pixel::rgba(255, 255, 255, 128);
        let black = Pixel::black();
        let combined = white + black;

        dbg!(combined);
    }
}
