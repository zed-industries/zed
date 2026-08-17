use super::CORE_INSTANCE_SORT;
use crate::{
    encode_section, ComponentExportKind, ComponentSection, ComponentSectionId, Encode, ExportKind,
};

/// Represents an argument to a module instantiation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModuleArg {
    /// The argument is an instance.
    Instance(u32),
}

impl Encode for ModuleArg {
    fn encode(&self, sink: &mut Vec<u8>) {
        let (sort, idx) = match self {
            Self::Instance(idx) => (CORE_INSTANCE_SORT, *idx),
        };
        sink.push(sort);
        idx.encode(sink);
    }
}

/// An encoder for the core instance section of WebAssembly components.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Component, InstanceSection, ExportKind, ModuleArg};
///
/// let mut instances = InstanceSection::new();
/// instances.export_items([("foo", ExportKind::Func, 0)]);
/// instances.instantiate(1, [("foo", ModuleArg::Instance(0))]);
///
/// let mut component = Component::new();
/// component.section(&instances);
///
/// let bytes = component.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct InstanceSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl InstanceSection {
    /// Create a new core instance section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of instances in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define an instance by instantiating a core module.
    pub fn instantiate<A, S>(&mut self, module_index: u32, args: A) -> &mut Self
    where
        A: IntoIterator<Item = (S, ModuleArg)>,
        A::IntoIter: ExactSizeIterator,
        S: AsRef<str>,
    {
        let args = args.into_iter();
        self.bytes.push(0x00);
        module_index.encode(&mut self.bytes);
        args.len().encode(&mut self.bytes);
        for (name, arg) in args {
            name.as_ref().encode(&mut self.bytes);
            arg.encode(&mut self.bytes);
        }
        self.num_added += 1;
        self
    }

    /// Define an instance by exporting core WebAssembly items.
    pub fn export_items<E, S>(&mut self, exports: E) -> &mut Self
    where
        E: IntoIterator<Item = (S, ExportKind, u32)>,
        E::IntoIter: ExactSizeIterator,
        S: AsRef<str>,
    {
        let exports = exports.into_iter();
        self.bytes.push(0x01);
        exports.len().encode(&mut self.bytes);
        for (name, kind, index) in exports {
            name.as_ref().encode(&mut self.bytes);
            kind.encode(&mut self.bytes);
            index.encode(&mut self.bytes);
        }
        self.num_added += 1;
        self
    }
}

impl Encode for InstanceSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl ComponentSection for InstanceSection {
    fn id(&self) -> u8 {
        ComponentSectionId::CoreInstance.into()
    }
}

/// An encoder for the instance section of WebAssembly components.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Component, ComponentInstanceSection, ComponentExportKind};
///
/// let mut instances = ComponentInstanceSection::new();
/// instances.export_items([("foo", ComponentExportKind::Func, 0)]);
/// instances.instantiate(1, [("foo", ComponentExportKind::Instance, 0)]);
///
/// let mut component = Component::new();
/// component.section(&instances);
///
/// let bytes = component.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct ComponentInstanceSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl ComponentInstanceSection {
    /// Create a new instance section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of instances in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define an instance by instantiating a component.
    pub fn instantiate<A, S>(&mut self, component_index: u32, args: A) -> &mut Self
    where
        A: IntoIterator<Item = (S, ComponentExportKind, u32)>,
        A::IntoIter: ExactSizeIterator,
        S: AsRef<str>,
    {
        let args = args.into_iter();
        self.bytes.push(0x00);
        component_index.encode(&mut self.bytes);
        args.len().encode(&mut self.bytes);
        for (name, kind, index) in args {
            name.as_ref().encode(&mut self.bytes);
            kind.encode(&mut self.bytes);
            index.encode(&mut self.bytes);
        }
        self.num_added += 1;
        self
    }

    /// Define an instance by exporting items.
    pub fn export_items<'a, E>(&mut self, exports: E) -> &mut Self
    where
        E: IntoIterator<Item = (&'a str, ComponentExportKind, u32)>,
        E::IntoIter: ExactSizeIterator,
    {
        let exports = exports.into_iter();
        self.bytes.push(0x01);
        exports.len().encode(&mut self.bytes);
        for (name, kind, index) in exports {
            crate::push_extern_name_byte(&mut self.bytes, name);
            name.encode(&mut self.bytes);
            kind.encode(&mut self.bytes);
            index.encode(&mut self.bytes);
        }
        self.num_added += 1;
        self
    }
}

impl Encode for ComponentInstanceSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl ComponentSection for ComponentInstanceSection {
    fn id(&self) -> u8 {
        ComponentSectionId::Instance.into()
    }
}
