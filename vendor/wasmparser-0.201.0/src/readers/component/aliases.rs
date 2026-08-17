use crate::{BinaryReader, ComponentExternalKind, ExternalKind, FromReader, Result};

/// Represents the kind of an outer alias in a WebAssembly component.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentOuterAliasKind {
    /// The alias is to a core module.
    CoreModule,
    /// The alias is to a core type.
    CoreType,
    /// The alias is to a component type.
    Type,
    /// The alias is to a component.
    Component,
}

/// Represents an alias in a WebAssembly component.
#[derive(Debug, Clone)]
pub enum ComponentAlias<'a> {
    /// The alias is to an export of a component instance.
    InstanceExport {
        /// The alias kind.
        kind: ComponentExternalKind,
        /// The instance index.
        instance_index: u32,
        /// The export name.
        name: &'a str,
    },
    /// The alias is to an export of a module instance.
    CoreInstanceExport {
        /// The alias kind.
        kind: ExternalKind,
        /// The instance index.
        instance_index: u32,
        /// The export name.
        name: &'a str,
    },
    /// The alias is to an outer item.
    Outer {
        /// The alias kind.
        kind: ComponentOuterAliasKind,
        /// The outward count, starting at zero for the current component.
        count: u32,
        /// The index of the item within the outer component.
        index: u32,
    },
}

/// Section reader for the component alias section
pub type ComponentAliasSectionReader<'a> = crate::SectionLimited<'a, ComponentAlias<'a>>;

impl<'a> FromReader<'a> for ComponentAlias<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        // We don't know what type of alias it is yet, so just read the sort bytes
        let offset = reader.original_position();
        let byte1 = reader.read_u8()?;
        let byte2 = if byte1 == 0x00 {
            Some(reader.read_u8()?)
        } else {
            None
        };

        Ok(match reader.read_u8()? {
            0x00 => ComponentAlias::InstanceExport {
                kind: ComponentExternalKind::from_bytes(byte1, byte2, offset)?,
                instance_index: reader.read_var_u32()?,
                name: reader.read_string()?,
            },
            0x01 => ComponentAlias::CoreInstanceExport {
                kind: BinaryReader::external_kind_from_byte(
                    byte2.ok_or_else(|| {
                        BinaryReader::invalid_leading_byte_error(
                            byte1,
                            "core instance export kind",
                            offset,
                        )
                    })?,
                    offset,
                )?,
                instance_index: reader.read_var_u32()?,
                name: reader.read_string()?,
            },
            0x02 => ComponentAlias::Outer {
                kind: component_outer_alias_kind_from_bytes(byte1, byte2, offset)?,
                count: reader.read_var_u32()?,
                index: reader.read_var_u32()?,
            },
            x => reader.invalid_leading_byte(x, "alias")?,
        })
    }
}

fn component_outer_alias_kind_from_bytes(
    byte1: u8,
    byte2: Option<u8>,
    offset: usize,
) -> Result<ComponentOuterAliasKind> {
    Ok(match byte1 {
        0x00 => match byte2.unwrap() {
            0x10 => ComponentOuterAliasKind::CoreType,
            0x11 => ComponentOuterAliasKind::CoreModule,
            x => {
                return Err(BinaryReader::invalid_leading_byte_error(
                    x,
                    "component outer alias kind",
                    offset + 1,
                ))
            }
        },
        0x03 => ComponentOuterAliasKind::Type,
        0x04 => ComponentOuterAliasKind::Component,
        x => {
            return Err(BinaryReader::invalid_leading_byte_error(
                x,
                "component outer alias kind",
                offset,
            ))
        }
    })
}
