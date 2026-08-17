//! Constrained conversion functions for assisting in situations where type
//! inference is difficult.
//!
//! For example, when declaring `glium` uniforms, we need to convert to fixed
//! length arrays. We can use the `Into` trait directly, but it is rather ugly!
//!
//! --- Doc-test disabled because glium causes problems with nightly-2019-01-01 needed for "simd"
//! ` ` `rust
//! #[macro_use]
//! extern crate glium;
//! extern crate cgmath;
//!
//! use cgmath::{Matrix4, Point2};
//! use cgmath::prelude::*;
//!
//! # fn main() {
//! let point = Point2::new(1, 2);
//! let matrix = Matrix4::from_scale(2.0);
//!
//! let uniforms = uniform! {
//!     point: Into::<[_; 2]>::into(point),
//!     matrix: Into::<[[_; 4]; 4]>::into(matrix),
//!     // Yuck!! (ﾉಥ益ಥ）ﾉ﻿ ┻━┻
//! };
//! # }
//! ` ` `
//!
//! Instead, we can use the conversion functions from the `conv` module:
//!
//! --- Doc-test disabled because glium causes problems nightly-2019-01-01 needed for "simd"
//! ` ` `rust
//! #[macro_use]
//! extern crate glium;
//! extern crate cgmath;
//!
//! use cgmath::{Matrix4, Point2};
//! use cgmath::prelude::*;
//! use cgmath::conv::*;
//!
//! # fn main() {
//! let point = Point2::new(1, 2);
//! let matrix = Matrix4::from_scale(2.0);
//!
//! let uniforms = uniform! {
//!     point: array2(point),
//!     matrix: array4x4(matrix),
//!     // ┬─┬ノ( º _ ºノ)
//! };
//! # }
//! ` ` `

/// Force a conversion into a 2-element array.
#[inline]
pub fn array2<T, A: Into<[T; 2]>>(value: A) -> [T; 2] {
    value.into()
}

/// Force a conversion into a 3-element array.
#[inline]
pub fn array3<T, A: Into<[T; 3]>>(value: A) -> [T; 3] {
    value.into()
}

/// Force a conversion into a 4-element array.
#[inline]
pub fn array4<T, A: Into<[T; 4]>>(value: A) -> [T; 4] {
    value.into()
}

/// Force a conversion into a 2x2-element array.
#[inline]
pub fn array2x2<T, A: Into<[[T; 2]; 2]>>(value: A) -> [[T; 2]; 2] {
    value.into()
}

/// Force a conversion into a 3x3-element array.
#[inline]
pub fn array3x3<T, A: Into<[[T; 3]; 3]>>(value: A) -> [[T; 3]; 3] {
    value.into()
}

/// Force a conversion into a 4x4-element array.
#[inline]
pub fn array4x4<T, A: Into<[[T; 4]; 4]>>(value: A) -> [[T; 4]; 4] {
    value.into()
}
