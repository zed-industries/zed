//! Regression test: sections with no `#[in_section]` items must still link.
//!
//!
//! Each section kind that routes through `get_section!` is exercised empty.
#![warn(missing_docs)]
#![allow(unsafe_code)]

use link_section::{
    section, Ref, TypedMovableSection, TypedMutableSection, TypedReferenceSection, TypedSection,
};

/// An empty typed section.
#[section(typed)]
pub static EMPTY_TYPED: TypedSection<u32>;

/// An empty mutable section.
#[section(mutable)]
pub static EMPTY_MUTABLE: TypedMutableSection<u32>;

/// An empty movable section (exercises the value *and* backref symbols).
#[section(movable)]
pub static EMPTY_MOVABLE: TypedMovableSection<u32>;

/// An empty reference section.
#[section(reference)]
pub static EMPTY_REFERENCE: TypedReferenceSection<Ref<u32>>;

/// Asserts every empty section links and reports zero items.
pub fn main() {
    assert!(EMPTY_TYPED.is_empty(), "typed");
    assert_eq!(EMPTY_TYPED.as_slice(), &[] as &[u32]);

    assert!(EMPTY_MUTABLE.is_empty(), "mutable");

    assert!(EMPTY_MOVABLE.is_empty(), "movable");
    // Sorting an empty movable section must also be a no-op rather than touching
    // the (null) backref bounds.
    unsafe { EMPTY_MOVABLE.sort_unstable() };
    assert!(EMPTY_MOVABLE.is_empty(), "movable after sort");

    assert!(EMPTY_REFERENCE.is_empty(), "reference");

    eprintln!("OK: all empty sections linked and report empty");
}
