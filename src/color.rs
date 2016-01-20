//! Struct representing color
//!
//! This module contains the `Color` struct representing the light
//! that is transferred by light sources. It is transformed into
//! RGB at the end of the rendering process.

use std::ops::{Add, Sub, Mul, Div};

/// A color which can be transformed into RGB components. Currently
/// colors are stored as three `f64` RGB components.
///
/// All components are normally in the range [0, 1], but colors can
/// go beyond that (e.g. when adding colors).
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Color {
    /// The red component of the color.
    pub r: f64,
    /// The green component of the color.
    pub g: f64,
    /// The blue component of the color.
    pub b: f64,
}

/// The color black.
pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {r: self.r + other.r, g: self.g + other.g, b: self.b + other.b}
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color {r: self.r - other.r, g: self.g - other.g, b: self.b - other.b}
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {r: self.r * other.r, g: self.g * other.g, b: self.b * other.b}
    }
}

impl Div for Color {
    type Output = Color;
    fn div(self, other: Color) -> Color {
        Color {r: self.r / other.r, g: self.g / other.g, b: self.b / other.b}
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color {
        Color {r: self.r * other, g: self.g * other, b: self.b * other}
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, other: f64) -> Color {
        Color {r: self.r / other, g: self.g / other, b: self.b / other}
    }
}

fn clamp_color_val(val: f64) -> u8 {
    let x = val * 256.0;
    if x < 0.0 {0} else if x >= 255.0 {255} else {x.trunc() as u8}
}

fn to_srgb(val: f64) -> u8 {
    // TODO: faster, using lookup table
    clamp_color_val(
        if val <= 0.03928 {
            val / 12.92
        } else {
            ((val + 0.055) / 1.055).powf(2.4)
        }
    )
}

impl Color {
    /// Creates a color from RGB components. The resulting color
    /// has components are that not clamped to [0, 1).
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Color {
        Color {r: r, g: g, b: b}
    }

    /// Get the RGB components as 3 bytes, useful for writing an
    /// image.
    pub fn rgb(&self) -> [u8; 3] {
        [clamp_color_val(self.r), clamp_color_val(self.g), clamp_color_val(self.b)]
    }

    /// Get the components of the image in BGR as 3 bytes, useful
    /// for writing an image.
    pub fn bgr(&self) -> [u8; 3] {
        [clamp_color_val(self.b), clamp_color_val(self.g), clamp_color_val(self.r)]
    }

    /// Write this color to the ith position in the row buffer.
    pub fn write_bgr(&self, buf: &mut Vec<u8>, i: usize) {
        buf[i * 3]     = to_srgb(self.b);
        buf[i * 3 + 1] = to_srgb(self.g);
        buf[i * 3 + 2] = to_srgb(self.r);
    }

    /// Some indication of significance; if 0, unsignificant; if
    /// greater than 0, significant. Used to disable shading when
    /// unnecessary.
    pub fn significance(&self) -> f64 {
        self.r + self.g + self.b
    }
}
