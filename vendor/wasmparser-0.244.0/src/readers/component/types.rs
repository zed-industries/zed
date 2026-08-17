use crate::limits::*;
use crate::prelude::*;
use crate::{
    BinaryReader, ComponentAlias, ComponentExportName, ComponentImport, ComponentTypeRef,
    FromReader, Import, RecGroup, Result, SectionLimited, TypeRef, ValType,
};
use core::fmt;

/// Represents the kind of an outer core alias in a WebAssembly component.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OuterAliasKind {
    /// The alias is to a core type.
    Type,
}

/// Represents a core type in a WebAssembly component.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CoreType<'a> {
    /// The type is for a core subtype.
    Rec(RecGroup),
    /// The type is for a core module.
    Module(Box<[ModuleTypeDeclaration<'a>]>),
}

impl<'a> FromReader<'a> for CoreType<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        // For the time being, this special logic handles an ambiguous encoding
        // in the component model: the `0x50` opcode represents both a core
        // module type as well as a GC non-final `sub` type. To avoid this, the
        // component model specification requires us to prefix a non-final `sub`
        // type with `0x00` when it is used as a top-level core type of a
        // component. Eventually (prior to the component model's v1.0 release),
        // a module type will get a new opcode and this special logic can go
        // away.
        Ok(match reader.peek()? {
            0x00 => {
                reader.read_u8()?;
                let x = reader.peek()?;
                if x != 0x50 {
                    return reader.invalid_leading_byte(x, "non-final sub type");
                }
                CoreType::Rec(reader.read()?)
            }
            0x50 => {
                reader.read_u8()?;
                CoreType::Module(
                    reader
                        .read_iter(MAX_WASM_MODULE_TYPE_DECLS, "module type declaration")?
                        .collect::<Result<_>>()?,
                )
            }
            _ => CoreType::Rec(reader.read()?),
        })
    }
}

/// Represents a module type declaration in a WebAssembly component.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ModuleTypeDeclaration<'a> {
    /// The module type definition is for a type.
    Type(RecGroup),
    /// The module type definition is for an export.
    Export {
        /// The name of the exported item.
        name: &'a str,
        /// The type reference of the export.
        ty: TypeRef,
    },
    /// The module type declaration is for an outer alias.
    OuterAlias {
        /// The alias kind.
        kind: OuterAliasKind,
        /// The outward count, starting at zero for the current type.
        count: u32,
        /// The index of the item within the outer type.
        index: u32,
    },
    /// The module type definition is for an import.
    Import(Import<'a>),
}

impl<'a> FromReader<'a> for ModuleTypeDeclaration<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(match reader.read_u8()? {
            0x00 => ModuleTypeDeclaration::Import(reader.read()?),
            0x01 => ModuleTypeDeclaration::Type(reader.read()?),
            0x02 => {
                let kind = match reader.read_u8()? {
                    0x10 => OuterAliasKind::Type,
                    x => {
                        return reader.invalid_leading_byte(x, "outer alias kind");
                    }
                };
                match reader.read_u8()? {
                    0x01 => ModuleTypeDeclaration::OuterAlias {
                        kind,
                        count: reader.read()?,
                        index: reader.read()?,
                    },
                    x => {
                        return reader.invalid_leading_byte(x, "outer alias target");
                    }
                }
            }
            0x03 => ModuleTypeDeclaration::Export {
                name: reader.read()?,
                ty: reader.read()?,
            },
            x => return reader.invalid_leading_byte(x, "type definition"),
        })
    }
}

/// A reader for the core type section of a WebAssembly component.
///
/// # Examples
/// ```
/// use wasmparser::{CoreTypeSectionReader, BinaryReader};
/// # let data: &[u8] = &[0x01, 0x60, 0x00, 0x00];
/// let reader = BinaryReader::new(data, 0);
/// let mut reader = CoreTypeSectionReader::new(reader).unwrap();
/// for ty in reader {
///     println!("Type {:?}", ty.expect("type"));
/// }
/// ```
pub type CoreTypeSectionReader<'a> = SectionLimited<'a, CoreType<'a>>;

/// Represents a value type in a WebAssembly component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentValType {
    /// The value type is a primitive type.
    Primitive(PrimitiveValType),
    /// The value type is a reference to a defined type.
    Type(u32),
}

impl<'a> FromReader<'a> for ComponentValType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        if let Some(ty) = PrimitiveValType::from_byte(reader.peek()?) {
            reader.read_u8()?;
            return Ok(ComponentValType::Primitive(ty));
        }

        Ok(ComponentValType::Type(reader.read_var_s33()? as u32))
    }
}

