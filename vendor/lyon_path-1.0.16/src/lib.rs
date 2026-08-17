#![doc(html_logo_url = "https://nical.github.io/lyon-doc/lyon-logo.svg")]
#![deny(bare_trait_objects)]
#![deny(unconditional_recursion)]
#![allow(clippy::match_like_matches_macro)]
#![allow(mismatched_lifetime_syntaxes)]
#![no_std]

//! Data structures and traits to work with paths (vector graphics).
//!
//! To build and consume paths, see the [builder](builder/index.html) and
//! [iterator](iterator/index.html) modules.
//!
//! This crate is reexported in [lyon](https://docs.rs/lyon/).
//!
//! # Examples
//!
//! ```
//! # extern crate lyon_path;
//! # fn main() {
//! use lyon_path::Path;
//! use lyon_path::math::{point};
//! use lyon_path::builder::*;
//!
//! // Create a builder object to build the path.
//! let mut builder = Path::builder();
//!
//! // Build a simple path.
//! let mut builder = Path::builder();
//! builder.begin(point(0.0, 0.0));
//! builder.line_to(point(1.0, 2.0));
//! builder.line_to(point(2.0, 0.0));
//! builder.line_to(point(1.0, 1.0));
//! builder.close();
//!
//! // Generate the actual path object.
//! let path = builder.build();
//!
//! for event in &path {
//!     println!("{:?}", event);
//! }
//! # }
//! ```
//!

extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

pub use lyon_geom as geom;

#[cfg(feature = "serialization")]
#[macro_use]
pub extern crate serde;

pub mod builder;
pub mod commands;
mod events;
pub mod iterator;
// TODO: remove "pub" on mod path to avoid redundant  "use lyon::path::path::Path" in user code
//       breaking change would require 1.1 bump?
pub mod path;
pub use path::*;
pub mod path_buffer;
pub mod polygon;

#[doc(hidden)]
pub mod private;

#[doc(inline)]
pub use crate::commands::{PathCommands, PathCommandsSlice};
pub use crate::events::*;
pub use crate::geom::ArcFlags;
#[doc(inline)]
pub use crate::path::{Path, PathSlice};
#[doc(inline)]
pub use crate::path_buffer::{PathBuffer, PathBufferSlice};
#[doc(inline)]
pub use crate::polygon::{IdPolygon, Polygon};

use core::fmt;
use math::Point;

pub mod traits {
    //! `lyon_path` traits reexported here for convenience.

    pub use crate::builder::Build;
    pub use crate::builder::PathBuilder;
    pub use crate::builder::SvgPathBuilder;
    pub use crate::iterator::PathIterator;
}

pub mod math {
    //! f32 version of the lyon_geom types used everywhere. Most other lyon crates
    //! reexport them.

    use crate::geom::euclid;

    /// Alias for ```euclid::default::Point2D<f32>```.
    pub type Point = euclid::default::Point2D<f32>;

    /// Alias for ```euclid::default::Point2D<f32>```.
    pub type Vector = euclid::default::Vector2D<f32>;

    /// Alias for ```euclid::default::Size2D<f32>```.
    pub type Size = euclid::default::Size2D<f32>;

    /// Alias for ```euclid::default::Box2D<f32>```
    pub type Box2D = euclid::default::Box2D<f32>;

    /// Alias for ```euclid::default::Transform2D<f32>```
    pub type Transform = euclid::default::Transform2D<f32>;

    /// Alias for ```euclid::default::Rotation2D<f32>```
    pub type Rotation = euclid::default::Rotation2D<f32>;

    /// Alias for ```euclid::default::Translation2D<f32>```
    pub type Translation = euclid::Translation2D<f32, euclid::UnknownUnit, euclid::UnknownUnit>;

    /// Alias for ```euclid::default::Scale<f32>```
    pub type Scale = euclid::default::Scale<f32>;

    /// An angle in radians (f32).
    pub type Angle = euclid::Angle<f32>;

    /// Shorthand for `Vector::new(x, y)`.
    #[inline]
    pub fn vector(x: f32, y: f32) -> Vector {
        Vector::new(x, y)
    }

    /// Shorthand for `Point::new(x, y)`.
    #[inline]
    pub fn point(x: f32, y: f32) -> Point {
        Point::new(x, y)
    }

    /// Shorthand for `Size::new(x, y)`.
    #[inline]
    pub fn size(w: f32, h: f32) -> Size {
        Size::new(w, h)
    }
}

