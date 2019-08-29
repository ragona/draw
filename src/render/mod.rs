use std::io;

use crate::canvas::Canvas;

pub mod svg;

/// A renderer takes a &Canvas and returns a Vec of bytes.
/// This allows for drawings to be saved, passed to a conversion method, etc.
pub trait Renderer {
    fn render(&self, canvas: &Canvas) -> Vec<u8>;
}

pub fn save(canvas: &Canvas, path: &str, renderer: impl Renderer) -> io::Result<()> {
    let bytes = renderer.render(canvas);
    Ok(())
}