impl<'a> FromReader<'a> for Option<ComponentValType> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        match reader.read_u8()? {
            0x0 => Ok(None),
            0x1 => Ok(Some(reader.read()?)),
            x => reader.invalid_leading_byte(x, "optional component value type"),
        }
    }
}

/// Represents a primitive value type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveValType {
    /// The type is a boolean.
    Bool,
    /// The type is a signed 8-bit integer.
    S8,
    /// The type is an unsigned 8-bit integer.
    U8,
    /// The type is a signed 16-bit integer.
    S16,
    /// The type is an unsigned 16-bit integer.
    U16,
    /// The type is a signed 32-bit integer.
    S32,
    /// The type is an unsigned 32-bit integer.
    U32,
    /// The type is a signed 64-bit integer.
    S64,
    /// The type is an unsigned 64-bit integer.
    U64,
    /// The type is a 32-bit floating point number with only one NaN.
    F32,
    /// The type is a 64-bit floating point number with only one NaN.
    F64,
    /// The type is a Unicode character.
    Char,
    /// The type is a string.
    String,
    /// The error-context type. (added with the async proposal for the component
    /// model)
    ErrorContext,
}

impl PrimitiveValType {
    fn from_byte(byte: u8) -> Option<PrimitiveValType> {
        Some(match byte {
            0x7f => PrimitiveValType::Bool,
            0x7e => PrimitiveValType::S8,
            0x7d => PrimitiveValType::U8,
            0x7c => PrimitiveValType::S16,
            0x7b => PrimitiveValType::U16,
            0x7a => PrimitiveValType::S32,
            0x79 => PrimitiveValType::U32,
            0x78 => PrimitiveValType::S64,
            0x77 => PrimitiveValType::U64,
            0x76 => PrimitiveValType::F32,
            0x75 => PrimitiveValType::F64,
            0x74 => PrimitiveValType::Char,
            0x73 => PrimitiveValType::String,
            0x64 => PrimitiveValType::ErrorContext,
            _ => return None,
        })
    }

    #[cfg(feature = "validate")]
    pub(crate) fn contains_ptr(&self) -> bool {
        matches!(self, Self::String)
    }

    /// Determines if primitive value type `a` is a subtype of `b`.
    pub fn is_subtype_of(a: Self, b: Self) -> bool {
        // Note that this intentionally diverges from the upstream specification
        // at this time and only considers exact equality for subtyping
        // relationships.
        //
        // More information can be found in the subtyping implementation for
        // component functions.
        a == b
    }
}

impl fmt::Display for PrimitiveValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PrimitiveValType::*;
        let s = match self {
            Bool => "bool",
            S8 => "s8",
            U8 => "u8",
            S16 => "s16",
            U16 => "u16",
            S32 => "s32",
            U32 => "u32",
            S64 => "s64",
            U64 => "u64",
            F32 => "f32",
            F64 => "f64",
            Char => "char",
            String => "string",
            ErrorContext => "error-context",
        };
        s.fmt(f)
    }
}

