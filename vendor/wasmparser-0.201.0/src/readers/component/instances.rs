use crate::limits::{MAX_WASM_INSTANTIATION_ARGS, MAX_WASM_INSTANTIATION_EXPORTS};
use crate::{
    BinaryReader, ComponentExport, ComponentExternalKind, Export, FromReader, Result,
    SectionLimited,
};

/// Represents the kind of an instantiation argument for a core instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InstantiationArgKind {
    /// The instantiation argument is a core instance.
    Instance,
}

/// Represents an argument to instantiating a WebAssembly module.
#[derive(Debug, Clone)]
pub struct InstantiationArg<'a> {
    /// The name of the module argument.
    pub name: &'a str,
    /// The kind of the module argument.
    pub kind: InstantiationArgKind,
    /// The index of the argument item.
    pub index: u32,
}

/// Represents an instance of a WebAssembly module.
#[derive(Debug, Clone)]
pub enum Instance<'a> {
    /// The instance is from instantiating a WebAssembly module.
    Instantiate {
        /// The module index.
        module_index: u32,
        /// The module's instantiation arguments.
        args: Box<[InstantiationArg<'a>]>,
    },
    /// The instance is a from exporting local items.
    FromExports(Box<[Export<'a>]>),
}

/// A reader for the core instance section of a WebAssembly component.
///
/// # Examples
///
/// ```
/// use wasmparser::InstanceSectionReader;
/// # let data: &[u8] = &[0x01, 0x00, 0x00, 0x01, 0x03, b'f', b'o', b'o', 0x12, 0x00];
/// let mut reader = InstanceSectionReader::new(data, 0).unwrap();
/// for inst in reader {
///     println!("Instance {:?}", inst.expect("instance"));
/// }
/// ```
pub type InstanceSectionReader<'a> = SectionLimited<'a, Instance<'a>>;

impl<'a> FromReader<'a> for Instance<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(match reader.read_u8()? {
            0x00 => Instance::Instantiate {
                module_index: reader.read_var_u32()?,
                args: reader
                    .read_iter(MAX_WASM_INSTANTIATION_ARGS, "core instantiation arguments")?
                    .collect::<Result<_>>()?,
            },
            0x01 => Instance::FromExports(
                reader
                    .read_iter(MAX_WASM_INSTANTIATION_ARGS, "core instantiation arguments")?
                    .collect::<Result<_>>()?,
            ),
            x => return reader.invalid_leading_byte(x, "core instance"),
        })
    }
}

impl<'a> FromReader<'a> for InstantiationArg<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(InstantiationArg {
            name: reader.read()?,
            kind: reader.read()?,
            index: reader.read()?,
        })
    }
}

impl<'a> FromReader<'a> for InstantiationArgKind {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(match reader.read_u8()? {
            0x12 => InstantiationArgKind::Instance,
            x => return reader.invalid_leading_byte(x, "instantiation arg kind"),
        })
    }
}

/// Represents an argument to instantiating a WebAssembly component.
#[derive(Debug, Clone)]
pub struct ComponentInstantiationArg<'a> {
    /// The name of the component argument.
    pub name: &'a str,
    /// The kind of the component argument.
    pub kind: ComponentExternalKind,
    /// The index of the argument item.
    pub index: u32,
}

/// Represents an instance in a WebAssembly component.
#[derive(Debug, Clone)]
pub enum ComponentInstance<'a> {
    /// The instance is from instantiating a WebAssembly component.
    Instantiate {
        /// The component index.
        component_index: u32,
        /// The component's instantiation arguments.
        args: Box<[ComponentInstantiationArg<'a>]>,
    },
    /// The instance is a from exporting local items.
    FromExports(Box<[ComponentExport<'a>]>),
}

/// A reader for the component instance section of a WebAssembly component.
///
/// # Examples
///
/// ```
/// use wasmparser::ComponentInstanceSectionReader;
/// # let data: &[u8] = &[0x01, 0x00, 0x00, 0x01, 0x03, b'f', b'o', b'o', 0x01, 0x00];
/// let mut reader = ComponentInstanceSectionReader::new(data, 0).unwrap();
/// for inst in reader {
///     println!("Instance {:?}", inst.expect("instance"));
/// }
/// ```
pub type ComponentInstanceSectionReader<'a> = SectionLimited<'a, ComponentInstance<'a>>;

impl<'a> FromReader<'a> for ComponentInstance<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(match reader.read_u8()? {
            0x00 => ComponentInstance::Instantiate {
                component_index: reader.read_var_u32()?,
                args: reader
                    .read_iter(MAX_WASM_INSTANTIATION_ARGS, "instantiation arguments")?
                    .collect::<Result<_>>()?,
            },
            0x01 => ComponentInstance::FromExports(
                (0..reader.read_size(MAX_WASM_INSTANTIATION_EXPORTS, "instantiation exports")?)
                    .map(|_| {
                        Ok(ComponentExport {
                            name: reader.read()?,
                            kind: reader.read()?,
                            index: reader.read()?,
                            ty: None,
                        })
                    })
                    .collect::<Result<_>>()?,
            ),
            x => return reader.invalid_leading_byte(x, "instance"),
        })
    }
}
impl<'a> FromReader<'a> for ComponentInstantiationArg<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(ComponentInstantiationArg {
            name: reader.read()?,
            kind: reader.read()?,
            index: reader.read()?,
        })
    }
}
