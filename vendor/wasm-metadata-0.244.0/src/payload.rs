use std::ops::Range;

use anyhow::Result;
use serde_derive::Serialize;
use wasmparser::{KnownCustom, Parser, Payload::*};

use crate::{
    Authors, ComponentNames, Description, Homepage, Licenses, Metadata, ModuleNames, Producers,
    Revision, Source,
};

/// Data representing either a Wasm Component or module
///
/// Each payload has additional [`Metadata`] associated with it,
/// but if it's a Component it may have also additional `Payloads` associated
/// with it.
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Payload {
    /// A representation of a Wasm Component
    Component {
        /// The metadata associated with the Component
        metadata: Metadata,
        /// The metadata of nested Components or Modules
        children: Vec<Payload>,
    },
    /// A representation of a Wasm Module
    Module(Metadata),
}

impl Payload {
    /// Parse metadata from a WebAssembly binary. Supports both core WebAssembly modules, and
    /// WebAssembly components.
    pub fn from_binary(input: &[u8]) -> Result<Self> {
        let mut output = Vec::new();

        for payload in Parser::new(0).parse_all(&input) {
            match payload? {
                Version { encoding, .. } => {
                    if output.is_empty() {
                        match encoding {
                            wasmparser::Encoding::Module => {
                                output.push(Self::empty_module(0..input.len()))
                            }
                            wasmparser::Encoding::Component => {
                                output.push(Self::empty_component(0..input.len()))
                            }
                        }
                    }
                }
                ModuleSection {
                    unchecked_range: range,
                    ..
                } => output.push(Self::empty_module(range)),
                ComponentSection {
                    unchecked_range: range,
                    ..
                } => output.push(Self::empty_component(range)),
                End { .. } => {
                    let finished = output.pop().expect("non-empty metadata stack");
                    if output.is_empty() {
                        return Ok(finished);
                    } else {
                        output.last_mut().unwrap().push_child(finished);
                    }
                }
                CustomSection(c) => match c.as_known() {
                    KnownCustom::Name(_) => {
                        let names = ModuleNames::from_bytes(c.data(), c.data_offset())?;
                        if let Some(name) = names.get_name() {
                            output
                                .last_mut()
                                .expect("non-empty metadata stack")
                                .metadata_mut()
                                .name = Some(name.clone());
                        }
                    }
                    KnownCustom::ComponentName(_) => {
                        let names = ComponentNames::from_bytes(c.data(), c.data_offset())?;
                        if let Some(name) = names.get_name() {
                            output
                                .last_mut()
                                .expect("non-empty metadata stack")
                                .metadata_mut()
                                .name = Some(name.clone());
                        }
                    }
                    KnownCustom::Producers(_) => {
                        let producers = Producers::from_bytes(c.data(), c.data_offset())?;
                        output
                            .last_mut()
                            .expect("non-empty metadata stack")
                            .metadata_mut()
                            .producers = Some(producers);
                    }
                    #[cfg(feature = "oci")]
                    KnownCustom::Unknown if c.name() == "authors" => {
                        let a = Authors::parse_custom_section(&c)?;
                        let Metadata { authors, .. } = output
                            .last_mut()
                            .expect("non-empty metadata stack")
                            .metadata_mut();
                        *authors = Some(a);
                    }
                    #[cfg(feature = "oci")]
                    KnownCustom::Unknown if c.name() == "description" => {
                        let a = Description::parse_custom_section(&c)?;
                        let Metadata { description, .. } = output
                            .last_mut()
                            .expect("non-empty metadata stack")
                            .metadata_mut();
                        *description = Some(a);
                    }
                    #[cfg(feature = "oci")]
                    KnownCustom::Unknown if c.name() == "licenses" => {
                        let a = Licenses::parse_custom_section(&c)?;
                        let Metadata { licenses, .. } = output
                            .last_mut()
                            .expect("non-empty metadata stack")
                            .metadata_mut();
                        *licenses = Some(a);
                    }
                    #[cfg(feature = "oci")]
                    KnownCustom::Unknown if c.name() == "source" => {
                        let a = Source::parse_custom_section(&c)?;
                        let Metadata { source, .. } = output
                            .last_mut()
                            .expect("non-empty metadata stack")
                            .metadata_mut();
                        *source = Some(a);
                    }
                    #[cfg(feature = "oci")]
                    KnownCustom::Unknown if c.name() == "homepage" => {
                        let a = Homepage::parse_custom_section(&c)?;
                        let Metadata { homepage, .. } = output
                            .last_mut()
                            .expect("non-empty metadata stack")
                            .metadata_mut();
                        *homepage = Some(a);
                    }
                    #[cfg(feature = "oci")]
                    KnownCustom::Unknown if c.name() == "revision" => {
                        let a = Revision::parse_custom_section(&c)?;
                        let Metadata { revision, .. } = output
                            .last_mut()
                            .expect("non-empty metadata stack")
                            .metadata_mut();
                        *revision = Some(a);
                    }
                    #[cfg(feature = "oci")]
                    KnownCustom::Unknown if c.name() == "version" => {
                        let a = crate::Version::parse_custom_section(&c)?;
                        let Metadata { version, .. } = output
                            .last_mut()
                            .expect("non-empty metadata stack")
                            .metadata_mut();
                        *version = Some(a);
                    }
                    #[cfg(feature = "oci")]
                    KnownCustom::Unknown if c.name() == ".dep-v0" => {
                        let a = crate::Dependencies::parse_custom_section(&c)?;
                        let Metadata { dependencies, .. } = output
                            .last_mut()
                            .expect("non-empty metadata stack")
                            .metadata_mut();
                        *dependencies = Some(a);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Err(anyhow::anyhow!(
            "malformed wasm binary, should have reached end"
        ))
    }

    /// Get a reference te the metadata
    pub fn metadata(&self) -> &Metadata {
        match self {
            Payload::Component { metadata, .. } => metadata,
            Payload::Module(metadata) => metadata,
        }
    }

    /// Get a mutable reference te the metadata
    pub fn metadata_mut(&mut self) -> &mut Metadata {
        match self {
            Payload::Component { metadata, .. } => metadata,
            Payload::Module(metadata) => metadata,
        }
    }

    fn empty_component(range: Range<usize>) -> Self {
        let mut this = Self::Component {
            metadata: Metadata::default(),
            children: vec![],
        };
        this.metadata_mut().range = range;
        this
    }

    fn empty_module(range: Range<usize>) -> Self {
        let mut this = Self::Module(Metadata::default());
        this.metadata_mut().range = range;
        this
    }

    fn push_child(&mut self, child: Self) {
        match self {
            Self::Module { .. } => panic!("module shouldnt have children"),
            Self::Component { children, .. } => children.push(child),
        }
    }
}
