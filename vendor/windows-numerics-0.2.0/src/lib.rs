#![allow(non_snake_case, clippy::all)]
#![doc = include_str!("../readme.md")]
#![allow(missing_docs)]
#![cfg_attr(all(not(feature = "std")), no_std)]

mod bindings;
pub use bindings::*;

mod matrix3x2;
mod matrix4x4;
mod vector2;
mod vector3;
mod vector4;
