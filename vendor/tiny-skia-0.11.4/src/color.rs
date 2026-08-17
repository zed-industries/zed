// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use tiny_skia_path::{NormalizedF32, Scalar};

/// 8-bit type for an alpha value. 255 is 100% opaque, zero is 100% transparent.
pub type AlphaU8 = u8;

/// Represents fully transparent AlphaU8 value.
pub const ALPHA_U8_TRANSPARENT: AlphaU8 = 0x00;

/// Represents fully opaque AlphaU8 value.
pub const ALPHA_U8_OPAQUE: AlphaU8 = 0xFF;

/// Represents fully transparent Alpha value.
pub const ALPHA_TRANSPARENT: NormalizedF32 = NormalizedF32::ZERO;

/// Represents fully opaque Alpha value.
pub const ALPHA_OPAQUE: NormalizedF32 = NormalizedF32::ONE;

/// A 32-bit RGBA color value.
///
/// Byteorder: RGBA (relevant for bytemuck casts)
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub struct ColorU8([u8; 4]);

impl ColorU8 {
    /// Creates a new color.
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        ColorU8([r, g, b, a])
    }

    /// Returns color's red component.
    pub const fn red(self) -> u8 {
        self.0[0]
    }

    /// Returns color's green component.
    pub const fn green(self) -> u8 {
        self.0[1]
    }

    /// Returns color's blue component.
    pub const fn blue(self) -> u8 {
        self.0[2]
    }

    /// Returns color's alpha component.
    pub const fn alpha(self) -> u8 {
        self.0[3]
    }

    /// Check that color is opaque.
    ///
    /// Alpha == 255
    pub fn is_opaque(&self) -> bool {
        self.alpha() == ALPHA_U8_OPAQUE
    }

    /// Converts into a premultiplied color.
    pub fn premultiply(&self) -> PremultipliedColorU8 {
        let a = self.alpha();
        if a != ALPHA_U8_OPAQUE {
            PremultipliedColorU8::from_rgba_unchecked(
                premultiply_u8(self.red(), a),
                premultiply_u8(self.green(), a),
                premultiply_u8(self.blue(), a),
                a,
            )
        } else {
            PremultipliedColorU8::from_rgba_unchecked(self.red(), self.green(), self.blue(), a)
        }
    }
}

impl core::fmt::Debug for ColorU8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ColorU8")
            .field("r", &self.red())
            .field("g", &self.green())
            .field("b", &self.blue())
            .field("a", &self.alpha())
            .finish()
    }
}

/// A 32-bit premultiplied RGBA color value.
///
/// Byteorder: RGBA (relevant for bytemuck casts)
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub struct PremultipliedColorU8([u8; 4]);

// Perfectly safe, since [u8; 4] is already Pod.
unsafe impl bytemuck::Zeroable for PremultipliedColorU8 {}
unsafe impl bytemuck::Pod for PremultipliedColorU8 {}

impl PremultipliedColorU8 {
    /// A transparent color.
    pub const TRANSPARENT: Self = PremultipliedColorU8::from_rgba_unchecked(0, 0, 0, 0);

    /// Creates a new premultiplied color.
    ///
    /// RGB components must be <= alpha.
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Option<Self> {
        if r <= a && g <= a && b <= a {
            Some(PremultipliedColorU8([r, g, b, a]))
        } else {
            None
        }
    }

    /// Creates a new color.
    pub(crate) const fn from_rgba_unchecked(r: u8, g: u8, b: u8, a: u8) -> Self {
        PremultipliedColorU8([r, g, b, a])
    }

    /// Returns color's red component.
    ///
    /// The value is <= alpha.
    pub const fn red(self) -> u8 {
        self.0[0]
    }

    /// Returns color's green component.
    ///
    /// The value is <= alpha.
    pub const fn green(self) -> u8 {
        self.0[1]
    }

    /// Returns color's blue component.
    ///
    /// The value is <= alpha.
    pub const fn blue(self) -> u8 {
        self.0[2]
    }

    /// Returns color's alpha component.
    pub const fn alpha(self) -> u8 {
        self.0[3]
    }

    /// Check that color is opaque.
    ///
    /// Alpha == 255
    pub fn is_opaque(&self) -> bool {
        self.alpha() == ALPHA_U8_OPAQUE
    }

    /// Returns a demultiplied color.
    pub fn demultiply(&self) -> ColorU8 {
        let alpha = self.alpha();
        if alpha == ALPHA_U8_OPAQUE {
            ColorU8(self.0)
        } else {
            let a = alpha as f64 / 255.0;
            ColorU8::from_rgba(
                (self.red() as f64 / a + 0.5) as u8,
                (self.green() as f64 / a + 0.5) as u8,
                (self.blue() as f64 / a + 0.5) as u8,
                alpha,
            )
        }
    }
}