/// Represents a type in a WebAssembly component.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ComponentType<'a> {
    /// The type is a component defined type.
    Defined(ComponentDefinedType<'a>),
    /// The type is a function type.
    Func(ComponentFuncType<'a>),
    /// The type is a component type.
    Component(Box<[ComponentTypeDeclaration<'a>]>),
    /// The type is an instance type.
    Instance(Box<[InstanceTypeDeclaration<'a>]>),
    /// The type is a fresh new resource type.
    Resource {
        /// The representation of this resource type in core WebAssembly.
        rep: ValType,
        /// An optionally-specified destructor to use for when this resource is
        /// no longer needed.
        dtor: Option<u32>,
    },
}

impl<'a> FromReader<'a> for ComponentType<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(match reader.read_u8()? {
            0x3f => ComponentType::Resource {
                rep: reader.read()?,
                dtor: match reader.read_u8()? {
                    0x00 => None,
                    0x01 => Some(reader.read()?),
                    b => return reader.invalid_leading_byte(b, "resource destructor"),
                },
            },
            byte @ (0x40 | 0x43) => {
                let params = reader
                    .read_iter(MAX_WASM_FUNCTION_PARAMS, "component function parameters")?
                    .collect::<Result<_>>()?;
                let result = read_resultlist(reader)?;
                ComponentType::Func(ComponentFuncType {
                    async_: byte == 0x43,
                    params,
                    result,
                })
            }
            0x41 => ComponentType::Component(
                reader
                    .read_iter(MAX_WASM_COMPONENT_TYPE_DECLS, "component type declaration")?
                    .collect::<Result<_>>()?,
            ),
            0x42 => ComponentType::Instance(
                reader
                    .read_iter(MAX_WASM_INSTANCE_TYPE_DECLS, "instance type declaration")?
                    .collect::<Result<_>>()?,
            ),
            x => {
                if let Some(ty) = PrimitiveValType::from_byte(x) {
                    ComponentType::Defined(ComponentDefinedType::Primitive(ty))
                } else {
                    ComponentType::Defined(ComponentDefinedType::read(reader, x)?)
                }
            }
        })
    }
}

/// Represents part of a component type declaration in a WebAssembly component.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ComponentTypeDeclaration<'a> {
    /// The component type declaration is for a core type.
    CoreType(CoreType<'a>),
    /// The component type declaration is for a type.
    Type(ComponentType<'a>),
    /// The component type declaration is for an alias.
    Alias(ComponentAlias<'a>),
    /// The component type declaration is for an export.
    Export {
        /// The name of the export.
        name: ComponentExportName<'a>,
        /// The type reference for the export.
        ty: ComponentTypeRef,
    },
    /// The component type declaration is for an import.
    Import(ComponentImport<'a>),
}

impl<'a> FromReader<'a> for ComponentTypeDeclaration<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        // Component types are effectively instance types with the additional
        // variant of imports; check for imports here or delegate to
        // `InstanceTypeDeclaration` with the appropriate conversions.
        if reader.peek()? == 0x03 {
            reader.read_u8()?;
            return Ok(ComponentTypeDeclaration::Import(reader.read()?));
        }

        Ok(match reader.read()? {
            InstanceTypeDeclaration::CoreType(t) => ComponentTypeDeclaration::CoreType(t),
            InstanceTypeDeclaration::Type(t) => ComponentTypeDeclaration::Type(t),
            InstanceTypeDeclaration::Alias(a) => ComponentTypeDeclaration::Alias(a),
            InstanceTypeDeclaration::Export { name, ty } => {
                ComponentTypeDeclaration::Export { name, ty }
            }
        })
    }
}

/// Represents an instance type declaration in a WebAssembly component.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InstanceTypeDeclaration<'a> {
    /// The component type declaration is for a core type.
    CoreType(CoreType<'a>),
    /// The instance type declaration is for a type.
    Type(ComponentType<'a>),
    /// The instance type declaration is for an alias.
    Alias(ComponentAlias<'a>),
    /// The instance type declaration is for an export.
    Export {
        /// The name of the export.
        name: ComponentExportName<'a>,
        /// The type reference for the export.
        ty: ComponentTypeRef,
    },
}

impl<'a> FromReader<'a> for InstanceTypeDeclaration<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(match reader.read_u8()? {
            0x00 => InstanceTypeDeclaration::CoreType(reader.read()?),
            0x01 => InstanceTypeDeclaration::Type(reader.read()?),
            0x02 => InstanceTypeDeclaration::Alias(reader.read()?),
            0x04 => InstanceTypeDeclaration::Export {
                name: reader.read()?,
                ty: reader.read()?,
            },
            x => return reader.invalid_leading_byte(x, "component or instance type declaration"),
        })
    }
}

/// Represents a type of a function in a WebAssembly component.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComponentFuncType<'a> {
    /// Whether or not this is an async function.
    pub async_: bool,
    /// The function parameters.
    pub params: Box<[(&'a str, ComponentValType)]>,
    /// The function result.
    pub result: Option<ComponentValType>,
}

pub(crate) fn read_resultlist(reader: &mut BinaryReader<'_>) -> Result<Option<ComponentValType>> {
    match reader.read_u8()? {
        0x00 => Ok(Some(reader.read()?)),
        0x01 => match reader.read_u8()? {
            0x00 => Ok(None),
            x => return reader.invalid_leading_byte(x, "number of results"),
        },
        x => return reader.invalid_leading_byte(x, "component function results"),
    }
}

/// Represents a case in a variant type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCase<'a> {
    /// The name of the variant case.
    pub name: &'a str,
    /// The value type of the variant case.
    pub ty: Option<ComponentValType>,
    /// The index of the variant case that is refined by this one.
    pub refines: Option<u32>,
}

impl<'a> FromReader<'a> for VariantCase<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(VariantCase {
            name: reader.read()?,
            ty: reader.read()?,
            refines: match reader.read_u8()? {
                0x0 => None,
                0x1 => Some(reader.read_var_u32()?),
                x => return reader.invalid_leading_byte(x, "variant case refines"),
            },
        })
    }
}

/// Represents a defined type in a WebAssembly component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentDefinedType<'a> {
    /// The type is one of the primitive value types.
    Primitive(PrimitiveValType),
    /// The type is a record with the given fields.
    Record(Box<[(&'a str, ComponentValType)]>),
    /// The type is a variant with the given cases.
    Variant(Box<[VariantCase<'a>]>),
    /// The type is a list of the given value type.
    List(ComponentValType),
    /// The type is a map of the given key and value types.
    Map(ComponentValType, ComponentValType),
    /// The type is a fixed size list of the given value type.
    FixedSizeList(ComponentValType, u32),
    /// The type is a tuple of the given value types.
    Tuple(Box<[ComponentValType]>),
    /// The type is flags with the given names.
    Flags(Box<[&'a str]>),
    /// The type is an enum with the given tags.
    Enum(Box<[&'a str]>),
    /// The type is an option of the given value type.
    Option(ComponentValType),
    /// The type is a result type.
    Result {
        /// The type returned for success.
        ok: Option<ComponentValType>,
        /// The type returned for failure.
        err: Option<ComponentValType>,
    },
    /// An owned handle to a resource.
    Own(u32),
    /// A borrowed handle to a resource.
    Borrow(u32),
    /// A future type with the specified payload type.
    Future(Option<ComponentValType>),
    /// A stream type with the specified payload type.
    Stream(Option<ComponentValType>),
}

impl<'a> ComponentDefinedType<'a> {
    fn read(reader: &mut BinaryReader<'a>, byte: u8) -> Result<ComponentDefinedType<'a>> {
        Ok(match byte {
            0x72 => ComponentDefinedType::Record(
                reader
                    .read_iter(MAX_WASM_RECORD_FIELDS, "record field")?
                    .collect::<Result<_>>()?,
            ),
            0x71 => ComponentDefinedType::Variant(
                reader
                    .read_iter(MAX_WASM_VARIANT_CASES, "variant cases")?
                    .collect::<Result<_>>()?,
            ),
            0x70 => ComponentDefinedType::List(reader.read()?),
            0x63 => ComponentDefinedType::Map(reader.read()?, reader.read()?),
            0x6f => ComponentDefinedType::Tuple(
                reader
                    .read_iter(MAX_WASM_TUPLE_TYPES, "tuple types")?
                    .collect::<Result<_>>()?,
            ),
            0x6e => ComponentDefinedType::Flags(
                reader
                    .read_iter(MAX_WASM_FLAG_NAMES, "flag names")?
                    .collect::<Result<_>>()?,
            ),
            0x6d => ComponentDefinedType::Enum(
                reader
                    .read_iter(MAX_WASM_ENUM_CASES, "enum cases")?
                    .collect::<Result<_>>()?,
            ),
            // NOTE: 0x6c (union) removed
            0x6b => ComponentDefinedType::Option(reader.read()?),
            0x6a => ComponentDefinedType::Result {
                ok: reader.read()?,
                err: reader.read()?,
            },
            0x69 => ComponentDefinedType::Own(reader.read()?),
            0x68 => ComponentDefinedType::Borrow(reader.read()?),
            0x67 => ComponentDefinedType::FixedSizeList(reader.read()?, reader.read_var_u32()?),
            0x66 => ComponentDefinedType::Stream(reader.read()?),
            0x65 => ComponentDefinedType::Future(reader.read()?),
            x => return reader.invalid_leading_byte(x, "component defined type"),
        })
    }
}

/// A reader for the type section of a WebAssembly component.
///
/// # Examples
///
/// ```
/// use wasmparser::{ComponentTypeSectionReader, BinaryReader};
/// let data: &[u8] = &[0x01, 0x40, 0x01, 0x03, b'f', b'o', b'o', 0x73, 0x00, 0x73];
/// let reader = BinaryReader::new(data, 0);
/// let mut reader = ComponentTypeSectionReader::new(reader).unwrap();
/// for ty in reader {
///     println!("Type {:?}", ty.expect("type"));
/// }
/// ```
pub type ComponentTypeSectionReader<'a> = SectionLimited<'a, ComponentType<'a>>;
