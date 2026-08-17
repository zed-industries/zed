// Copyright 2025 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! StackSafe prevents stack overflows in deeply recursive algorithms by providing intelligent stack
//! management. No more crashes from recursive functions or data structures that exceed the default
//! stack size - StackSafe automatically allocates additional stack space when needed, eliminating
//! the need for manual stack size tuning or complex refactoring to iterative approaches.
//!
//! ## Quick Start
//!
//! Add StackSafe to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! stacksafe = "1"
//! ```
//!
//! Transform recursive functions with the [`#[stacksafe]`](stacksafe) attribute to prevent stack
//! overflow:
//!
//! ```rust
//! use stacksafe::stacksafe;
//!
//! #[stacksafe]
//! fn fibonacci(n: u64) -> u64 {
//!     match n {
//!         0 | 1 => n,
//!         _ => fibonacci(n - 1) + fibonacci(n - 2),
//!     }
//! }
//!
//! // No stack overflow, even for deep recursion
//! println!("Fibonacci of 30: {}", fibonacci(30));
//! ```
//!
//! ## Recursive Data Structures
//!
//! Use [`StackSafe<T>`] to wrap recursive data structures and prevent stack overflow during
//! traversal:
//!
//! ```rust
//! use stacksafe::StackSafe;
//! use stacksafe::stacksafe;
//!
//! #[derive(Debug, Clone)]
//! enum BinaryTree {
//!     Leaf(i32),
//!     Node {
//!         value: i32,
//!         left: Box<StackSafe<BinaryTree>>,
//!         right: Box<StackSafe<BinaryTree>>,
//!     },
//! }
//!
//! #[stacksafe]
//! fn tree_sum(tree: &BinaryTree) -> i32 {
//!     match tree {
//!         BinaryTree::Leaf(value) => *value,
//!         BinaryTree::Node { value, left, right } => value + tree_sum(left) + tree_sum(right),
//!     }
//! }
//! ```
//!
//! ## How It Works
//!
//! - [`#[stacksafe]`](stacksafe) attribute monitors remaining stack space at function entry points.
//!   When available space falls below a threshold (default: 128 KiB), it automatically allocates a
//!   new stack segment (default: 2 MiB) and continues execution, preventing stack overflow.
//!
//! - [`StackSafe<T>`] is a wrapper type that transparently implement common traits like [`Clone`],
//!   [`Debug`], and [`PartialEq`] with `#[stacksafe]` support, ensuring stack-safe operations on
//!   recursive data structures without risking overflow.
//!
//! - In `debug` builds, accessing [`StackSafe<T>`] performs additional checks to ensure the current
//!   function is properly annotated with `#[stacksafe]`, helping catch potential issues during
//!   development.
//!
//! Read this [blog post](https://fast.github.io/blog/stacksafe-taming-recursion-in-rust-without-stack-overflow/)
//! for an in-depth explanation of StackSafe's design and implementation.
//!
//! ## Configuration
//!
//! Customize stack management behavior:
//!
//! ```rust
//! use stacksafe::set_minimum_stack_size;
//! use stacksafe::set_stack_allocation_size;
//!
//! // Trigger allocation when < 64 KiB remaining (default: 128 KiB).
//! set_minimum_stack_size(64 * 1024);
//!
//! // Allocate 4 MiB stacks for deep recursion (default: 2 MiB).
//! set_stack_allocation_size(4 * 1024 * 1024);
//! ```
//!
//! ## Feature Flags
//!
//! StackSafe supports several optional features:
//!
//! - `serde`: Provides stack-safe serialization and deserialization for [`StackSafe<T>`].
//!
//! ## Platform Support
//!
//! StackSafe works on all major platforms supported by the [`stacker`](https://crates.io/crates/stacker) crate, including:
//!
//! - Linux (x86_64, ARM64, others)
//! - macOS (Intel, Apple Silicon)
//! - Windows (MSVC, GNU)
//! - FreeBSD, NetBSD, OpenBSD
//! - And more...

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod internal;

