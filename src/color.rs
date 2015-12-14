use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

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

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Color {
        Color {r: self.r * other, g: self.g * other, b: self.b * other}
    }
}

impl Div<f32> for Color {
    type Output = Color;
    fn div(self, other: f32) -> Color {
        Color {r: self.r / other, g: self.g / other, b: self.b / other}
    }
}

fn clamp_color_val(val: f32) -> u8 {
    let x = val * 256.0;
    if x < 0.0 {0} else if x >= 255.0 {255} else {x.trunc() as u8}
}

impl Color {
    pub fn fromRGB(r: f32, g: f32, b: f32) -> Color {
        Color {r: r, g: g, b: b}
    }

    pub fn RGB(&self) -> [u8; 3] {
        [clamp_color_val(self.r), clamp_color_val(self.g), clamp_color_val(self.b)]
    }
}
