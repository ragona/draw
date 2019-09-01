use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use crate::canvas::Canvas;

pub mod bitmap;
pub mod svg;

/// A renderer takes a &Canvas and returns a Vec of bytes.
/// This allows for drawings to be saved, passed to a conversion method, etc.
pub trait Renderer {
    fn render(&self, canvas: &Canvas) -> Vec<u8>;
}

pub fn save(canvas: &Canvas, path: &str, renderer: impl Renderer) -> io::Result<()> {
    let path = Path::new(path);
    let bytes = renderer.render(canvas);
    let mut file = File::create(&path).expect("Failed to create file");
    file.write_all(&bytes).expect("Failed to write file");

    Ok(())
}
