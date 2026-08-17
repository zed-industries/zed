#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod decode;
pub mod encode;
mod errors;
mod marker;

pub use crate::marker::Marker;

/// Version of the MessagePack [spec](http://github.com/msgpack/msgpack/blob/master/spec.md).
pub const MSGPACK_VERSION: u32 = 5;
