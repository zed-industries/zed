mod code;
mod custom;
mod data;
mod dump;
mod elements;
mod exports;
mod functions;
mod globals;
mod imports;
mod linking;
mod memories;
mod names;
mod producers;
mod start;
mod tables;
mod tags;
mod types;

pub use code::*;
pub use custom::*;
pub use data::*;
pub use dump::*;
pub use elements::*;
pub use exports::*;
pub use functions::*;
pub use globals::*;
pub use imports::*;
pub use linking::*;
pub use memories::*;
pub use names::*;
pub use producers::*;
pub use start::*;
pub use tables::*;
pub use tags::*;
pub use types::*;

use crate::Encode;

pub(crate) const CORE_FUNCTION_SORT: u8 = 0x00;
pub(crate) const CORE_TABLE_SORT: u8 = 0x01;
pub(crate) const CORE_MEMORY_SORT: u8 = 0x02;
pub(crate) const CORE_GLOBAL_SORT: u8 = 0x03;
pub(crate) const CORE_TAG_SORT: u8 = 0x04;

/// A WebAssembly module section.
///
/// Various builders defined in this crate already implement this trait, but you
/// can also implement it yourself for your own custom section builders, or use
/// `RawSection` to use a bunch of raw bytes as a section.
pub trait Section: Encode {
    /// Gets the section identifier for this section.
    fn id(&self) -> u8;

    /// Appends this section to the specified destination list of bytes.
    fn append_to(&self, dst: &mut Vec<u8>) {
        dst.push(self.id());
        self.encode(dst);
    }
}

/// Known section identifiers of WebAssembly modules.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum SectionId {
    /// The custom section.
    Custom = 0,
    /// The type section.
    Type = 1,
    /// The import section.
    Import = 2,
    /// The function section.
    Function = 3,
    /// The table section.
    Table = 4,
    /// The memory section.
    Memory = 5,
    /// The global section.
    Global = 6,
    /// The export section.
    Export = 7,
    /// The start section.
    Start = 8,
    /// The element section.
    Element = 9,
    /// The code section.
    Code = 10,
    /// The data section.
    Data = 11,
    /// The data count section.
    DataCount = 12,
    /// The tag section.
    ///
    /// This section is supported by the exception handling proposal.
    Tag = 13,
}

impl From<SectionId> for u8 {
    #[inline]
    fn from(id: SectionId) -> u8 {
        id as u8
    }
}

impl Encode for SectionId {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.push(*self as u8);
    }
}

/// Represents a WebAssembly component that is being encoded.
///
/// Sections within a WebAssembly module are encoded in a specific order.
///
/// Modules may also added as a section to a WebAssembly component.
#[derive(Clone, Debug)]
pub struct Module {
    pub(crate) bytes: Vec<u8>,
}

impl Module {
    /// The 8-byte header at the beginning of all core wasm modules.
    #[rustfmt::skip]
    pub const HEADER: [u8; 8] = [
        // Magic
        0x00, 0x61, 0x73, 0x6D,
        // Version
        0x01, 0x00, 0x00, 0x00,
    ];

    /// Begin writing a new `Module`.
    #[rustfmt::skip]
    pub fn new() -> Self {
        Module {
            bytes: Self::HEADER.to_vec(),
        }
    }

    /// Write a section into this module.
    ///
    /// It is your responsibility to define the sections in the [proper
    /// order](https://webassembly.github.io/spec/core/binary/modules.html#binary-module),
    /// and to ensure that each kind of section (other than custom sections) is
    /// only defined once. While this is a potential footgun, it also allows you
    /// to use this crate to easily construct test cases for bad Wasm module
    /// encodings.
    pub fn section(&mut self, section: &impl Section) -> &mut Self {
        self.bytes.push(section.id());
        section.encode(&mut self.bytes);
        self
    }

    /// Get the encoded Wasm module as a slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Finish writing this Wasm module and extract ownership of the encoded
    /// bytes.
    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

impl Default for Module {
    fn default() -> Self {
        Self::new()
    }
}
