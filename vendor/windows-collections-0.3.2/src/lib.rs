#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]
#![expect(
    missing_docs,
    non_snake_case,
    non_camel_case_types,
    clippy::missing_transmute_annotations
)]

mod bindings;
pub use bindings::*;

#[cfg(feature = "std")]
const E_BOUNDS: windows_core::HRESULT = windows_core::HRESULT(0x8000000B_u32 as _);

#[cfg(feature = "std")]
mod iterable;
#[cfg(feature = "std")]
mod map_view;
#[cfg(feature = "std")]
mod vector_view;
