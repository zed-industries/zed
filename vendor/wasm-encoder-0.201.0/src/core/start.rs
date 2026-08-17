use crate::{encoding_size, Encode, Section, SectionId};

/// An encoder for the start section of WebAssembly modules.
///
/// # Example
///
/// Note: this doesn't actually define the function at index 0, its type, or its
/// code body, so the resulting Wasm module will be invalid. See `TypeSection`,
/// `FunctionSection`, and `CodeSection` for details on how to generate those
/// things.
///
/// ```
/// use wasm_encoder::{Module, StartSection};
///
/// let start = StartSection { function_index: 0 };
///
/// let mut module = Module::new();
/// module.section(&start);
///
/// let wasm_bytes = module.finish();
/// ```
#[derive(Clone, Copy, Debug)]
pub struct StartSection {
    /// The index of the start function.
    pub function_index: u32,
}

impl Encode for StartSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encoding_size(self.function_index).encode(sink);
        self.function_index.encode(sink);
    }
}

impl Section for StartSection {
    fn id(&self) -> u8 {
        SectionId::Start.into()
    }
}
