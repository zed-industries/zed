//! Behavior semantics for `ArrayDeque`.
//!
//! `ArrayDeque` provides two different behaviors, `Saturating` and `Wrapping`,
//! determining whether to remove existing element automatically when pushing
//! to a full deque.
//!
//! The behavior is indicated by a marker type parameter of `ArrayDeque`,
//! which defaults to `Saturating`.
//!
//! ## Saturating
//!
//! Pushing any element when `ArrayDeque` is full will directly return an `Err(CapacityError)`
//! containing the element attempting to push, leaving the `ArrayDeque` unchanged.
//!
//! ```
//! use arraydeque::{ArrayDeque, Saturating, CapacityError};
//!
//! let mut tester: ArrayDeque<_, 2, Saturating> = ArrayDeque::new();
//!
//! assert_eq!(tester.push_back(1), Ok(()));
//! assert_eq!(tester.push_back(2), Ok(()));
//! assert_eq!(tester.push_back(3), Err(CapacityError { element: 3 }));
//! ```
//!
//! ## Wrapping
//!
//! Pushing any element when `ArrayDeque` is full will pop an element at
//! the other side to spare room.
//!
//! ```
//! use arraydeque::{ArrayDeque, Wrapping};
//!
//! let mut tester: ArrayDeque<_, 2, Wrapping> = ArrayDeque::new();
//!
//! assert_eq!(tester.push_back(1), None);
//! assert_eq!(tester.push_back(2), None);
//! assert_eq!(tester.push_back(3), Some(1));
//! ```

/// Marker trait for indicating behaviors of `ArrayDeque`.
pub trait Behavior {}

/// Behavior for `ArrayDeque` that specifies saturating write semantics.
pub struct Saturating;

impl Behavior for Saturating {}

/// Behavior for `ArrayDeque` that specifies wrapping write semantics.
pub struct Wrapping;

impl Behavior for Wrapping {}
