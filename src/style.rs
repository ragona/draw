use crate::RGB;

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

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Style {
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
}

impl Style {
    pub fn default() -> Style {
        Style {
            fill: None,
            stroke: None,
        }
    }

    pub fn new(fill: Fill, stroke: Stroke) -> Style {
        Style {
            fill: Some(fill),
            stroke: Some(stroke),
        }
    }

    pub fn filled(color: RGB) -> Style {
        Style {
            fill: Some(Fill::new(color)),
            stroke: None,
        }
    }

    pub fn stroked(width: u32, color: RGB) -> Style {
        Style {
            fill: None,
            stroke: Some(Stroke::new(width, color)),
        }
    }
}
