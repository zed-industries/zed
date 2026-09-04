use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    Background, Bounds, ContentMask, Corners, DevicePixels, Hsla, ScaledPixels, Size, point, size,
};

/// How a layer mixes with what is under it. The CSS `mix-blend-mode` values
/// in spec order, plus `plus-lighter`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum BlendMode {
    #[default]
    Normal = 0,
    Multiply = 1,
    Screen = 2,
    Overlay = 3,
    Darken = 4,
    Lighten = 5,
    ColorDodge = 6,
    ColorBurn = 7,
    HardLight = 8,
    SoftLight = 9,
    Difference = 10,
    Exclusion = 11,
    Hue = 12,
    Saturation = 13,
    Color = 14,
    Luminosity = 15,
    PlusLighter = 16,
}

/// A 4 by 5 colour matrix in row-major order. Each output channel is a
/// weighted sum of the input r, g, b, a and a constant. Every CSS `filter`
/// function except `blur()` and `drop-shadow()` is one of these, and a list
/// of them multiplies into one.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ColorMatrix(pub [f32; 20]);

impl ColorMatrix {
    /// The matrix that changes nothing.
    pub const IDENTITY: Self = Self([
        1., 0., 0., 0., 0., //
        0., 1., 0., 0., 0., //
        0., 0., 1., 0., 0., //
        0., 0., 0., 1., 0.,
    ]);

    /// Whether the matrix changes nothing.
    pub fn is_identity(&self) -> bool {
        *self == Self::IDENTITY
    }

    /// The matrix that applies `self` first and then `next`.
    pub fn then(&self, next: &Self) -> Self {
        let a = &next.0;
        let b = &self.0;
        let mut out = [0.0; 20];
        for row in 0..4 {
            for col in 0..5 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += a[row * 5 + k] * b[k * 5 + col];
                }
                if col == 4 {
                    sum += a[row * 5 + 4];
                }
                out[row * 5 + col] = sum;
            }
        }
        Self(out)
    }
}

impl Default for ColorMatrix {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// An effect layer as the renderer sees it. The renderer draws the content
/// between the begin and end marks into a texture of its own, then paints
/// that texture over the frame with these effects, in device pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct EffectLayer {
    /// The box of the element, clipped to its content mask.
    pub bounds: Bounds<ScaledPixels>,
    /// Clips the result the way it clips the content.
    pub content_mask: ContentMask<ScaledPixels>,
    /// The corners of the box, which clip the backdrop effect.
    pub corner_radii: Corners<ScaledPixels>,
    /// The curvature of each corner. See [`crate::CornerShape`].
    pub corner_shapes: Corners<f32>,
    /// Gaussian sigma of the blur on the content. 0 is none.
    pub blur: f32,
    /// Gaussian sigma of the blur on what is under the box. 0 is none.
    pub backdrop_blur: f32,
    /// Multiplies the alpha of the content.
    pub opacity: f32,
    /// A `BlendMode` as a number.
    pub blend_mode: u32,
    /// 1 when `mask` is live.
    pub has_mask: u32,
    /// 1 when the layer changes what is under it.
    pub has_backdrop: u32,
    /// 1 when the box clips its content to its rounded corners.
    pub clips_content: u32,
    /// 1 when the layer paints a shadow of its content.
    pub has_shadow: u32,
    /// Gaussian sigma of the shadow. 0 is a sharp copy.
    pub shadow_blur: f32,
    /// Where the shadow sits against the content, in device pixels. Two
    /// scalars, because a `float2` here would need 8 byte alignment in
    /// Metal and the Rust struct has none.
    pub shadow_offset_x: f32,
    /// See `shadow_offset_x`.
    pub shadow_offset_y: f32,
    /// The colour of the shadow.
    pub shadow_color: Hsla,
    /// Applied to the content after the blur.
    pub color_matrix: [f32; 20],
    /// Applied to the backdrop after its blur.
    pub backdrop_matrix: [f32; 20],
    /// A fill over the box whose alpha keeps or drops each pixel.
    pub mask: Background,
    /// WGSL rounds a struct that holds a `vec2<f32>` up to 8 bytes. This pad
    /// keeps the Rust size at that multiple, so the field after a layer in a
    /// storage buffer lands at the same offset on both sides.
    pub pad: u32,
}

