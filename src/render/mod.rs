//! Renders Canvas objects to bytes, allows for saving to disk.
use std::fs::{self, File};
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
    if path.is_dir() {
        panic!("Warning: Attempted to save file over an existing directory");
    }
    // create folder if it doesn't exist
    if path.parent().is_none() {
        panic!("Failed to find parent directory")
    }
    let folder = path.parent().unwrap();
    if !Path::exists(folder) {
        fs::create_dir_all(folder)?;
    }
    // render the canvas
    let bytes = renderer.render(canvas);
    // save to disk, overwrite
    let mut file = File::create(&path)?;
    file.write_all(&bytes)?;

    Ok(())
}
