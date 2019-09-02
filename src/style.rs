//! Vector object styles; Fill and Stroke data
use crate::RGB;
use rand::Rng;

/// Shape fill
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fill {
    pub color: RGB,
    // todo: Opacity
}

impl Fill {
    pub fn new(color: RGB) -> Fill {
        Fill { color }
    }
}

/// Shape stroke
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Stroke {
    pub width: u32,
    pub color: RGB,
}

impl Stroke {
    pub fn new(width: u32, color: RGB) -> Stroke {
        Stroke { width, color }
    }
}

/// Optional Fill and Stroke
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Style {
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
}

impl Style {
    /// Default empty style with no fill or stroke
    pub fn default() -> Style {
        Style {
            fill: None,
            stroke: None,
        }
    }

    /// New style with both fill and stroke
    pub fn new(fill: Fill, stroke: Stroke) -> Style {
        Style {
            fill: Some(fill),
            stroke: Some(stroke),
        }
    }

    /// New style with only a solid fill color and no stroke
    pub fn filled(color: RGB) -> Style {
        Style {
            fill: Some(Fill::new(color)),
            stroke: None,
        }
    }

    /// New style with only a stroke and no fill
    pub fn stroked(width: u32, color: RGB) -> Style {
        Style {
            fill: None,
            stroke: Some(Stroke::new(width, color)),
        }
    }
}

///Convenience methods for colors
pub struct Color {}

impl Color {
    pub fn black() -> RGB {
        RGB { r: 0, g: 0, b: 0 }
    }
    pub fn gray(shade: u8) -> RGB {
        RGB {
            r: shade,
            g: shade,
            b: shade,
        }
    }
    pub fn random() -> RGB {
        let mut rng = rand::thread_rng();
        RGB {
            r: rng.gen_range(0, 255),
            g: rng.gen_range(0, 255),
            b: rng.gen_range(0, 255),
        }
    }
}
