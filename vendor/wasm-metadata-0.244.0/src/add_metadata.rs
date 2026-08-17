use crate::{Producers, rewrite_wasm};
use anyhow::Result;
use std::fmt::Debug;

/// Add metadata (module name, producers) to a WebAssembly file.
///
/// Supports both core WebAssembly modules and components. In components,
/// metadata will be added to the outermost component.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct AddMetadata {
    /// Add a module or component name to the names section
    pub name: AddMetadataField<String>,

    /// Add a programming language to the producers section
    pub language: Vec<(String, String)>,

    /// Add a tool and its version to the producers section
    pub processed_by: Vec<(String, String)>,

    /// Add an SDK and its version to the producers section
    pub sdk: Vec<(String, String)>,

    /// Contact details of the people or organization responsible,
    /// encoded as a freeform string.
    #[cfg(feature = "oci")]
    pub authors: AddMetadataField<crate::Authors>,

    /// A human-readable description of the binary
    #[cfg(feature = "oci")]
    pub description: AddMetadataField<crate::Description>,

    /// License(s) under which contained software is distributed as an SPDX License Expression.
    #[cfg(feature = "oci")]
    pub licenses: AddMetadataField<crate::Licenses>,

    /// URL to get source code for building the image
    #[cfg(feature = "oci")]
    pub source: AddMetadataField<crate::Source>,

    /// URL to find more information on the binary
    #[cfg(feature = "oci")]
    pub homepage: AddMetadataField<crate::Homepage>,

    /// Source control revision identifier for the packaged software.
    #[cfg(feature = "oci")]
    pub revision: AddMetadataField<crate::Revision>,

    /// Version of the packaged software
    #[cfg(feature = "oci")]
    pub version: AddMetadataField<crate::Version>,
}

impl AddMetadata {
    /// Process a WebAssembly binary. Supports both core WebAssembly modules, and WebAssembly
    /// components. The module and component will have, at very least, an empty name and producers
    /// section created.
    pub fn to_wasm(&self, input: &[u8]) -> Result<Vec<u8>> {
        let add_producers = Producers::from_meta(self);
        rewrite_wasm(self, &add_producers, input)
    }
}

/// Defines how to modify a field of the component/module metadata
#[derive(Debug, Clone)]
pub enum AddMetadataField<T: Debug + Clone> {
    /// Keep the existing value of the field
    Keep,
    /// Remove the existing value of the field
    Clear,
    /// Set the field to a new value
    Set(T),
}

impl<T: Debug + Clone> AddMetadataField<T> {
    /// Returns true if the field should be cleared
    pub fn is_clear(&self) -> bool {
        matches!(self, Self::Clear)
    }

    /// Returns true if the field should be kept
    pub fn is_keep(&self) -> bool {
        matches!(self, Self::Keep)
    }
}

impl<T: Debug + Clone> Default for AddMetadataField<T> {
    fn default() -> Self {
        Self::Keep
    }
}
