mod aliases;
mod builder;
mod canonicals;
mod components;
mod exports;
mod imports;
mod instances;
mod modules;
mod names;
mod start;
mod types;

pub use self::aliases::*;
pub use self::builder::*;
pub use self::canonicals::*;
pub use self::components::*;
pub use self::exports::*;
pub use self::imports::*;
pub use self::instances::*;
pub use self::modules::*;
pub use self::names::*;
pub use self::start::*;
pub use self::types::*;

use crate::{CustomSection, Encode, ProducersSection, RawCustomSection};

// Core sorts extended by the component model
const CORE_TYPE_SORT: u8 = 0x10;
const CORE_MODULE_SORT: u8 = 0x11;
const CORE_INSTANCE_SORT: u8 = 0x12;

const CORE_SORT: u8 = 0x00;
const FUNCTION_SORT: u8 = 0x01;
const VALUE_SORT: u8 = 0x02;
const TYPE_SORT: u8 = 0x03;
const COMPONENT_SORT: u8 = 0x04;
const INSTANCE_SORT: u8 = 0x05;

/// A WebAssembly component section.
///
/// Various builders defined in this crate already implement this trait, but you
/// can also implement it yourself for your own custom section builders, or use
/// `RawSection` to use a bunch of raw bytes as a section.
pub trait ComponentSection: Encode {
    /// Gets the section identifier for this section.
    fn id(&self) -> u8;

    /// Appends this section to the specified destination list of bytes.
    fn append_to_component(&self, dst: &mut Vec<u8>) {
        dst.push(self.id());
        self.encode(dst);
    }
}

/// Known section identifiers of WebAssembly components.
///
/// These sections are supported by the component model proposal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum ComponentSectionId {
    /// The section is a core custom section.
    CoreCustom = 0,
    /// The section is a core module section.
    CoreModule = 1,
    /// The section is a core instance section.
    CoreInstance = 2,
    /// The section is a core type section.
    CoreType = 3,
    /// The section is a component section.
    Component = 4,
    /// The section is an instance section.
    Instance = 5,
    /// The section is an alias section.
    Alias = 6,
    /// The section is a type section.
    Type = 7,
    /// The section is a canonical function section.
    CanonicalFunction = 8,
    /// The section is a start section.
    Start = 9,
    /// The section is an import section.
    Import = 10,
    /// The section is an export section.
    Export = 11,
}

impl From<ComponentSectionId> for u8 {
    #[inline]
    fn from(id: ComponentSectionId) -> u8 {
        id as u8
    }
}

impl Encode for ComponentSectionId {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.push(*self as u8);
    }
}

/// Represents a WebAssembly component that is being encoded.
///
/// Unlike core WebAssembly modules, the sections of a component
/// may appear in any order and may be repeated.
///
/// Components may also added as a section to other components.
#[derive(Clone, Debug)]
pub struct Component {
    bytes: Vec<u8>,
}

impl Component {
    /// The 8-byte header at the beginning of all components.
    #[rustfmt::skip]
    pub const HEADER: [u8; 8] = [
        // Magic
        0x00, 0x61, 0x73, 0x6D,
        // Version
        0x0d, 0x00, 0x01, 0x00,
    ];

    /// Begin writing a new `Component`.
    pub fn new() -> Self {
        Self {
            bytes: Self::HEADER.to_vec(),
        }
    }

    /// Finish writing this component and extract ownership of the encoded bytes.
    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }

    /// Write a section to this component.
    pub fn section(&mut self, section: &impl ComponentSection) -> &mut Self {
        self.bytes.push(section.id());
        section.encode(&mut self.bytes);
        self
    }

    /// View the encoded bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }
}

impl Default for Component {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentSection for CustomSection<'_> {
    fn id(&self) -> u8 {
        ComponentSectionId::CoreCustom.into()
    }
}

impl ComponentSection for RawCustomSection<'_> {
    fn id(&self) -> u8 {
        ComponentSectionId::CoreCustom.into()
    }
}

impl ComponentSection for ProducersSection {
    fn id(&self) -> u8 {
        ComponentSectionId::CoreCustom.into()
    }
}