impl core::fmt::Debug for PremultipliedColorU8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PremultipliedColorU8")
            .field("r", &self.red())
            .field("g", &self.green())
            .field("b", &self.blue())
            .field("a", &self.alpha())
            .finish()
    }
}

/// An RGBA color value, holding four floating point components.
///
/// # Guarantees
///
/// - All values are in 0..=1 range.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    r: NormalizedF32,
    g: NormalizedF32,
    b: NormalizedF32,
    a: NormalizedF32,
}

const NV_ZERO: NormalizedF32 = NormalizedF32::ZERO;
const NV_ONE: NormalizedF32 = NormalizedF32::ONE;

impl Color {
    /// A transparent color.
    pub const TRANSPARENT: Color = Color {
        r: NV_ZERO,
        g: NV_ZERO,
        b: NV_ZERO,
        a: NV_ZERO,
    };
    /// A black color.
    pub const BLACK: Color = Color {
        r: NV_ZERO,
        g: NV_ZERO,
        b: NV_ZERO,
        a: NV_ONE,
    };
    /// A white color.
    pub const WHITE: Color = Color {
        r: NV_ONE,
        g: NV_ONE,
        b: NV_ONE,
        a: NV_ONE,
    };

    /// Creates a new color from 4 components.
    ///
    /// All values must be in 0..=1 range.
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Option<Self> {
        Some(Color {
            r: NormalizedF32::new(r)?,
            g: NormalizedF32::new(g)?,
            b: NormalizedF32::new(b)?,
            a: NormalizedF32::new(a)?,
        })
    }

    /// Creates a new color from 4 components.
    ///
    /// u8 will be divided by 255 to get the float component.
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: NormalizedF32::new_u8(r),
            g: NormalizedF32::new_u8(g),
            b: NormalizedF32::new_u8(b),
            a: NormalizedF32::new_u8(a),
        }
    }

    /// Returns color's red component.
    ///
    /// The value is guarantee to be in a 0..=1 range.
    pub fn red(&self) -> f32 {
        self.r.get()
    }

    /// Returns color's green component.
    ///
    /// The value is guarantee to be in a 0..=1 range.
    pub fn green(&self) -> f32 {
        self.g.get()
    }

    /// Returns color's blue component.
    ///
    /// The value is guarantee to be in a 0..=1 range.
    pub fn blue(&self) -> f32 {
        self.b.get()
    }

    /// Returns color's alpha component.
    ///
    /// The value is guarantee to be in a 0..=1 range.
    pub fn alpha(&self) -> f32 {
        self.a.get()
    }

    /// Sets the red component value.
    ///
    /// The new value will be clipped to the 0..=1 range.
    pub fn set_red(&mut self, c: f32) {
        self.r = NormalizedF32::new_clamped(c);
    }

    /// Sets the green component value.
    ///
    /// The new value will be clipped to the 0..=1 range.
    pub fn set_green(&mut self, c: f32) {
        self.g = NormalizedF32::new_clamped(c);
    }

    /// Sets the blue component value.
    ///
    /// The new value will be clipped to the 0..=1 range.
    pub fn set_blue(&mut self, c: f32) {
        self.b = NormalizedF32::new_clamped(c);
    }

    /// Sets the alpha component value.
    ///
    /// The new value will be clipped to the 0..=1 range.
    pub fn set_alpha(&mut self, c: f32) {
        self.a = NormalizedF32::new_clamped(c);
    }

    /// Shifts color's opacity.
    ///
    /// Essentially, multiplies color's alpha by opacity.
    ///
    /// `opacity` will be clamped to the 0..=1 range first.
    /// The final alpha will also be clamped.
    pub fn apply_opacity(&mut self, opacity: f32) {
        self.a = NormalizedF32::new_clamped(self.a.get() * opacity.bound(0.0, 1.0));
    }

    /// Check that color is opaque.
    ///
    /// Alpha == 1.0
    pub fn is_opaque(&self) -> bool {
        self.a == ALPHA_OPAQUE
    }

    /// Converts into a premultiplied color.
    pub fn premultiply(&self) -> PremultipliedColor {
        if self.is_opaque() {
            PremultipliedColor {
                r: self.r,
                g: self.g,
                b: self.b,
                a: self.a,
            }
        } else {
            PremultipliedColor {
                r: NormalizedF32::new_clamped(self.r.get() * self.a.get()),
                g: NormalizedF32::new_clamped(self.g.get() * self.a.get()),
                b: NormalizedF32::new_clamped(self.b.get() * self.a.get()),
                a: self.a,
            }
        }
    }

    /// Converts into `ColorU8`.
    pub fn to_color_u8(&self) -> ColorU8 {
        let c = color_f32_to_u8(self.r, self.g, self.b, self.a);
        ColorU8::from_rgba(c[0], c[1], c[2], c[3])
    }
}

