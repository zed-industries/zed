use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// Information about a namespace
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Namespace {
    /// Namespace type
    pub ns_type: OsString,
    /// Handle to the namespace
    pub path: PathBuf,
    /// Namespace identifier (inode number)
    pub identifier: u64,
    /// Device id of the namespace
    pub device_id: u64,
}

impl PartialEq for Namespace {
    fn eq(&self, other: &Self) -> bool {
        // see https://lore.kernel.org/lkml/87poky5ca9.fsf@xmission.com/
        self.identifier == other.identifier && self.device_id == other.device_id
    }
}

/// All namespaces of a process.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Namespaces(pub HashMap<OsString, Namespace>);
