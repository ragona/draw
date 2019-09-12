//! `draw` is a simple 2D vector drawing library.
//!
//! - `Canvas` is a container that defines the size and top-level components of your drawing.
//! - `Drawing` defines the position, style, and sub-components of a drawing.
//! - `Shape` defines the geometry of an individual shape such as a `Circle` or `Line`.
//! - `Style` defines the fill and stroke of a drawing.  
//!
//! The general flow for creating a piece of art is:
//!
//! 1. Create a `Canvas`
//! 2. Create `Drawing` objects and add them to the Canvas `display_list`.
//! 3. Position and style the drawings
//! 4. Render and save the `Canvas` to whatever output format you want. (SVG, PNG, etc...)
//!
//! ## Basic Example
//!
//! ```rust
//! use draw::{Canvas, Drawing, Shape, Style, Fill, Stroke, RGB};
//! use draw::render::{self, svg::SvgRenderer};
//!
//! // create a canvas to draw on
//! let mut canvas = Canvas::new(100, 100);
//!
//! // create a rectangle
//! let mut rect = Drawing::new(Shape::Rectangle {
//!     width: 50,
//!     height: 50,
//! });
//!
//! // move it around
//! rect.position.x = 25.0;
//! rect.position.y = 25.0;
//!
//! // give it a cool style
//! rect.style = Style {
//!     fill: Some(Fill {
//!         color: RGB::new(0, 0, 0),
//!     }),
//!     stroke: Some(Stroke {
//!         width: 2,
//!         color: RGB::new(255, 0, 0),
//!     }),
//! };
//!
//! // add it to the canvas
//! canvas.display_list.add(rect);
//!
//! // save the canvas as an svg
//! render::save(
//!     &canvas,
//!     "tests/svg/basic_end_to_end.svg",
//!     SvgRenderer::new(),
//! )
//! .expect("Failed to save");
//! ```
//!
use cgmath::EuclideanSpace;
use cgmath::Point2;
use rgb;

pub mod canvas;
pub mod drawing;
pub mod render;
pub mod shape;
pub mod style;

pub use crate::canvas::Canvas;
pub use crate::drawing::{DisplayList, Drawing};
pub use crate::render::svg::SvgRenderer;
pub use crate::shape::{LineBuilder, Shape};
pub use crate::style::Color;
pub use crate::style::{Fill, Stroke, Style};

/// Drawings are stored in a vector; this `usize` is a handle to access the child
pub type DrawingId = usize;

pub type Point = Point2<f32>;

/// An alias for RGB<u8>
pub type RGB = rgb::RGB<u8>;
