//! Renders Canvas objects to bytes, allows for saving to disk.
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
    /// Take a reference to a canvas, return a byte array
    fn render(&self, canvas: &Canvas) -> Vec<u8>;
}

/// Saves a canvas to disk. Note: Will overwrite any existing file at the `path` you provide.
/// * `canvas` - Reference to the canvas you want to save
/// * `path` - Full or relative path to where you want to save the file
/// * `renderer` - An object that implements the `Renderer` trait
pub fn save(canvas: &Canvas, path: &str, renderer: impl Renderer) -> io::Result<()> {
    let path = Path::new(path);
    let bytes = renderer.render(canvas);
    let mut file = File::create(&path)?;
    file.write_all(&bytes)?;

    Ok(())
}
