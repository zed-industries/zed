//! Utility functions for improving error messages in builder's generated code.
//!
//! These free functions are simple wrappers over the respective traits. They allow the
//! generated code to pass the concrete type of the member using the turbofish syntax,
//! which improves the compile errors when the member's type `T` doesn't implement
//! the target trait.
//!
//! They improve the spans of error messages because compiler knows that it needs to
//! point to the origin of the offending type (member's type T) from the turbofish
//! syntax to where the type came from (original code written by the user).
use core::fmt::Debug;

#[inline(always)]
pub fn clone_member<T: Clone>(member: &Option<T>) -> Option<T> {
    member.clone()
}

#[inline(always)]
pub fn as_dyn_debug<T: Debug>(member: &T) -> &dyn Debug {
    member
}

#[inline(always)]
pub fn copy_member<T: Copy>(member: &Option<T>) -> Option<T> {
    *member
}
