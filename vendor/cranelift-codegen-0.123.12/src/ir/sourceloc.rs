//! Source locations.
//!
//! Cranelift tracks the original source location of each instruction, and preserves the source
//! location when instructions are transformed.

use core::fmt;
#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// A source location.
///
/// This is an opaque 32-bit number attached to each Cranelift IR instruction. Cranelift does not
/// interpret source locations in any way, they are simply preserved from the input to the output.
///
/// The default source location uses the all-ones bit pattern `!0`. It is used for instructions
/// that can't be given a real source location.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct SourceLoc(u32);

impl SourceLoc {
    /// Create a new source location with the given bits.
    pub fn new(bits: u32) -> Self {
        Self(bits)
    }

    /// Is this the default source location?
    pub fn is_default(self) -> bool {
        self == Default::default()
    }

    /// Read the bits of this source location.
    pub fn bits(self) -> u32 {
        self.0
    }
}

impl Default for SourceLoc {
    fn default() -> Self {
        Self(!0)
    }
}

impl fmt::Display for SourceLoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_default() {
            write!(f, "@-")
        } else {
            write!(f, "@{:04x}", self.0)
        }
    }
}

/// Source location relative to another base source location.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct RelSourceLoc(u32);

impl RelSourceLoc {
    /// Create a new relative source location with the given bits.
    pub fn new(bits: u32) -> Self {
        Self(bits)
    }

    /// Creates a new `RelSourceLoc` based on the given base and offset.
    pub fn from_base_offset(base: SourceLoc, offset: SourceLoc) -> Self {
        if base.is_default() || offset.is_default() {
            Self::default()
        } else {
            Self(offset.bits().wrapping_sub(base.bits()))
        }
    }

    /// Expands the relative source location into an absolute one, using the given base.
    pub fn expand(&self, base: SourceLoc) -> SourceLoc {
        if self.is_default() || base.is_default() {
            Default::default()
        } else {
            SourceLoc::new(self.0.wrapping_add(base.bits()))
        }
    }

    /// Is this the default relative source location?
    pub fn is_default(self) -> bool {
        self == Default::default()
    }
}

impl Default for RelSourceLoc {
    fn default() -> Self {
        Self(!0)
    }
}

impl fmt::Display for RelSourceLoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_default() {
            write!(f, "@-")
        } else {
            write!(f, "@+{:04x}", self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::SourceLoc;
    use alloc::string::ToString;

    #[test]
    fn display() {
        assert_eq!(SourceLoc::default().to_string(), "@-");
        assert_eq!(SourceLoc::new(0).to_string(), "@0000");
        assert_eq!(SourceLoc::new(16).to_string(), "@0010");
        assert_eq!(SourceLoc::new(0xabcdef).to_string(), "@abcdef");
    }
}
