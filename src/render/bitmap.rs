//! Render Canvas to bitmap formats (PNG, JPG, etc...)
use crate::render::{svg::SvgRenderer, Renderer};
use crate::Canvas;
use usvg::TreeParsing;

/// Renders `Canvas` to a PNG format
pub struct PngRenderer {
    svg_renderer: SvgRenderer,
}

impl PngRenderer {
    pub fn new() -> Self {
        PngRenderer {
            svg_renderer: SvgRenderer::new(),
        }
    }
}

impl Renderer for PngRenderer {
    fn render(&self, canvas: &Canvas) -> Vec<u8> {
        let opt = usvg::Options::default();

        let svg_data = self.svg_renderer.render(canvas);
        let rtree = usvg::Tree::from_data(&svg_data, &opt).unwrap();
        let mut pixmap = resvg::tiny_skia::Pixmap::new(canvas.width, canvas.height).unwrap();

        resvg::Tree::from_usvg(&rtree).render(usvg::Transform::default(), &mut pixmap.as_mut());

        pixmap.encode_png().unwrap()
    }
}
