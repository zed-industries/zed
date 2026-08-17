use crate::{encode_section, Encode, Section, SectionId};

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
}

impl Encode for MemoryType {
    fn encode(&self, sink: &mut Vec<u8>) {
        let mut flags = 0;
        if self.maximum.is_some() {
            flags |= 0b001;
        }
        if self.shared {
            flags |= 0b010;
        }
        if self.memory64 {
            flags |= 0b100;
        }

        sink.push(flags);
        self.minimum.encode(sink);
        if let Some(max) = self.maximum {
            max.encode(sink);
        }
    }
}

#[cfg(feature = "wasmparser")]
impl From<wasmparser::MemoryType> for MemoryType {
    fn from(memory_ty: wasmparser::MemoryType) -> Self {
        MemoryType {
            minimum: memory_ty.initial,
            maximum: memory_ty.maximum,
            memory64: memory_ty.memory64,
            shared: memory_ty.shared,
        }
    }
}
