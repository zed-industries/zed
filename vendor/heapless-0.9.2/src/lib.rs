//! `static` friendly data structures that don't require dynamic memory allocation
//!
//! The core principle behind `heapless` is that its data structures are backed by a *static* memory
//! allocation. For example, you can think of `heapless::Vec` as an alternative version of
//! `std::Vec` with fixed capacity and that can't be re-allocated on the fly (e.g. via `push`).
//!
//! All `heapless` data structures store their memory allocation *inline* and specify their capacity
//! via their type parameter `N`. This means that you can instantiate a `heapless` data structure on
//! the stack, in a `static` variable, or even in the heap.
//!
//! ```
//! use heapless::Vec; // fixed capacity `std::Vec`
//!
//! // on the stack
//! let mut xs: Vec<u8, 8> = Vec::new(); // can hold up to 8 elements
//! xs.push(42)?;
//! assert_eq!(xs.pop(), Some(42));
//!
//! // in a `static` variable
//! static mut XS: Vec<u8, 8> = Vec::new();
//!
//! let xs = unsafe { &mut XS };
//!
//! xs.push(42)?;
//! assert_eq!(xs.pop(), Some(42));
//!
//! // in the heap (though kind of pointless because no reallocation)
//! let mut ys: Box<Vec<u8, 8>> = Box::new(Vec::new());
//! ys.push(42)?;
//! assert_eq!(ys.pop(), Some(42));
//! # Ok::<(), u8>(())
//! ```
//!
//! Because they have fixed capacity `heapless` data structures don't implicitly reallocate. This
//! means that operations like `heapless::Vec.push` are *truly* constant time rather than amortized
//! constant time with potentially unbounded (depends on the allocator) worst case execution time
//! (which is bad/unacceptable for hard real time applications).
//!
//! `heapless` data structures don't use a memory allocator which means no risk of an uncatchable
//! Out Of Memory (OOM) condition while performing operations on them. It's certainly possible to
//! run out of capacity while growing `heapless` data structures, but the API lets you handle this
//! possibility by returning a `Result` on operations that may exhaust the capacity of the data
//! structure.
//!
//! List of currently implemented data structures:
#![cfg_attr(
    any(
        arm_llsc,
        all(
            target_pointer_width = "32",
            any(target_has_atomic = "64", feature = "portable-atomic")
        ),
        all(
            target_pointer_width = "64",
            any(
                all(target_has_atomic = "128", feature = "nightly"),
                feature = "portable-atomic"
            )
        )
    ),
    doc = "- [`Arc`][pool::arc::Arc]: Like `std::sync::Arc` but backed by a lock-free memory pool rather than `[global_allocator]`."
)]
#![cfg_attr(
    any(
        arm_llsc,
        all(
            target_pointer_width = "32",
            any(target_has_atomic = "64", feature = "portable-atomic")
        ),
        all(
            target_pointer_width = "64",
            any(
                all(target_has_atomic = "128", feature = "nightly"),
                feature = "portable-atomic"
            )
        )
    ),
    doc = "- [`Box`][pool::boxed::Box]: Like `std::boxed::Box` but backed by a lock-free memory pool rather than `[global_allocator]`."
)]
#![cfg_attr(
    any(
        arm_llsc,
        all(
            target_pointer_width = "32",
            any(target_has_atomic = "64", feature = "portable-atomic")
        ),
        all(
            target_pointer_width = "64",
            any(
                all(target_has_atomic = "128", feature = "nightly"),
                feature = "portable-atomic"
            )
        )
    ),
    doc = "- [`Object`](pool::object::Object): Objects managed by an object pool."
)]
//! - [`BinaryHeap`]: A priority queue.
//! - [`Deque`]: A double-ended queue.
//! - [`HistoryBuf`]: A “history buffer”, similar to a write-only ring buffer.
//! - [`IndexMap`]: A hash table.
//! - [`IndexSet`]: A hash set.
//! - [`LinearMap`]: A linear map.
//! - [`SortedLinkedList`](sorted_linked_list::SortedLinkedList): A sorted linked list.
//! - [`String`]: A string.
//! - [`Vec`]: A vector.
//! - [`mpmc::MpMcQueue`](mpmc): A lock-free multiple-producer, multiple-consumer queue.
//! - [`spsc::Queue`](spsc): A lock-free single-producer, single-consumer queue.
//!
//! # Zeroize Support
//!
//! The `zeroize` feature enables secure memory wiping for the data structures via the [`zeroize`](https://crates.io/crates/zeroize)
//! crate. Sensitive data can be properly erased from memory when no longer needed.
//!
//! When zeroizing a container, all underlying memory (including unused portion of the containers)
//! is overwritten with zeros, length counters are reset, and the container is left in a valid but
//! empty state that can be reused.
//!
//! Check the [documentation of the zeroize crate](https://docs.rs/zeroize/) for more information.
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate does *not* have a Minimum Supported Rust Version (MSRV) and may make use of language
//! features and API in the standard library available in the latest stable Rust version.
//!
//! In other words, changes in the Rust version requirement of this crate are not considered semver
//! breaking change and may occur in patch version releases.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]
#![cfg_attr(
    all(
        feature = "nightly",
        target_pointer_width = "64",
        target_has_atomic = "128"
    ),
    feature(integer_atomics)
)]
#![warn(
    clippy::use_self,
    clippy::too_long_first_doc_paragraph,
    clippy::redundant_pub_crate,
    clippy::option_if_let_else,
    clippy::ptr_as_ptr,
    clippy::ref_as_ptr,
    clippy::doc_markdown,
    clippy::semicolon_if_nothing_returned,
    clippy::if_not_else
)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use binary_heap::BinaryHeap;
pub use c_string::CString;
pub use deque::Deque;
pub use history_buf::{HistoryBuf, OldestOrdered};
pub use index_map::IndexMap;
pub use index_set::IndexSet;
pub use len_type::LenType;
pub use linear_map::LinearMap;
pub use string::String;

