//! Code shared between the WGSL front and back ends.

mod diagnostics;
mod to_wgsl;
mod types;

pub use diagnostics::DisplayFilterableTriggeringRule;
pub use to_wgsl::{address_space_str, ToWgsl, TryToWgsl};
pub use types::TypeContext;
