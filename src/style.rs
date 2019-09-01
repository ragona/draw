//! Vector object styles; Fill and Stroke data
use crate::RGB;

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
