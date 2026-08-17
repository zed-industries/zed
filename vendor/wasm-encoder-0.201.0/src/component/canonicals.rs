use crate::{encode_section, ComponentSection, ComponentSectionId, Encode};

/// Represents options for canonical function definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalOption {
    /// The string types in the function signature are UTF-8 encoded.
    UTF8,
    /// The string types in the function signature are UTF-16 encoded.
    UTF16,
    /// The string types in the function signature are compact UTF-16 encoded.
    CompactUTF16,
    /// The memory to use if the lifting or lowering of a function requires memory access.
    ///
    /// The value is an index to a core memory.
    Memory(u32),
    /// The realloc function to use if the lifting or lowering of a function requires memory
    /// allocation.
    ///
    /// The value is an index to a core function of type `(func (param i32 i32 i32 i32) (result i32))`.
    Realloc(u32),
    /// The post-return function to use if the lifting of a function requires
    /// cleanup after the function returns.
    PostReturn(u32),
}

impl Encode for CanonicalOption {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            Self::UTF8 => sink.push(0x00),
            Self::UTF16 => sink.push(0x01),
            Self::CompactUTF16 => sink.push(0x02),
            Self::Memory(idx) => {
                sink.push(0x03);
                idx.encode(sink);
            }
            Self::Realloc(idx) => {
                sink.push(0x04);
                idx.encode(sink);
            }
            Self::PostReturn(idx) => {
                sink.push(0x05);
                idx.encode(sink);
            }
        }
    }
}

/// An encoder for the canonical function section of WebAssembly components.
///
/// # Example
///
/// ```
/// use wasm_encoder::{Component, CanonicalFunctionSection, CanonicalOption};
///
/// let mut functions = CanonicalFunctionSection::new();
/// functions.lift(0, 0, [CanonicalOption::UTF8]);
///
/// let mut component = Component::new();
/// component.section(&functions);
///
/// let bytes = component.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct CanonicalFunctionSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl CanonicalFunctionSection {
    /// Construct a new component function section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of functions in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define a function that will lift a core WebAssembly function to the canonical ABI.
    pub fn lift<O>(&mut self, core_func_index: u32, type_index: u32, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        let options = options.into_iter();
        self.bytes.push(0x00);
        self.bytes.push(0x00);
        core_func_index.encode(&mut self.bytes);
        options.len().encode(&mut self.bytes);
        for option in options {
            option.encode(&mut self.bytes);
        }
        type_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Define a function that will lower a canonical ABI function to a core WebAssembly function.
    pub fn lower<O>(&mut self, func_index: u32, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        let options = options.into_iter();
        self.bytes.push(0x01);
        self.bytes.push(0x00);
        func_index.encode(&mut self.bytes);
        options.len().encode(&mut self.bytes);
        for option in options {
            option.encode(&mut self.bytes);
        }
        self.num_added += 1;
        self
    }

    /// Defines a function which will create an owned handle to the resource
    /// specified by `ty_index`.
    pub fn resource_new(&mut self, ty_index: u32) -> &mut Self {
        self.bytes.push(0x02);
        ty_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function which will drop the specified type of handle.
    pub fn resource_drop(&mut self, ty_index: u32) -> &mut Self {
        self.bytes.push(0x03);
        ty_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function which will return the representation of the specified
    /// resource type.
    pub fn resource_rep(&mut self, ty_index: u32) -> &mut Self {
        self.bytes.push(0x04);
        ty_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }
}

impl Encode for CanonicalFunctionSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl ComponentSection for CanonicalFunctionSection {
    fn id(&self) -> u8 {
        ComponentSectionId::CanonicalFunction.into()
    }
}
