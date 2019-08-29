use crate::render::Renderer;
use crate::canvas::Canvas;

pub struct SvgRenderer {}

impl Renderer for SvgRenderer {
    fn render(&self, canvas: &Canvas) -> Vec<u8> {
        unimplemented!()
    }
}