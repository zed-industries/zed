//! # Unarray
//!
//! Helper utilities for working with arrays of uninitialized memory.
//!
//! ## Current stable Rust
//!
//! Creating arrays in Rust can be somewhat painful. Currently, your best option in the general
//! case is to allocate your elements in a `Vec`, then convert to an array:
//! ```
//! # use core::convert::TryInto;
//! const LEN: usize = 1000;
//! let mut elements = Vec::with_capacity(LEN);  // heap allocation here
//!
//! for i in 0..LEN {
//!   elements.push(123);
//! }
//!
//! let result: [i32; LEN] = elements.try_into().unwrap();
//! ```
//! This needlessly allocates space on the heap, which is then immediately freed. If your type
//! implements `Copy`, and has a sensible default value, you can avoid this allocation by creating
//! an array literal (e.g. `[0; 1000]`), then iterating over each element and setting it, but this
//! also incurrs an unnecessary initialization cost. Why set each element to `0`, then set it
//! again, when you could just set it once?
//!
//! ## `uninit_buf` and `mark_initialized`
//!
//! The lowest-level tools provided by this library are the pair of functions: [`uninit_buf`] and
//! [`mark_initialized`]. These are ergonomic wrappers around the [`core::mem::MaybeUninit`] type.
//! Roughly speaking, most uses of these functions will follow the following steps:
//!  - Stack-allocate a region of uninitialized memory with [`uninit_buf`]
//!  - Initialize each element
//!  - Unsafely declare that all elements are initialized using [`mark_initialized`]
//!
//! For example:
//! ```
//! # use unarray::*;
//! let mut buffer = uninit_buf();  
//!
//! for elem in &mut buffer {
//!   elem.write(123);  
//! }
//!
//! let result = unsafe { mark_initialized(buffer) };
//! assert_eq!(result, [123; 1000]);
//! ```
//! These functions closely map onto tools provided by [`core::mem::MaybeUninit`], so should feel
//! familiar. However, [`mark_initialized`] is an unsafe function, since it's possible to create
//! uninitialized values that aren't wrapped in `MaybeUninit`. It's up to the programmer to make
//! sure every element has been initialized before calling [`mark_initialized`], otherwise it's UB.
//!
//! For this, there are also fully safe APIs that cover some of the common patterns via an
//! extension trait on `[T; N]`:
//!
//! ## `UnarrayArrayExt` extension trait
//!
//! ```
//! # use unarray::*;
//! // mapping an array via a `Result`
//! let strings = ["123", "234"];
//! let numbers = strings.map_result(|s| s.parse());
//! assert_eq!(numbers, Ok([123, 234]));
//!
//! let bad_strings = ["123", "uh oh"];
//! let result = bad_strings.map_result(|s| s.parse::<i32>());
//! assert!(result.is_err());  // since one of the element fails, the whole operation fails
//! ```
//! There is also `map_option` for functions which return an `Option`
//!
//! ## Collecting iterators
//!
//! Iterators generally don't know their length at compile time. But it's often the case that the
//! programmer knows the length ahead of time. In cases like this, it's common to want to collect
//! these elements into an array, without heap allocation or initializing default elements.
//!
//! Arrays don't implement `FromIterator` for this very reason. So this library provides
//! `ArrayFromIter`:
//! ```
//! # use unarray::*;
//! let iter = [1, 2, 3].into_iter().map(|i| i * 2);
//! let ArrayFromIter(array) = iter.collect();  // inferred to be `ArrayFromIter::<i32, 3>`
//! assert_eq!(array, Some([2, 4, 6]));
//! ```
//! However, this can fail, since the iterator may not actually yield the right number of elements.
//! In these cases, the inner option is `None`:
//! ```
//! # use unarray::*;
//! let iter = [1, 2, 3, 4].into_iter();
//! match iter.collect() {
//!   ArrayFromIter(Some([a, b, c])) => println!("3 elements, {a}, {b}, {c}"),
//!   ArrayFromIter(None) => println!("not 3 elements"),
//! }
//! ```
//! ## `build_array-*` functions
//!
//! Finally, it's often the case that you want to initialize each array element based on its index.
//! For that, [`build_array`] takes a const generic length, and a function that takes an index and
//! returns an element, and builds the array for you:
//! ```
//! use unarray::*;
//! let array: [usize; 5] = build_array(|i| i * 2);
//! assert_eq!(array, [0, 2, 4, 6, 8]);
//! ```
//! There are also variants that allow fallibly constructing an array, via [`build_array_result`]
//! or [`build_array_option`], similar to [`UnarrayArrayExt::map_result`] and [`UnarrayArrayExt::map_option`].

