use crate::{Encode, Section, SectionId, encode_section};
use alloc::vec::Vec;

/// An encoder for the memory section.
///
/// Memory sections are only supported for modules.
///
/// # Example
///
/// ```
/// use wasm_encoder::{Module, MemorySection, MemoryType};
///
/// let mut memories = MemorySection::new();
/// memories.memory(MemoryType {
///     minimum: 1,
///     maximum: None,
///     memory64: false,
///     shared: false,
///     page_size_log2: None,
/// });
///
/// let mut module = Module::new();
/// module.section(&memories);
///
/// let wasm_bytes = module.finish();
/// ```
#[derive(Clone, Default, Debug)]
pub struct MemorySection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl MemorySection {
    /// Create a new memory section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of memories in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define a memory.
    pub fn memory(&mut self, memory_type: MemoryType) -> &mut Self {
        memory_type.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }
}

impl Encode for MemorySection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl Section for MemorySection {
    fn id(&self) -> u8 {
        SectionId::Memory.into()
    }
}

/// A memory's type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemoryType {
    /// Minimum size, in pages, of this memory
    pub minimum: u64,
    /// Maximum size, in pages, of this memory
    pub maximum: Option<u64>,
    /// Whether or not this is a 64-bit memory.
    pub memory64: bool,
    /// Whether or not this memory is shared.
    pub shared: bool,
    /// The log base 2 of a custom page size for this memory.
    ///
    /// The default page size for Wasm memories is 64KiB, i.e. 2<sup>16</sup> or
    /// `65536`.
    ///
    /// After the introduction of [the custom-page-sizes
    /// proposal](https://github.com/WebAssembly/custom-page-sizes), Wasm can
    /// customize the page size. It must be a power of two that is less than or
    /// equal to 64KiB. Attempting to encode an invalid page size may panic.
    pub page_size_log2: Option<u32>,
}

impl Encode for MemoryType {
    fn encode(&self, sink: &mut Vec<u8>) {
        let mut flags = 0;
        if self.maximum.is_some() {
            flags |= 0b0001;
        }
        if self.shared {
            flags |= 0b0010;
        }
        if self.memory64 {
            flags |= 0b0100;
        }
        if self.page_size_log2.is_some() {
            flags |= 0b1000;
        }
        sink.push(flags);
        self.minimum.encode(sink);
        if let Some(max) = self.maximum {
            max.encode(sink);
        }
        if let Some(p) = self.page_size_log2 {
            p.encode(sink);
        }
    }
}
