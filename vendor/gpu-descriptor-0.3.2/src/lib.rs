//!
//! Library for Vulkan-like APIs to allocated descriptor sets
//! from descriptor pools fast, with least overhead and zero fragmentation.
//!
//! Straightforward usage:
//! ```ignore
//! use gpu_descriptor::DescriptorAllocator;
//!
//! let mut allocator = DescriptorAllocator::new(max_update_after_bind_descriptors_in_all_pools); // Limit as dictated by API for selected hardware
//!
//! let result = allocator.allocate(
//!     device, // Implementation of `gpu_descriptor::DescriptorDevice`. Comes from plugins.
//!     layout, // Descriptor set layout recognized by device's type.
//!     flags,  // Flags specified when layout was created.
//!     layout_descriptor_count, // Descriptors count in the layout.
//!     count, // count of sets to allocated.
//! );
//! ```
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

extern crate alloc;

mod allocator;

pub use {crate::allocator::*, gpu_descriptor_types::*};