use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// Attribute macro for automatic stack overflow prevention in recursive functions.
///
/// This macro transforms functions to automatically check available stack space
/// and allocate new stack segments when needed, preventing stack overflow in
/// deeply recursive scenarios.
///
/// # Examples
///
/// ```rust
/// use stacksafe::stacksafe;
///
/// #[stacksafe]
/// fn factorial(n: u64) -> u64 {
///     if n <= 1 { 1 } else { n * factorial(n - 1) }
/// }
/// ```
///
/// For recursive data structures:
///
/// ```rust
/// use stacksafe::StackSafe;
/// use stacksafe::stacksafe;
///
/// struct TreeNode<T> {
///     value: T,
///     left: Option<Box<StackSafe<TreeNode<T>>>>,
///     right: Option<Box<StackSafe<TreeNode<T>>>>,
/// }
///
/// #[stacksafe]
/// fn tree_depth<T>(node: &Option<Box<StackSafe<TreeNode<T>>>>) -> usize {
///     match node {
///         None => 0,
///         Some(n) => 1 + tree_depth(&n.left).max(tree_depth(&n.right)),
///     }
/// }
/// ```
///
/// # Limitations
///
/// - Cannot be applied to `async` functions
/// - Cannot be applied to `const` functions
/// - Functions with `impl Trait` return types may need type annotations
/// - Adds small runtime overhead for stack size checking
pub use stacksafe_macro::stacksafe;

static MINIMUM_STACK_SIZE: AtomicUsize = AtomicUsize::new(128 * 1024);
static STACK_ALLOC_SIZE: AtomicUsize = AtomicUsize::new(2 * 1024 * 1024);

/// Configures the minimum stack space threshold for triggering stack allocation in bytes.
///
/// When a function marked with [`#[stacksafe]`](stacksafe) is called and the remaining stack
/// space is less than this threshold, a new stack segment will be allocated.
///
/// Defaults to 128 KiB.
pub fn set_minimum_stack_size(bytes: usize) {
    MINIMUM_STACK_SIZE.store(bytes, Ordering::Relaxed);
}

/// Returns the current minimum stack space threshold in bytes.
///
/// This value determines when new stack segments are allocated for functions
/// marked with [`#[stacksafe]`](stacksafe).
pub fn get_minimum_stack_size() -> usize {
    MINIMUM_STACK_SIZE.load(Ordering::Relaxed)
}

/// Configures the size of newly allocated stack segments in bytes.
///
/// When a function marked with [`#[stacksafe]`](stacksafe) needs more stack space,
/// it allocates a new stack segment of this size.
///
/// Defaults to 2 MiB.
pub fn set_stack_allocation_size(bytes: usize) {
    STACK_ALLOC_SIZE.store(bytes, Ordering::Relaxed);
}

/// Returns the current stack allocation size in bytes.
///
/// This is the size of new stack segments allocated when functions marked
/// with [`#[stacksafe]`](stacksafe) require additional stack space.
pub fn get_stack_allocation_size() -> usize {
    STACK_ALLOC_SIZE.load(Ordering::Relaxed)
}

/// A wrapper type for recursive data structures with automatic stack-safe operations.
///
/// [`StackSafe<T>`] wraps values that are part of recursive data structures, ensuring
/// that operations like cloning, dropping, comparison, and serialization are performed
/// safely without risking stack overflow.
///
/// The wrapper provides transparent access to the underlying value through [`Deref`]
/// and [`DerefMut`], but enforces that such access occurs within a stack-safe context
/// (i.e., within a function marked with [`#[stacksafe]`](stacksafe)).
pub struct StackSafe<T>(std::mem::ManuallyDrop<T>);

impl<T> StackSafe<T> {
    /// Creates a new [`StackSafe<T>`] wrapper around the given value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacksafe::StackSafe;
    ///
    /// let wrapped = StackSafe::new(vec![1, 2, 3]);
    /// ```
    pub fn new(value: T) -> Self {
        StackSafe(std::mem::ManuallyDrop::new(value))
    }

