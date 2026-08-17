use crate::{BinaryReader, BinaryReaderError, NameMap, Result, Subsection, Subsections};
use std::ops::Range;

/// Type used to iterate and parse the contents of the `component-name` custom
/// section in compnents, similar to the `name` section of core modules.
pub type ComponentNameSectionReader<'a> = Subsections<'a, ComponentName<'a>>;

/// Represents a name read from the names custom section.
#[derive(Clone)]
#[allow(missing_docs)]
pub enum ComponentName<'a> {
    Component {
        name: &'a str,
        name_range: Range<usize>,
    },
    CoreFuncs(NameMap<'a>),
    CoreGlobals(NameMap<'a>),
    CoreMemories(NameMap<'a>),
    CoreTables(NameMap<'a>),
    CoreModules(NameMap<'a>),
    CoreInstances(NameMap<'a>),
    CoreTypes(NameMap<'a>),
    Types(NameMap<'a>),
    Instances(NameMap<'a>),
    Components(NameMap<'a>),
    Funcs(NameMap<'a>),
    Values(NameMap<'a>),

    /// An unknown [name subsection](https://webassembly.github.io/spec/core/appendix/custom.html#subsections).
    Unknown {
        /// The identifier for this subsection.
        ty: u8,
        /// The contents of this subsection.
        data: &'a [u8],
        /// The range of bytes, relative to the start of the original data
        /// stream, that the contents of this subsection reside in.
        range: Range<usize>,
    },
}

impl<'a> Subsection<'a> for ComponentName<'a> {
    fn from_reader(id: u8, mut reader: BinaryReader<'a>) -> Result<Self> {
        let data = reader.remaining_buffer();
        let offset = reader.original_position();
        Ok(match id {
            0 => {
                let name = reader.read_string()?;
                if !reader.eof() {
                    return Err(BinaryReaderError::new(
                        "trailing data at the end of a name",
                        reader.original_position(),
                    ));
                }
                ComponentName::Component {
                    name,
                    name_range: offset..offset + reader.position,
                }
            }
            1 => {
                let ctor: fn(NameMap<'a>) -> ComponentName<'a> = match reader.read_u8()? {
                    0x00 => match reader.read_u8()? {
                        0x00 => ComponentName::CoreFuncs,
                        0x01 => ComponentName::CoreTables,
                        0x02 => ComponentName::CoreMemories,
                        0x03 => ComponentName::CoreGlobals,
                        0x10 => ComponentName::CoreTypes,
                        0x11 => ComponentName::CoreModules,
                        0x12 => ComponentName::CoreInstances,
                        _ => {
                            return Ok(ComponentName::Unknown {
                                ty: 1,
                                data,
                                range: offset..offset + data.len(),
                            });
                        }
                    },
                    0x01 => ComponentName::Funcs,
                    0x02 => ComponentName::Values,
                    0x03 => ComponentName::Types,
                    0x04 => ComponentName::Components,
                    0x05 => ComponentName::Instances,
                    _ => {
                        return Ok(ComponentName::Unknown {
                            ty: 1,
                            data,
                            range: offset..offset + data.len(),
                        });
                    }
                };
                ctor(NameMap::new(
                    reader.remaining_buffer(),
                    reader.original_position(),
                )?)
            }
            ty => ComponentName::Unknown {
                ty,
                data,
                range: offset..offset + data.len(),
            },
        })
    }
}
