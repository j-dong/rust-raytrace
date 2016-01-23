//! Module to load and process textures.
//!
//! Textures are images that are used when rendering. They represent
//! the colors in an object. Right now, it is only being used
//! for the skybox.

use std::path::Path;
use color::Color;

/// A texture stored in memory. It can be loaded from a file, and
/// pixels can be sampled.
pub struct Texture {
    width: u32,
    heigh: u32,
    data: Box<[u8]>,
}

impl Texture {
    /// Load a texture from a file.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Texture, u8> {
        unimplemented!()
    }
    /// Get the color at a position. The parameters are in pixels.
    pub fn at(&self, x: u32, y: u32) -> Color {
        unimplemented!()
    }
    /// Sample the color at a position. The parameters are in the
    /// range [0, 1], and they will be clamped. The result will be
    /// blended.
    pub fn sample(&self, x: f64, y: f64) -> Color {
        unimplemented!()
    }
}
