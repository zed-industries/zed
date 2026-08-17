/*!
`tiny-skia` is a tiny [Skia](https://skia.org/) subset ported to Rust.

`tiny-skia` API is a bit unconventional.
It doesn't look like cairo, QPainter (Qt), HTML Canvas or even Skia itself.
Instead, `tiny-skia` provides a set of low-level drawing APIs
and a user should manage the world transform, clipping mask and style manually.

See the `examples/` directory for usage examples.
*/

#![no_std]
#![warn(missing_docs)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![allow(clippy::approx_constant)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::identity_op)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::wrong_self_convention)]

#[cfg(not(any(feature = "std", feature = "no-std-float")))]
compile_error!("You have to activate either the `std` or the `no-std-float` feature.");

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

mod alpha_runs;
mod blend_mode;
mod blitter;
mod color;
mod edge;
mod edge_builder;
mod edge_clipper;
mod fixed_point;
mod geom;
mod line_clipper;
mod mask;
mod math;
mod path64;
mod path_geometry;
mod pipeline;
mod pixmap;
mod scan;
mod shaders;
mod wide;

mod painter; // Keep it under `pixmap` for a better order in the docs.

pub use blend_mode::BlendMode;
pub use color::{Color, ColorU8, PremultipliedColor, PremultipliedColorU8};
pub use color::{ALPHA_OPAQUE, ALPHA_TRANSPARENT, ALPHA_U8_OPAQUE, ALPHA_U8_TRANSPARENT};
pub use mask::{Mask, MaskType};
pub use painter::{FillRule, Paint};
pub use pixmap::{Pixmap, PixmapMut, PixmapRef, BYTES_PER_PIXEL};
pub use shaders::{FilterQuality, GradientStop, PixmapPaint, SpreadMode};
pub use shaders::{LinearGradient, Pattern, RadialGradient, Shader};

pub use tiny_skia_path::{IntRect, IntSize, NonZeroRect, Point, Rect, Size, Transform};
pub use tiny_skia_path::{LineCap, LineJoin, Stroke, StrokeDash};
pub use tiny_skia_path::{Path, PathBuilder, PathSegment, PathSegmentsIter, PathStroker};

/// An integer length that is guarantee to be > 0
type LengthU32 = core::num::NonZeroU32;