impl EffectLayer {
    /// The pixels of the frame the layer touches: its box grown by the reach
    /// of its blurs, inside its content mask and inside `parent`.
    pub fn region(&self, parent: LayerRegion) -> LayerRegion {
        let shadow = if self.has_shadow != 0 {
            3.0 * self.shadow_blur + self.shadow_offset_x.abs().max(self.shadow_offset_y.abs())
        } else {
            0.0
        };
        let pad = (3.0 * self.blur.max(self.backdrop_blur)).max(shadow).ceil() as i32;
        LayerRegion::around(self.bounds, pad)
            .intersect(LayerRegion::around(self.content_mask.bounds, 0))
            .intersect(parent)
    }
}

/// A rectangle of the frame in whole device pixels, for the renderers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct LayerRegion {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl LayerRegion {
    /// Half the size, at the origin. The next step of a shrink chain.
    pub fn halved(self) -> Self {
        Self::new(0, 0, (self.width / 2).max(1), (self.height / 2).max(1))
    }

    /// A region that is never smaller than nothing.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width: width.max(0),
            height: height.max(0),
        }
    }

    /// The whole frame.
    pub fn of_viewport(viewport_size: Size<DevicePixels>) -> Self {
        Self::new(0, 0, viewport_size.width.0, viewport_size.height.0)
    }

    /// The pixels `bounds` touches, grown by `pad` on every side.
    pub fn around(bounds: Bounds<ScaledPixels>, pad: i32) -> Self {
        let left = bounds.origin.x.0.floor() as i32 - pad;
        let top = bounds.origin.y.0.floor() as i32 - pad;
        let right = (bounds.origin.x.0 + bounds.size.width.0).ceil() as i32 + pad;
        let bottom = (bounds.origin.y.0 + bounds.size.height.0).ceil() as i32 + pad;
        Self::new(left, top, right - left, bottom - top)
    }

    /// The pixels in both regions.
    pub fn intersect(self, other: Self) -> Self {
        let left = self.x.max(other.x);
        let top = self.y.max(other.y);
        let right = (self.x + self.width).min(other.x + other.width);
        let bottom = (self.y + self.height).min(other.y + other.height);
        Self::new(left, top, right - left, bottom - top)
    }

    /// Whether the region has no pixels.
    pub fn is_empty(self) -> bool {
        self.width <= 0 || self.height <= 0
    }

    /// The region as bounds in device pixels.
    pub fn bounds(self) -> Bounds<ScaledPixels> {
        Bounds {
            origin: point(ScaledPixels(self.x as f32), ScaledPixels(self.y as f32)),
            size: size(
                ScaledPixels(self.width as f32),
                ScaledPixels(self.height as f32),
            ),
        }
    }
}

/// How a renderer blurs a region: two passes of a Gaussian, one per axis,
/// on a texture shrunk by `scale`, so a wide blur costs the same as a
/// narrow one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlurPlan {
    /// How many source pixels one pixel of the small texture stands for.
    pub scale: i32,
    /// The standard deviation in pixels of the small texture.
    pub sigma: f32,
    /// Taps on each side of the centre.
    pub radius: i32,
}

impl BlurPlan {
    /// The plan for a Gaussian of `sigma` device pixels.
    pub fn new(sigma: f32) -> Self {
        // A power of two, so the renderer shrinks the source by exact 2 by 2
        // box steps. A small grid that samples a big source skips pixels,
        // and the result flickers as content scrolls.
        let scale = if sigma >= 32.0 {
            8
        } else if sigma >= 16.0 {
            4
        } else if sigma >= 8.0 {
            2
        } else {
            1
        };
        let sigma = sigma / scale as f32;
        let radius = (3.0 * sigma).ceil().min(24.0) as i32;
        Self {
            scale,
            sigma,
            radius,
        }
    }

    /// How many times the renderer halves the source before the blur.
    pub fn shrink_steps(&self) -> u32 {
        self.scale.trailing_zeros()
    }
}