    /// Consumes the [`StackSafe<T>`] wrapper and returns the inner value.
    ///
    /// # Panics
    ///
    /// In debug builds, panics if called outside of a stack-safe context.
    /// This helps ensure that recursive data structure access is properly
    /// protected against stack overflow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[stacksafe::stacksafe]
    /// # fn main() {
    /// use stacksafe::StackSafe;
    ///
    /// let wrapped = StackSafe::new(vec![1, 2, 3]);
    /// let inner = wrapped.into_inner();
    /// assert_eq!(inner, vec![1, 2, 3]);
    /// # }
    /// ```
    #[track_caller]
    pub fn into_inner(mut self) -> T {
        debug_assert!(
            crate::internal::is_protected(),
            "`StackSafe` should only be accessed within a stack-safe context\n\
            help: add `#[stacksafe::stacksafe]` to the function containing this access"
        );

        let value = unsafe { std::mem::ManuallyDrop::take(&mut self.0) };
        std::mem::forget(self);
        value
    }
}

impl<T> From<T> for StackSafe<T> {
    fn from(value: T) -> Self {
        StackSafe::new(value)
    }
}

impl<T: Default> Default for StackSafe<T> {
    fn default() -> Self {
        StackSafe(std::mem::ManuallyDrop::new(T::default()))
    }
}

impl<T> Deref for StackSafe<T> {
    type Target = T;

    /// Provides transparent access to the wrapped value.
    ///
    /// # Panics
    ///
    /// In debug builds, panics if called outside of a stack-safe context.
    /// This helps ensure that recursive data structure access is properly
    /// protected against stack overflow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacksafe::StackSafe;
    /// use stacksafe::stacksafe;
    ///
    /// #[stacksafe]
    /// fn get_length(data: &StackSafe<Vec<i32>>) -> usize {
    ///     data.len() // Automatic deref to Vec<i32>
    /// }
    /// ```
    #[track_caller]
    fn deref(&self) -> &Self::Target {
        debug_assert!(
            crate::internal::is_protected(),
            "`StackSafe` should only be accessed within a stack-safe context\n\
            help: add `#[stacksafe::stacksafe]` to the function containing this access"
        );

        &self.0
    }
}

impl<T> DerefMut for StackSafe<T> {
    /// Provides transparent mutable access to the wrapped value.
    ///
    /// # Panics
    ///
    /// In debug builds, panics if called outside of a stack-safe context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacksafe::StackSafe;
    /// use stacksafe::stacksafe;
    ///
    /// #[stacksafe]
    /// fn append_value(data: &mut StackSafe<Vec<i32>>, value: i32) {
    ///     data.push(value); // Automatic deref to Vec<i32>
    /// }
    /// ```
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        debug_assert!(
            crate::internal::is_protected(),
            "`StackSafe` should only be accessed within a stack-safe context\n\
            help: add `#[stacksafe::stacksafe]` to the function containing this access"
        );

        &mut self.0
    }
}

impl<T: Clone> Clone for StackSafe<T> {
    #[stacksafe(crate = crate)]
    fn clone(&self) -> Self {
        StackSafe(self.0.clone())
    }
}

impl<T> Drop for StackSafe<T> {
    #[stacksafe(crate = crate)]
    fn drop(&mut self) {
        unsafe {
            std::mem::ManuallyDrop::drop(&mut self.0);
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for StackSafe<T> {
    #[stacksafe(crate = crate)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", *self.0)
        } else {
            write!(f, "{:?}", *self.0)
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for StackSafe<T> {
    #[stacksafe(crate = crate)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#}", *self.0)
        } else {
            write!(f, "{}", *self.0)
        }
    }
}

impl<T: PartialEq> PartialEq for StackSafe<T> {
    #[stacksafe(crate = crate)]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq> Eq for StackSafe<T> {}

impl<T: PartialOrd> PartialOrd for StackSafe<T> {
    #[stacksafe(crate = crate)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord> Ord for StackSafe<T> {
    #[stacksafe(crate = crate)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: std::hash::Hash> std::hash::Hash for StackSafe<T> {
    #[stacksafe(crate = crate)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for StackSafe<T> {
    #[stacksafe(crate = crate)]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'a, T: serde::Deserialize<'a>> serde::Deserialize<'a> for StackSafe<T> {
    #[stacksafe(crate = crate)]
    fn deserialize<D: serde::Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let value = T::deserialize(deserializer)?;
        Ok(StackSafe(std::mem::ManuallyDrop::new(value)))
    }
}
