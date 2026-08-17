use serde_derive::Serialize;
use std::ops::Range;

use crate::{
    Authors, Dependencies, Description, Homepage, Licenses, Producers, Revision, Source, Version,
};

/// Metadata associated with a Wasm Component or Module
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct Metadata {
    /// The component name, if any. Found in the component-name section.
    pub name: Option<String>,
    /// The component's producers section, if any.
    pub producers: Option<Producers>,
    /// The component's authors section, if any.
    pub authors: Option<Authors>,
    /// Human-readable description of the binary
    pub description: Option<Description>,
    /// License(s) under which contained software is distributed as an SPDX License Expression.
    pub licenses: Option<Licenses>,
    /// URL to get source code for building the image
    pub source: Option<Source>,
    /// URL to find more information on the binary
    pub homepage: Option<Homepage>,
    /// Source control revision identifier for the packaged software.
    pub revision: Option<Revision>,
    /// Version of the packaged software
    pub version: Option<Version>,
    /// Byte range of the module in the parent binary
    pub range: Range<usize>,
    /// Dependencies of the component
    pub dependencies: Option<Dependencies>,
}
