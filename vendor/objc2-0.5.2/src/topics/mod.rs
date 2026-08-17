//! Various explanations and topics of discussion.

pub mod about_generated;

#[cfg(not(feature = "gnustep-1-7"))]
#[doc = include_str!("core_foundation_interop.md")]
pub mod core_foundation_interop {}
#[doc = include_str!("layered_safety.md")]
pub mod layered_safety {}
#[cfg(not(doctest))]
#[doc = include_str!("../../CHANGELOG.md")]
pub mod changelog {}