pub use vec::{Vec, VecView};

#[macro_use]
#[cfg(test)]
mod test_helpers;

pub mod c_string;
pub mod deque;
pub mod history_buf;
pub mod index_map;
pub mod index_set;
mod len_type;
pub mod linear_map;
mod slice;
pub mod storage;
pub mod string;
pub mod vec;

// FIXME: Workaround a compiler ICE in rust 1.83 to 1.86
// https://github.com/rust-lang/rust/issues/138979#issuecomment-2760839948
#[expect(dead_code)]
fn dead_code_ice_workaround() {}

#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;

pub mod binary_heap;
#[cfg(feature = "bytes")]
mod bytes;
#[cfg(feature = "defmt")]
mod defmt;
#[cfg(any(
    // assume we have all atomics available if we're using portable-atomic
    feature = "portable-atomic",
    // target has native atomic CAS (mpmc_large requires usize, otherwise just u8)
    all(feature = "mpmc_large", target_has_atomic = "ptr"),
    all(not(feature = "mpmc_large"), target_has_atomic = "8")
))]
pub mod mpmc;
#[cfg(any(
    arm_llsc,
    all(
        target_pointer_width = "32",
        any(target_has_atomic = "64", feature = "portable-atomic")
    ),
    all(
        target_pointer_width = "64",
        any(
            all(target_has_atomic = "128", feature = "nightly"),
            feature = "portable-atomic"
        )
    )
))]
pub mod pool;
pub mod sorted_linked_list;
#[cfg(any(
    // assume we have all atomics available if we're using portable-atomic
    feature = "portable-atomic",
    // target has native atomic CAS. Note this is too restrictive, spsc requires load/store only, not CAS.
    // This should be `cfg(target_has_atomic_load_store)`, but that's not stable yet.
    target_has_atomic = "ptr",
    // or the current target is in a list in build.rs of targets known to have load/store but no CAS.
    has_atomic_load_store
))]
pub mod spsc;

#[cfg(feature = "ufmt")]
mod ufmt;

#[cfg(feature = "embedded-io-v0.7")]
mod embedded_io;

/// Implementation details for macros.
/// Do not use. Used for macros only. Not covered by semver guarantees.
#[doc(hidden)]
pub mod _export {
    pub use crate::string::format;
}

/// The error type for fallible [`Vec`] and [`String`] methods.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct CapacityError;

impl core::fmt::Display for CapacityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("insufficient capacity")
    }
}

impl core::error::Error for CapacityError {}
