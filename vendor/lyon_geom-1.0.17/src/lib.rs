#![doc(html_logo_url = "https://nical.github.io/lyon-doc/lyon-logo.svg")]
#![deny(bare_trait_objects)]
#![deny(unconditional_recursion)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::let_and_return)]
#![allow(clippy::many_single_char_names)]
#![no_std]

//! Simple 2D geometric primitives on top of euclid.
//!
//! This crate is reexported in [lyon](https://docs.rs/lyon/).
//!
//! # Overview.
//!
//! This crate implements some of the maths to work with:
//!
//! - lines and line segments,
//! - quadratic and cubic bézier curves,
//! - elliptic arcs,
//! - triangles.
//!
//! # Flattening
//!
//! Flattening is the action of approximating a curve with a succession of line segments.
//!
//! <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 30" height="30mm" width="120mm">
//!   <path d="M26.7 24.94l.82-11.15M44.46 5.1L33.8 7.34" fill="none" stroke="#55d400" stroke-width=".5"/>
//!   <path d="M26.7 24.94c.97-11.13 7.17-17.6 17.76-19.84M75.27 24.94l1.13-5.5 2.67-5.48 4-4.42L88 6.7l5.02-1.6" fill="none" stroke="#000"/>
//!   <path d="M77.57 19.37a1.1 1.1 0 0 1-1.08 1.08 1.1 1.1 0 0 1-1.1-1.08 1.1 1.1 0 0 1 1.08-1.1 1.1 1.1 0 0 1 1.1 1.1" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
//!   <path d="M77.57 19.37a1.1 1.1 0 0 1-1.08 1.08 1.1 1.1 0 0 1-1.1-1.08 1.1 1.1 0 0 1 1.08-1.1 1.1 1.1 0 0 1 1.1 1.1" color="#000" fill="#fff"/>
//!   <path d="M80.22 13.93a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.08 1.1 1.1 0 0 1 1.08 1.08" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
//!   <path d="M80.22 13.93a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.08 1.1 1.1 0 0 1 1.08 1.08" color="#000" fill="#fff"/>
//!   <path d="M84.08 9.55a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.1-1.1 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
//!   <path d="M84.08 9.55a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.1-1.1 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="#fff"/>
//!   <path d="M89.1 6.66a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.08-1.08 1.1 1.1 0 0 1 1.1 1.08" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
//!   <path d="M89.1 6.66a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.08-1.08 1.1 1.1 0 0 1 1.1 1.08" color="#000" fill="#fff"/>
//!   <path d="M94.4 5a1.1 1.1 0 0 1-1.1 1.1A1.1 1.1 0 0 1 92.23 5a1.1 1.1 0 0 1 1.08-1.08A1.1 1.1 0 0 1 94.4 5" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
//!   <path d="M94.4 5a1.1 1.1 0 0 1-1.1 1.1A1.1 1.1 0 0 1 92.23 5a1.1 1.1 0 0 1 1.08-1.08A1.1 1.1 0 0 1 94.4 5" color="#000" fill="#fff"/>
//!   <path d="M76.44 25.13a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
//!   <path d="M76.44 25.13a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="#fff"/>
//!   <path d="M27.78 24.9a1.1 1.1 0 0 1-1.08 1.08 1.1 1.1 0 0 1-1.1-1.08 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
//!   <path d="M27.78 24.9a1.1 1.1 0 0 1-1.08 1.08 1.1 1.1 0 0 1-1.1-1.08 1.1 1.1 0 0 1 1.1-1.1 1.1 1.1 0 0 1 1.08 1.1" color="#000" fill="#fff"/>
//!   <path d="M45.4 5.14a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.1-1.1 1.1 1.1 0 0 1 1.1-1.08 1.1 1.1 0 0 1 1.1 1.08" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
//!   <path d="M45.4 5.14a1.1 1.1 0 0 1-1.08 1.1 1.1 1.1 0 0 1-1.1-1.1 1.1 1.1 0 0 1 1.1-1.08 1.1 1.1 0 0 1 1.1 1.08" color="#000" fill="#fff"/>
//!   <path d="M28.67 13.8a1.1 1.1 0 0 1-1.1 1.08 1.1 1.1 0 0 1-1.08-1.08 1.1 1.1 0 0 1 1.08-1.1 1.1 1.1 0 0 1 1.1 1.1" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
//!   <path d="M28.67 13.8a1.1 1.1 0 0 1-1.1 1.08 1.1 1.1 0 0 1-1.08-1.08 1.1 1.1 0 0 1 1.08-1.1 1.1 1.1 0 0 1 1.1 1.1" color="#000" fill="#fff"/>
//!   <path d="M35 7.32a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.1A1.1 1.1 0 0 1 35 7.33" color="#000" fill="none" stroke="#030303" stroke-linecap="round" stroke-opacity=".5"/>
//!   <path d="M35 7.32a1.1 1.1 0 0 1-1.1 1.1 1.1 1.1 0 0 1-1.08-1.1 1.1 1.1 0 0 1 1.1-1.1A1.1 1.1 0 0 1 35 7.33" color="#000" fill="#fff"/>
//!   <text style="line-height:6.61458302px" x="35.74" y="284.49" font-size="5.29" font-family="Sans" letter-spacing="0" word-spacing="0" fill="#b3b3b3" stroke-width=".26" transform="translate(19.595 -267)">
//!     <tspan x="35.74" y="284.49" font-size="10.58">→</tspan>
//!   </text>
//! </svg>
//!
//! The tolerance threshold taken as input by the flattening algorithms corresponds
//! to the maximum distance between the curve and its linear approximation.
//! The smaller the tolerance is, the more precise the approximation and the more segments
//! are generated. This value is typically chosen in function of the zoom level.
//!
//! <svg viewBox="0 0 47.5 13.2" height="100" width="350" xmlns="http://www.w3.org/2000/svg">
//!   <path d="M-2.44 9.53c16.27-8.5 39.68-7.93 52.13 1.9" fill="none" stroke="#dde9af" stroke-width="4.6"/>
//!   <path d="M-1.97 9.3C14.28 1.03 37.36 1.7 49.7 11.4" fill="none" stroke="#00d400" stroke-width=".57" stroke-linecap="round" stroke-dasharray="4.6, 2.291434"/>
//!   <path d="M-1.94 10.46L6.2 6.08l28.32-1.4 15.17 6.74" fill="none" stroke="#000" stroke-width=".6"/>
//!   <path d="M6.83 6.57a.9.9 0 0 1-1.25.15.9.9 0 0 1-.15-1.25.9.9 0 0 1 1.25-.15.9.9 0 0 1 .15 1.25" color="#000" stroke="#000" stroke-width=".57" stroke-linecap="round" stroke-opacity=".5"/>
//!   <path d="M35.35 5.3a.9.9 0 0 1-1.25.15.9.9 0 0 1-.15-1.25.9.9 0 0 1 1.25-.15.9.9 0 0 1 .15 1.24" color="#000" stroke="#000" stroke-width=".6" stroke-opacity=".5"/>
//!   <g fill="none" stroke="#ff7f2a" stroke-width=".26">
//!     <path d="M20.4 3.8l.1 1.83M19.9 4.28l.48-.56.57.52M21.02 5.18l-.5.56-.6-.53" stroke-width=".2978872"/>
//!   </g>
//! </svg>
//!
//! The figure above shows a close up on a curve (the dotted line) and its linear
//! approximation (the black segments). The tolerance threshold is represented by
//! the light green area and the orange arrow.
//!

