//! UI node types and related data structures.
//!
//! Layouts are composed of multiple nodes, which live in a tree-like data structure.

#[cfg(feature = "taffy_tree")]
use slotmap::{DefaultKey, Key, KeyData};

/// A type representing the id of a single node in a tree of nodes
///
/// Internally it is a wrapper around a u64 and a `NodeId` can be converted to and from
/// and u64 if needed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(u64);
impl NodeId {
    /// Create a new NodeId from a u64 value
    pub const fn new(val: u64) -> Self {
        Self(val)
    }
}

impl From<u64> for NodeId {
    #[inline]
    fn from(raw: u64) -> Self {
        Self(raw)
    }
}
impl From<NodeId> for u64 {
    #[inline]
    fn from(id: NodeId) -> Self {
        id.0
    }
}
impl From<usize> for NodeId {
    #[inline]
    fn from(raw: usize) -> Self {
        Self(raw as u64)
    }
}
impl From<NodeId> for usize {
    #[inline]
    fn from(id: NodeId) -> Self {
        id.0 as usize
    }
}

#[cfg(feature = "taffy_tree")]
impl From<DefaultKey> for NodeId {
    #[inline]
    fn from(key: DefaultKey) -> Self {
        Self(key.data().as_ffi())
    }
}

#[cfg(feature = "taffy_tree")]
impl From<NodeId> for DefaultKey {
    #[inline]
    fn from(key: NodeId) -> Self {
        KeyData::from_ffi(key.0).into()
    }
}