/// Line cap as defined by the SVG specification.
///
/// See: <https://svgwg.org/specs/strokes/#StrokeLinecapProperty>
///
/// <svg viewBox="0 0 400 399.99998" height="400" width="400">
///   <g transform="translate(0,-652.36229)">
///     <path style="opacity:1;fill:#80b3ff;stroke:#000000;stroke-width:1;stroke-linejoin:round;" d="m 240,983 a 30,30 0 0 1 -25,-15 30,30 0 0 1 0,-30.00001 30,30 0 0 1 25.98076,-15 l 0,30 z"/>
///     <path style="fill:#80b3ff;stroke:#000000;stroke-width:1px;stroke-linecap:butt;" d="m 390,782.6 -150,0 0,-60 150,0.5"/>
///     <circle style="opacity:1;fill:#ff7f2a;stroke:#000000;stroke-width:1;stroke-linejoin:round;" r="10" cy="752.89227" cx="240.86813"/>
///     <path style="fill:none;stroke:#000000;stroke-width:1px;stroke-linejoin:round;" d="m 240,722.6 150,60"/>
///     <path style="fill:#80b3ff;stroke:#000000;stroke-width:1px;stroke-linecap:butt;" d="m 390,882 -180,0 0,-60 180,0.4"/>
///     <circle style="opacity:1;fill:#ff7f2a;stroke:#000000;stroke-width:1;stroke-linejoin:round;" cx="239.86813" cy="852.20868" r="10" />
///     <path style="fill:none;stroke:#000000;stroke-width:1px;stroke-linejoin:round;" d="m 210.1,822.3 180,60"/>
///     <path style="fill:#80b3ff;stroke:#000000;stroke-width:1px;stroke-linecap:butt;" d="m 390,983 -150,0 0,-60 150,0.4"/>
///     <circle style="opacity:1;fill:#ff7f2a;stroke:#000000;stroke-width:1;stroke-linejoin:round;" cx="239.86813" cy="953.39734" r="10" />
///     <path style="fill:none;stroke:#000000;stroke-width:1px;stroke-linejoin:round;" d="m 390,983 -150,-60 L 210,953 l 30,30 -21.5,-9.5 L 210,953 218.3,932.5 240,923.4"/>
///     <text y="757.61273" x="183.65314" style="font-style:normal;font-weight:normal;font-size:20px;line-height:125%;font-family:Sans;text-align:end;text-anchor:end;fill:#000000;stroke:none;">
///        <tspan y="757.61273" x="183.65314">LineCap::Butt</tspan>
///        <tspan y="857.61273" x="183.65314">LineCap::Square</tspan>
///        <tspan y="957.61273" x="183.65314">LineCap::Round</tspan>
///      </text>
///   </g>
/// </svg>
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum LineCap {
    /// The stroke for each sub-path does not extend beyond its two endpoints.
    /// A zero length sub-path will therefore not have any stroke.
    Butt,
    /// At the end of each sub-path, the shape representing the stroke will be
    /// extended by a rectangle with the same width as the stroke width and
    /// whose length is half of the stroke width. If a sub-path has zero length,
    /// then the resulting effect is that the stroke for that sub-path consists
    /// solely of a square with side length equal to the stroke width, centered
    /// at the sub-path's point.
    Square,
    /// At each end of each sub-path, the shape representing the stroke will be extended
    /// by a half circle with a radius equal to the stroke width.
    /// If a sub-path has zero length, then the resulting effect is that the stroke for
    /// that sub-path consists solely of a full circle centered at the sub-path's point.
    Round,
}

/// Line join as defined by the SVG specification.
///
/// See: <https://svgwg.org/specs/strokes/#StrokeLinejoinProperty>
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum LineJoin {
    /// A sharp corner is to be used to join path segments.
    Miter,
    /// Same as a miter join, but if the miter limit is exceeded,
    /// the miter is clipped at a miter length equal to the miter limit value
    /// multiplied by the stroke width.
    MiterClip,
    /// A round corner is to be used to join path segments.
    Round,
    /// A beveled corner is to be used to join path segments.
    /// The bevel shape is a triangle that fills the area between the two stroked
    /// segments.
    Bevel,
}

/// The positive or negative side of a vector or segment.
///
/// Given a reference vector `v0`, a vector `v1` is on the positive side
/// if the sign of the cross product `v0 x v1` is positive.
///
/// This type does not use the left/right terminology to avoid confusion with
/// left-handed / right-handed coordinate systems. Right-handed coordinate systems
/// seem to be what a lot of people are most familiar with (especially in 2D), however
/// most vector graphics specifications use y-down left-handed coordinate systems.
/// Unfortunately mirroring the y axis inverts the meaning of "left" and "right", which
/// causes confusion. In practice:
///
/// - In a y-down left-handed coordinate system such as `SVG`'s, `Side::Positive` is the right side.
/// - In a y-up right-handed coordinate system, `Side::Positive` is the left side.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum Side {
    Positive,
    Negative,
}

impl Side {
    #[inline]
    pub fn opposite(self) -> Self {
        match self {
            Side::Positive => Side::Negative,
            Side::Negative => Side::Positive,
        }
    }

    #[inline]
    pub fn is_positive(self) -> bool {
        self == Side::Positive
    }

    #[inline]
    pub fn is_negative(self) -> bool {
        self == Side::Negative
    }

