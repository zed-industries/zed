use crate::{ComponentSection, ComponentSectionId, Encode, Module};

/// An encoder for the module section of WebAssembly components.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Module, Component, ModuleSection};
///
/// let mut module = Module::new();
/// let mut component = Component::new();
/// component.section(&ModuleSection(&module));
///
/// let bytes = component.finish();
/// ```
#[derive(Clone, Debug)]
pub struct ModuleSection<'a>(pub &'a Module);

impl Encode for ModuleSection<'_> {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.bytes.encode(sink);
    }
}

impl ComponentSection for ModuleSection<'_> {
    fn id(&self) -> u8 {
        ComponentSectionId::CoreModule.into()
    }
}