//#![allow(needless_return)] // clippy

#[cfg(any(test, feature = "std"))]
extern crate std;

// Reexport dependencies.
pub use arrayvec;
pub use euclid;

#[cfg(feature = "serialization")]
#[macro_use]
pub extern crate serde;

#[macro_use]
mod segment;
pub mod arc;
pub mod cubic_bezier;
mod cubic_bezier_intersections;
mod line;
pub mod quadratic_bezier;
mod triangle;
pub mod utils;

#[doc(inline)]
pub use crate::arc::{Arc, ArcFlags, SvgArc};
#[doc(inline)]
pub use crate::cubic_bezier::CubicBezierSegment;
#[doc(inline)]
pub use crate::line::{Line, LineEquation, LineSegment};
#[doc(inline)]
pub use crate::quadratic_bezier::QuadraticBezierSegment;
#[doc(inline)]
pub use crate::segment::Segment;
#[doc(inline)]
pub use crate::triangle::Triangle;

pub use crate::scalar::Scalar;

mod scalar {
    pub(crate) use euclid::Trig;
    pub(crate) use num_traits::cast::cast;
    pub(crate) use num_traits::{Float, FloatConst, NumCast};

    use core::fmt::{Debug, Display};
    use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

    pub trait Scalar:
        Float
        + NumCast
        + FloatConst
        + Sized
        + Display
        + Debug
        + Trig
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
    {
        const HALF: Self;
        const ZERO: Self;
        const ONE: Self;
        const TWO: Self;
        const THREE: Self;
        const FOUR: Self;
        const FIVE: Self;
        const SIX: Self;
        const SEVEN: Self;
        const EIGHT: Self;
        const NINE: Self;
        const TEN: Self;

        const MIN: Self;
        const MAX: Self;

        const EPSILON: Self;
        const DIV_EPSILON: Self = Self::EPSILON;

        /// Epsilon constants are usually not a good way to deal with float precision.
        /// Float precision depends on the magnitude of the values and so should appropriate
        /// epsilons.
        fn epsilon_for(_reference: Self) -> Self {
            Self::EPSILON
        }

        fn value(v: f32) -> Self;
    }