    #[inline]
    pub fn to_f32(self) -> f32 {
        match self {
            Side::Positive => 1.0,
            Side::Negative => -1.0,
        }
    }
}

/// The fill rule defines how to determine what is inside and what is outside of the shape.
///
/// See the SVG specification.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum FillRule {
    EvenOdd,
    NonZero,
}

impl FillRule {
    #[inline]
    pub fn is_in(&self, winding_number: i16) -> bool {
        match *self {
            FillRule::EvenOdd => winding_number % 2 != 0,
            FillRule::NonZero => winding_number != 0,
        }
    }

    #[inline]
    pub fn is_out(&self, winding_number: i16) -> bool {
        !self.is_in(winding_number)
    }
}

/// The two possible orientations for the edges of a shape to be built in.
///
/// Positive winding corresponds to the positive orientation in trigonometry.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum Winding {
    Positive,
    Negative,
}

/// ID of a control point in a path.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct ControlPointId(pub u32);

impl ControlPointId {
    pub const INVALID: Self = ControlPointId(u32::MAX);
    pub fn offset(self) -> usize {
        self.0 as usize
    }
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
    pub fn from_usize(val: usize) -> Self {
        ControlPointId(val as u32)
    }
}

impl fmt::Debug for ControlPointId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

/// ID of an endpoint point in a path.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct EndpointId(pub u32);
impl EndpointId {
    pub const INVALID: Self = EndpointId(u32::MAX);
    pub fn offset(self) -> usize {
        self.0 as usize
    }
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
    pub fn from_usize(val: usize) -> Self {
        EndpointId(val as u32)
    }
}

impl fmt::Debug for EndpointId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

/// Refers to an event in a path.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct EventId(#[doc(hidden)] pub u32);

impl EventId {
    pub const INVALID: Self = EventId(u32::MAX);
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

/// Interface for types types (typically endpoints and control points) that have
/// a 2D position.
pub trait Position {
    fn position(&self) -> Point;
}

impl<U> Position for crate::geom::euclid::Point2D<f32, U> {
    fn position(&self) -> Point {
        self.to_untyped()
    }
}

impl<'l, T: Position> Position for &'l T {
    fn position(&self) -> Point {
        (*self).position()
    }
}

impl Position for (f32, f32) {
    fn position(&self) -> Point {
        Point::new(self.0, self.1)
    }
}

impl Position for [f32; 2] {
    fn position(&self) -> Point {
        Point::new(self[0], self[1])
    }
}

impl<T> Position for (Point, T) {
    fn position(&self) -> Point {
        self.0
    }
}

/// Interface for objects storing endpoints and control points positions.
///
/// This interface can be implemented by path objects themselves or via external
/// data structures.
pub trait PositionStore {
    fn get_endpoint(&self, id: EndpointId) -> Point;
    fn get_control_point(&self, id: ControlPointId) -> Point;
}

impl<'l> PositionStore for (&'l [Point], &'l [Point]) {
    fn get_endpoint(&self, id: EndpointId) -> Point {
        self.0[id.to_usize()]
    }
    fn get_control_point(&self, id: ControlPointId) -> Point {
        self.1[id.to_usize()]
    }
}

/// Interface for objects storing custom attributes associated with endpoints.
///
/// This interface can be implemented by path objects themselves or via external
/// data structures.
pub trait AttributeStore {
    /// Returns the endpoint's custom attributes as a slice of 32 bits floats.
    ///
    /// The size of the slice must be equal to the result of `num_attributes()`.
    fn get(&self, id: EndpointId) -> Attributes;

    /// Returns the number of float attributes per endpoint.
    ///
    /// All endpoints must have the same number of attributes.
    fn num_attributes(&self) -> usize;
}

impl AttributeStore for () {
    fn get(&self, _: EndpointId) -> Attributes {
        NO_ATTRIBUTES
    }

    fn num_attributes(&self) -> usize {
        0
    }
}

/// A view over a contiguous storage of custom attributes.
pub struct AttributeSlice<'l> {
    data: &'l [f32],
    stride: usize,
}

impl<'l> AttributeSlice<'l> {
    pub fn new(data: &'l [f32], num_attributes: usize) -> Self {
        AttributeSlice {
            data,
            stride: num_attributes,
        }
    }
}

impl<'l> AttributeStore for AttributeSlice<'l> {
    fn get(&self, id: EndpointId) -> Attributes {
        let start = id.to_usize() * self.stride;
        let end = start + self.stride;
        &self.data[start..end]
    }

    fn num_attributes(&self) -> usize {
        self.stride
    }
}

/// An alias for `usize`.
pub type AttributeIndex = usize;
/// An alias for a slice of `f32` values.
pub type Attributes<'l> = &'l [f32];
/// An empty attribute slice.
pub const NO_ATTRIBUTES: Attributes<'static> = &[];
