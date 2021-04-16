use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::op_macros::{forward_ref_binop, forward_ref_op_assign};

pub type Rgb8 = image::Rgb<u8>;

pub trait FromHexStr<T = Self> {
    fn from_hex(hex: &str) -> Result<T, Box<dyn std::error::Error>>;
}

impl FromHexStr for Rgb8 {
    fn from_hex(hex: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if hex.len() > 7 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Hex color string invalid: {}", hex),
            )
            .into());
        }

        let hex = if hex.len() == 7 { &hex[1..] } else { hex };

        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;

        Ok(Rgb8::from([r, g, b]))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RgbF32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Default for RgbF32 {
    fn default() -> RgbF32 {
        RgbF32 {
            r: 0.,
            g: 0.,
            b: 0.,
        }
    }
}

impl FromHexStr for RgbF32 {
    fn from_hex(hex: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if hex.len() > 7 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Hex color string invalid: {}", hex),
            )
            .into());
        }

        let hex = if hex.len() == 7 { &hex[1..] } else { hex };

        let r = u8::from_str_radix(&hex[0..2], 16)? as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16)? as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16)? as f32 / 255.0;

        Ok(RgbF32 { r, g, b })
    }
}

impl RgbF32 {
    pub fn new(r: f32, g: f32, b: f32) -> RgbF32 {
        RgbF32 { r, g, b }
    }

    pub fn new_u8(r: u8, g: u8, b: u8) -> RgbF32 {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        RgbF32 { r, g, b }
    }

    pub fn dist(&self) -> f32 {
        (self.r * self.r + self.g * self.g + self.b * self.b).sqrt()
    }

    pub fn abs(&mut self) {
        self.r = self.r.abs();
        self.g = self.g.abs();
        self.b = self.b.abs();
    }

    pub fn to_abs(self) -> RgbF32 {
        RgbF32 {
            r: self.r.abs(),
            g: self.g.abs(),
            b: self.b.abs(),
        }
    }

    pub fn normalize(&mut self) {
        let d = self.dist();
        if d != 0.0 {
            self.r /= d;
            self.g /= d;
            self.b /= d;
        }
    }

    pub fn to_normal(self) -> RgbF32 {
        let d = self.dist();

        if d != 0.0 {
            let r = self.r / d;
            let g = self.g / d;
            let b = self.b / d;

            RgbF32 { r, g, b }
        } else {
            RgbF32::default()
        }
    }

    pub fn sum_rgb(&self) -> f32 {
        self.r + self.g + self.b
    }

    pub fn to_bytes(self) -> [u8; 3] {
        let r = (self.r.clamp(0., 1.) * 255.0).round();
        let g = (self.g.clamp(0., 1.) * 255.0).round();
        let b = (self.b.clamp(0., 1.) * 255.0).round();

        [r as u8, g as u8, b as u8]
    }

    // like to_bytes but instead of accurate rounding it just cuts the comma
    pub fn to_bytes_fast(self) -> [u8; 3] {
        let r = self.r.clamp(0., 1.) * 255.0;
        let g = self.g.clamp(0., 1.) * 255.0;
        let b = self.b.clamp(0., 1.) * 255.0;

        [r as u8, g as u8, b as u8]
    }

    pub fn to_hex_string(self) -> String {
        let raw = self.to_bytes();
        format!("#{:x}{:x}{:x}", raw[0], raw[1], raw[2])
    }
}

impl From<&Rgb8> for RgbF32 {
    fn from(rbg: &Rgb8) -> RgbF32 {
        RgbF32 {
            r: rbg.0[0] as f32 / 255.0,
            g: rbg.0[1] as f32 / 255.0,
            b: rbg.0[2] as f32 / 255.0,
        }
    }
}

impl From<&mut Rgb8> for RgbF32 {
    fn from(rbg: &mut Rgb8) -> RgbF32 {
        RgbF32 {
            r: rbg.0[0] as f32 / 255.0,
            g: rbg.0[1] as f32 / 255.0,
            b: rbg.0[2] as f32 / 255.0,
        }
    }
}

impl From<RgbF32> for [u8; 3] {
    fn from(rgb: RgbF32) -> [u8; 3] {
        let r = (rgb.r * 255.0).round() as u8;
        let g = (rgb.g * 255.0).round() as u8;
        let b = (rgb.b * 255.0).round() as u8;
        [r, g, b]
    }
}

impl From<RgbF32> for Rgb8 {
    fn from(rgb: RgbF32) -> Rgb8 {
        let r = (rgb.r * 255.0).round() as u8;
        let g = (rgb.g * 255.0).round() as u8;
        let b = (rgb.b * 255.0).round() as u8;
        Rgb8::from([r, g, b])
    }
}

//
// Operators
//

impl Add for RgbF32 {
    type Output = RgbF32;

    fn add(self, rhs: RgbF32) -> Self::Output {
        RgbF32 {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
forward_ref_binop! { impl Add, add for RgbF32, RgbF32}

impl AddAssign for RgbF32 {
    fn add_assign(&mut self, rhs: RgbF32) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
forward_ref_op_assign! { impl AddAssign, add_assign for RgbF32, RgbF32}

impl Sub<RgbF32> for RgbF32 {
    type Output = RgbF32;

    fn sub(self, rhs: RgbF32) -> Self::Output {
        RgbF32 {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}
forward_ref_binop! { impl Sub, sub for RgbF32, RgbF32}

impl SubAssign for RgbF32 {
    fn sub_assign(&mut self, rhs: RgbF32) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}
forward_ref_op_assign! { impl SubAssign, sub_assign for RgbF32, RgbF32}

impl Mul<RgbF32> for RgbF32 {
    type Output = RgbF32;

    fn mul(self, rhs: RgbF32) -> Self::Output {
        RgbF32 {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
forward_ref_binop! { impl Mul, mul for RgbF32, RgbF32}

impl MulAssign for RgbF32 {
    fn mul_assign(&mut self, rhs: RgbF32) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}
forward_ref_op_assign! { impl MulAssign, mul_assign for RgbF32, RgbF32}

impl Div<RgbF32> for RgbF32 {
    type Output = RgbF32;

    fn div(self, rhs: RgbF32) -> Self::Output {
        RgbF32 {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}
forward_ref_binop! { impl Div, div for RgbF32, RgbF32}

impl DivAssign for RgbF32 {
    fn div_assign(&mut self, rhs: RgbF32) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}
forward_ref_op_assign! { impl DivAssign, div_assign for RgbF32, RgbF32}

//
// Ops with f64
//

impl Mul<f32> for RgbF32 {
    type Output = RgbF32;

    fn mul(self, rhs: f32) -> Self::Output {
        RgbF32 {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
forward_ref_binop! { impl Mul, mul for RgbF32, f32}

impl MulAssign<f32> for RgbF32 {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}
forward_ref_op_assign! { impl MulAssign, mul_assign for RgbF32, f32}

impl Div<f32> for RgbF32 {
    type Output = RgbF32;

    fn div(self, rhs: f32) -> Self::Output {
        RgbF32 {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
forward_ref_binop! { impl Div, div for RgbF32, f32}

impl DivAssign<f32> for RgbF32 {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}
forward_ref_op_assign! { impl DivAssign, div_assign for RgbF32, f32}
