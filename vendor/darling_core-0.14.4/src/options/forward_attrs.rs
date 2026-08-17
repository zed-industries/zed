use syn::NestedMeta;

use crate::util::PathList;
use crate::{FromMeta, Result};

/// A rule about which attributes to forward to the generated struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForwardAttrs {
    All,
    Only(PathList),
}

impl ForwardAttrs {
    /// Returns `true` if this will not forward any attributes.
    pub fn is_empty(&self) -> bool {
        match *self {
            ForwardAttrs::All => false,
            ForwardAttrs::Only(ref list) => list.is_empty(),
        }
    }
}

impl FromMeta for ForwardAttrs {
    fn from_word() -> Result<Self> {
        Ok(ForwardAttrs::All)
    }

    fn from_list(nested: &[NestedMeta]) -> Result<Self> {
        Ok(ForwardAttrs::Only(PathList::from_list(nested)?))
    }
}