    impl Scalar for f32 {
        const HALF: Self = 0.5;
        const ZERO: Self = 0.0;
        const ONE: Self = 1.0;
        const TWO: Self = 2.0;
        const THREE: Self = 3.0;
        const FOUR: Self = 4.0;
        const FIVE: Self = 5.0;
        const SIX: Self = 6.0;
        const SEVEN: Self = 7.0;
        const EIGHT: Self = 8.0;
        const NINE: Self = 9.0;
        const TEN: Self = 10.0;

        const MIN: Self = f32::MIN;
        const MAX: Self = f32::MAX;

        const EPSILON: Self = 1e-4;

        fn epsilon_for(reference: Self) -> Self {
            // The thresholds are chosen by looking at the table at
            // https://blog.demofox.org/2017/11/21/floating-point-precision/ plus a bit
            // of trial and error. They might change in the future.
            // TODO: instead of casting to an integer, could look at the exponent directly.
            let magnitude = reference.abs() as i32;
            match magnitude {
                0..=7 => 1e-5,
                8..=1023 => 1e-3,
                1024..=4095 => 1e-2,
                5096..=65535 => 1e-1,
                65536..=8_388_607 => 0.5,
                _ => 1.0,
            }
        }

        #[inline]
        fn value(v: f32) -> Self {
            v
        }
    }

    // Epsilon constants are usually not a good way to deal with float precision.
    // Float precision depends on the magnitude of the values and so should appropriate
    // epsilons. This function addresses this somewhat empirically.
    impl Scalar for f64 {
        const HALF: Self = 0.5;
        const ZERO: Self = 0.0;
        const ONE: Self = 1.0;
        const TWO: Self = 2.0;
        const THREE: Self = 3.0;
        const FOUR: Self = 4.0;
        const FIVE: Self = 5.0;
        const SIX: Self = 6.0;
        const SEVEN: Self = 7.0;
        const EIGHT: Self = 8.0;
        const NINE: Self = 9.0;
        const TEN: Self = 10.0;

