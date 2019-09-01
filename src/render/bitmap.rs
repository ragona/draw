use crate::render::Renderer;
use crate::Canvas;

/// Renders `Canvas` to a PNG format
/// todo: Actually implement
pub struct PNGRenderer {}

impl Renderer for PNGRenderer {
    fn render(&self, _canvas: &Canvas) -> Vec<u8> {
        unimplemented!()
    }
}
