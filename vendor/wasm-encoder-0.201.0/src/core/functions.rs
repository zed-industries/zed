use crate::{encode_section, Encode, Section, SectionId};

/// An encoder for the function section of WebAssembly modules.
///
/// # Example
///
/// ```
/// use wasm_encoder::{Module, FunctionSection, ValType};
///
/// let mut functions = FunctionSection::new();
/// let type_index = 0;
/// functions.function(type_index);
///
/// let mut module = Module::new();
/// module.section(&functions);
///
/// // Note: this will generate an invalid module because we didn't generate a
/// // code section containing the function body. See the documentation for
/// // `CodeSection` for details.
///
/// let bytes = module.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct FunctionSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl FunctionSection {
    /// Construct a new module function section encoder.
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

    /// Define a function in a module's function section.
    pub fn function(&mut self, type_index: u32) -> &mut Self {
        type_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }
}

impl Encode for FunctionSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl Section for FunctionSection {
    fn id(&self) -> u8 {
        SectionId::Function.into()
    }
}