        const MIN: Self = f64::MIN;
        const MAX: Self = f64::MAX;

        const EPSILON: Self = 1e-8;

        fn epsilon_for(reference: Self) -> Self {
            let magnitude = reference.abs() as i64;
            match magnitude {
                0..=65_535 => 1e-8,
                65_536..=8_388_607 => 1e-5,
                8_388_608..=4_294_967_295 => 1e-3,
                _ => 1e-1,
            }
        }

        #[inline]
        fn value(v: f32) -> Self {
            v as f64
        }
    }
}

/// Alias for `euclid::default::Point2D`.
pub use euclid::default::Point2D as Point;

/// Alias for `euclid::default::Vector2D`.
pub use euclid::default::Vector2D as Vector;

/// Alias for `euclid::default::Size2D`.
pub use euclid::default::Size2D as Size;

/// Alias for `euclid::default::Box2D`
pub use euclid::default::Box2D;

/// Alias for `euclid::default::Transform2D`
pub type Transform<S> = euclid::default::Transform2D<S>;

/// Alias for `euclid::default::Rotation2D`
pub type Rotation<S> = euclid::default::Rotation2D<S>;

/// Alias for `euclid::default::Translation2D`
pub type Translation<S> = euclid::Translation2D<S, euclid::UnknownUnit, euclid::UnknownUnit>;

/// Alias for `euclid::default::Scale`
pub use euclid::default::Scale;

/// An angle in radians.
pub use euclid::Angle;

/// Shorthand for `Vector::new(x, y)`.
#[inline]
pub fn vector<S>(x: S, y: S) -> Vector<S> {
    Vector::new(x, y)
}

/// Shorthand for `Point::new(x, y)`.
#[inline]
pub fn point<S>(x: S, y: S) -> Point<S> {
    Point::new(x, y)
}

/// Shorthand for `Size::new(x, y)`.
#[inline]
pub fn size<S>(w: S, h: S) -> Size<S> {
    Size::new(w, h)
}

pub mod traits {
    pub use crate::segment::Segment;

    use crate::{Point, Rotation, Scalar, Scale, Transform, Translation, Vector};

    pub trait Transformation<S> {
        fn transform_point(&self, p: Point<S>) -> Point<S>;
        fn transform_vector(&self, v: Vector<S>) -> Vector<S>;
    }

    impl<S: Scalar> Transformation<S> for Transform<S> {
        fn transform_point(&self, p: Point<S>) -> Point<S> {
            self.transform_point(p)
        }

        fn transform_vector(&self, v: Vector<S>) -> Vector<S> {
            self.transform_vector(v)
        }
    }

    impl<S: Scalar> Transformation<S> for Rotation<S> {
        fn transform_point(&self, p: Point<S>) -> Point<S> {
            self.transform_point(p)
        }

        fn transform_vector(&self, v: Vector<S>) -> Vector<S> {
            self.transform_vector(v)
        }
    }

    impl<S: Scalar> Transformation<S> for Translation<S> {
        fn transform_point(&self, p: Point<S>) -> Point<S> {
            self.transform_point(p)
        }

        fn transform_vector(&self, v: Vector<S>) -> Vector<S> {
            v
        }
    }

    impl<S: Scalar> Transformation<S> for Scale<S> {
        fn transform_point(&self, p: Point<S>) -> Point<S> {
            (*self).transform_point(p)
        }

        fn transform_vector(&self, v: Vector<S>) -> Vector<S> {
            (*self).transform_vector(v)
        }
    }

    // Automatically implement Transformation for all &Transformation.
    impl<'l, S: Scalar, T: Transformation<S>> Transformation<S> for &'l T {
        #[inline]
        fn transform_point(&self, p: Point<S>) -> Point<S> {
            (*self).transform_point(p)
        }

        #[inline]
        fn transform_vector(&self, v: Vector<S>) -> Vector<S> {
            (*self).transform_vector(v)
        }
    }
}
