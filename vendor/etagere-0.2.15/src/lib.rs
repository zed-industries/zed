//! Dynamic texture atlas allocation using the shelf packing algorithm.
//!
//! ## Texture atlas allocation
//!
//! The goal of a texture atlas allocator is to pack multiple rectangles into a larger one.
//! When rendering 2D or 3D graphics on the GPU, this packing important for draw call batching. 
//!
//! This crate provides two implementations of the shelf packing algorithm for *dynamic*
//! texture atlas allocation (dynamic here means supporting both allocation and deallocation).
//!
//! [A thousand ways to pack the bin](https://github.com/juj/RectangleBinPack/blob/master/RectangleBinPack.pdf)
//! is a good resource to learn about rectangle packing algorithms, although it does not not cover
//! deallocation which complicates the problem space a fair bit.
//!
//! The shelf packing algorithm is explained in the above document. It isn't the most efficient
//! packing strategy, however its simplicity makes it possible to support reasonably fast deallocation.
//! Note that the [guillotiere](https://crates.io/crates/guillotiere) crate implements a similar API
//! using the guillotine packing algorithm and may or may not provide better results depending on the
//! type of workload.
//!
//! ## Two allocators
//!
//! - [`AtlasAllocator`] Tracks allocations for each individual item and does a reasonable
//!   job of dealing with fragmentation, at a runtime cost.
//! - [`BucketedAtlasAllocator`] groups items by buckets and only reclaim the space occupied
//!   by a bucket when all of its items are deallocated. In addition, it has limited support
//!   for merging consecutive empty shelves. These limitations allow faster allocation and
//!   deallocation, making it an appealing option when the atlas is expected to hold a very
//!   large amount of small items.
//!
//! Both allocators support:
//! - Requiring an alignment for the items.
//! - Packing into vertical shelves instead of horizontal ones (might provide a better output
//!   for some workloads).
//! - Splitting the allocator into multiple columns in order to make shelves smaller and allow
//!   more of them.
//! - Dumping the content of the atlas in SVG format for easy debugging.
//!
//! See [`AllocatorOptions`](struct.AllocatorOptions.html)
//!
//! In addition, this repository contains a command-line application to experiment with and
//! test the implementations interactively.
//!
//! ## Example
//!
//! ```rust
//! use etagere::*;
//!
//! let mut atlas = AtlasAllocator::new(size2(1000, 1000));
//!
//! let a = atlas.allocate(size2(100, 100)).unwrap();
//! let b = atlas.allocate(size2(900, 200)).unwrap();
//! println!("Allocated {:?} and {:?}", a.rectangle, b.rectangle);
//!
//! atlas.deallocate(a.id);
//!
//! let c = atlas.allocate(size2(300, 200)).unwrap();
//!
//! atlas.deallocate(c.id);
//! atlas.deallocate(b.id);
//! ```
//!
//! ## License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//!
//! [`AtlasAllocator`]: struct.AtlasAllocator.html
//! [`BucketedAtlasAllocator`]: struct.BucketedAtlasAllocator.html

#[cfg(feature = "serde")]
#[macro_use]
pub extern crate serde;
pub extern crate euclid;

mod bucketed;
mod allocator;
#[cfg(feature = "ffi")]
pub mod ffi;

pub use allocator::*;
pub use bucketed::*;
pub use euclid::{point2, size2};

pub type Point = euclid::default::Point2D<i32>;
pub type Size = euclid::default::Size2D<i32>;
pub type Rectangle = euclid::default::Box2D<i32>;

/// Options to tweak the behavior of the atlas allocator.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct AllocatorOptions {
    /// Align item sizes to a multiple of this alignment.
    ///
    /// Default value: [1, 1] (no alignment).
    pub alignment: Size,
    /// Use vertical instead of horizontal shelves.
    ///
    /// Default value: false.
    pub vertical_shelves: bool,
    /// If possible split the allocator's surface into multiple columns.
    ///
    /// Having multiple columns allows having more (smaller shelves).
    ///
    /// Default value: 1.
    pub num_columns: i32,
}

pub const DEFAULT_OPTIONS: AllocatorOptions = AllocatorOptions {
    vertical_shelves: false,
    alignment: size2(1, 1),
    num_columns: 1,
};

impl Default for AllocatorOptions {
    fn default() -> Self {
        DEFAULT_OPTIONS
    }
}

/// The `AllocId` and `Rectangle` resulting from an allocation.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Allocation {
    pub id: AllocId,
    pub rectangle: Rectangle,
}

/// ID referring to an allocated rectangle.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct AllocId(pub(crate) u32);

impl AllocId {
    #[inline]
    pub(crate) fn new(index: u16, gen: u16) -> Self {
        AllocId(index as u32 | ((gen as u32) << 16))
    }

    #[inline]
    pub(crate) fn index(&self) -> u16 {
        self.0 as u16
    }

    #[inline]
    pub(crate) fn generation(&self) -> u16 {
        (self.0 >> 16) as u16
    }

    #[inline]
    pub fn serialize(&self) -> u32 {
        self.0
    }

    #[inline]
    pub fn deserialize(bytes: u32) -> Self {
        AllocId(bytes)
    }
}