/// A premultiplied RGBA color value, holding four floating point components.
///
/// # Guarantees
///
/// - All values are in 0..=1 range.
/// - RGB components are <= A.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PremultipliedColor {
    r: NormalizedF32,
    g: NormalizedF32,
    b: NormalizedF32,
    a: NormalizedF32,
}

impl PremultipliedColor {
    /// Returns color's red component.
    ///
    /// - The value is guarantee to be in a 0..=1 range.
    /// - The value is <= alpha.
    pub fn red(&self) -> f32 {
        self.r.get()
    }

    /// Returns color's green component.
    ///
    /// - The value is guarantee to be in a 0..=1 range.
    /// - The value is <= alpha.
    pub fn green(&self) -> f32 {
        self.g.get()
    }

    /// Returns color's blue component.
    ///
    /// - The value is guarantee to be in a 0..=1 range.
    /// - The value is <= alpha.
    pub fn blue(&self) -> f32 {
        self.b.get()
    }

    /// Returns color's alpha component.
    ///
    /// - The value is guarantee to be in a 0..=1 range.
    pub fn alpha(&self) -> f32 {
        self.a.get()
    }

    /// Returns a demultiplied color.
    pub fn demultiply(&self) -> Color {
        let a = self.a.get();
        if a == 0.0 {
            Color::TRANSPARENT
        } else {
            Color {
                r: NormalizedF32::new_clamped(self.r.get() / a),
                g: NormalizedF32::new_clamped(self.g.get() / a),
                b: NormalizedF32::new_clamped(self.b.get() / a),
                a: self.a,
            }
        }
    }

    /// Converts into `PremultipliedColorU8`.
    pub fn to_color_u8(&self) -> PremultipliedColorU8 {
        let c = color_f32_to_u8(self.r, self.g, self.b, self.a);
        PremultipliedColorU8::from_rgba_unchecked(c[0], c[1], c[2], c[3])
    }
}

/// Return a*b/255, rounding any fractional bits.
pub fn premultiply_u8(c: u8, a: u8) -> u8 {
    let prod = u32::from(c) * u32::from(a) + 128;
    ((prod + (prod >> 8)) >> 8) as u8
}

fn color_f32_to_u8(
    r: NormalizedF32,
    g: NormalizedF32,
    b: NormalizedF32,
    a: NormalizedF32,
) -> [u8; 4] {
    [
        (r.get() * 255.0 + 0.5) as u8,
        (g.get() * 255.0 + 0.5) as u8,
        (b.get() * 255.0 + 0.5) as u8,
        (a.get() * 255.0 + 0.5) as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn premultiply_u8() {
        assert_eq!(
            ColorU8::from_rgba(10, 20, 30, 40).premultiply(),
            PremultipliedColorU8::from_rgba_unchecked(2, 3, 5, 40)
        );
    }

    #[test]
    fn premultiply_u8_opaque() {
        assert_eq!(
            ColorU8::from_rgba(10, 20, 30, 255).premultiply(),
            PremultipliedColorU8::from_rgba_unchecked(10, 20, 30, 255)
        );
    }

    #[test]
    fn demultiply_u8_1() {
        assert_eq!(
            PremultipliedColorU8::from_rgba_unchecked(2, 3, 5, 40).demultiply(),
            ColorU8::from_rgba(13, 19, 32, 40)
        );
    }

    #[test]
    fn demultiply_u8_2() {
        assert_eq!(
            PremultipliedColorU8::from_rgba_unchecked(10, 20, 30, 255).demultiply(),
            ColorU8::from_rgba(10, 20, 30, 255)
        );
    }

    #[test]
    fn demultiply_u8_3() {
        assert_eq!(
            PremultipliedColorU8::from_rgba_unchecked(153, 99, 54, 180).demultiply(),
            ColorU8::from_rgba(217, 140, 77, 180)
        );
    }

    #[test]
    fn bytemuck_casts_rgba() {
        let slice = &[
            PremultipliedColorU8::from_rgba_unchecked(0, 1, 2, 3),
            PremultipliedColorU8::from_rgba_unchecked(10, 11, 12, 13),
        ];
        let bytes: &[u8] = bytemuck::cast_slice(slice);
        assert_eq!(bytes, &[0, 1, 2, 3, 10, 11, 12, 13]);
    }
}