#![cfg_attr(not(test), no_std)]
#![deny(clippy::missing_safety_doc, missing_docs)]
use core::mem::MaybeUninit;

mod build;
mod map;
mod from_iter;
#[cfg(test)]
mod testing;

pub use build::{build_array, build_array_option, build_array_result};
pub use map::UnarrayArrayExt;
pub use from_iter::ArrayFromIter;

/// Convert a `[MaybeUninit<T>; N]` to a `[T; N]`
///
/// ```
/// # use unarray::*;
/// let mut buffer = uninit_buf::<i32, 1000>();
///
/// for elem in &mut buffer {
///   elem.write(123);
/// }
///
/// let result = unsafe { mark_initialized(buffer) };
/// assert_eq!(result, [123; 1000])
/// ```
///
/// This largely acts as a workaround to the fact that [`core::mem::transmute`] cannot be used with
/// const generic arrays, as it can't prove they have the same size (even when intuitively they are
/// the same, e.g. `[i32; N]` and `[u32; N]`).
///
/// This is similar to the nightly-only [`core::mem::MaybeUninit::array_assume_init`]
///
/// # Safety
///
/// Internally, this uses [`core::mem::transmute_copy`] to convert a `[MaybeUninit<T>; N]` to `[T; N]`.
/// As such, you must make sure every element has been initialized before calling this function. If
/// there are uninitialized elements in `src`, these will be converted to `T`s, which is UB. For
/// example:
/// ```no_run
/// # use unarray::*;
/// // ⚠️ This example produces UB ⚠️
/// let bools = uninit_buf::<bool, 10>();
/// let uh_oh = unsafe { mark_initialized(bools) };  // UB: creating an invalid instance
/// if uh_oh[0] {                                    // double UB: reading from unintiailized memory
///   // ...
/// }
/// ```
/// Even if you never use a value, it's still UB. This is especially true for types with
/// [`core::ops::Drop`] implementations:
/// ```no_run
/// # use unarray::*;
/// // ⚠️ This example produces UB ⚠️
/// let strings = uninit_buf::<String, 10>();
/// let uh_oh = unsafe { mark_initialized(strings) };  // UB: creating an invalid instance
///
/// // uh_oh is dropped here, freeing memory at random addresses
/// ```
pub unsafe fn mark_initialized<T, const N: usize>(src: [MaybeUninit<T>; N]) -> [T; N] {
    core::mem::transmute_copy::<[MaybeUninit<T>; N], [T; N]>(&src)
}

/// Create an array of unintialized memory
///
/// This function is just a safe wrapper around `MaybeUninit::uninit().assume_init()`, which is
/// safe when used to create a `[MaybeUninit<T>; N]`, since this type explicitly requires no
/// initialization
///
/// ```
/// # use unarray::*;
/// let mut buffer = uninit_buf::<i32, 1000>();
///
/// for elem in &mut buffer {
///   elem.write(123);
/// }
///
/// let result = unsafe { mark_initialized(buffer) };
/// assert_eq!(result, [123; 1000])
/// ```
///
/// This is similar to the nightly-only [`core::mem::MaybeUninit::uninit_array`]
pub fn uninit_buf<T, const N: usize>() -> [MaybeUninit<T>; N] {
    // SAFETY:
    // This is safe because we are assuming that a `[MaybeUninit<T>; N]` is initialized. However,
    // since `MaybeUninit` doesn't require initialization, doing nothing counts as "initializing",
    // so this is always safe
    unsafe { MaybeUninit::uninit().assume_init() }
}
