use crate::{BinaryReader, Result};
use core::fmt;
use core::ops::Range;

/// A reader for custom sections of a WebAssembly module.
#[derive(Clone)]
pub struct CustomSectionReader<'a> {
    name: &'a str,
    reader: BinaryReader<'a>,
}

impl<'a> CustomSectionReader<'a> {
    /// Constructs a new `CustomSectionReader` for the given data and offset.
    pub fn new(mut reader: BinaryReader<'a>) -> Result<CustomSectionReader<'a>> {
        let name = reader.read_string()?;
        Ok(CustomSectionReader { name, reader })
    }

    /// The name of the custom section.
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// The offset, relative to the start of the original module or component,
    /// that the `data` payload for this custom section starts at.
    pub fn data_offset(&self) -> usize {
        self.reader.original_position()
    }

    /// The actual contents of the custom section.
    pub fn data(&self) -> &'a [u8] {
        self.reader.remaining_buffer()
    }

    /// The range of bytes that specify this whole custom section (including
    /// both the name of this custom section and its data) specified in
    /// offsets relative to the start of the byte stream.
    pub fn range(&self) -> Range<usize> {
        self.reader.range()
    }

    /// Attempts to match and see if this custom section is statically known to
    /// `wasmparser` with any known section reader.
    ///
    /// This will inspect `self.name()` and return a [`KnownCustom`] if the name
    /// matches a known custom section where there is a parser available for it.
    /// This can also be used as a convenience function for creating such
    /// parsers.
    ///
    /// If the custom section name is not known, or if a reader could not be
    /// created, then `KnownCustom::Unknown` is returned.
    pub fn as_known(&self) -> KnownCustom<'a> {
        match self.name() {
            "name" => KnownCustom::Name(crate::NameSectionReader::new(self.reader.shrink())),
            #[cfg(feature = "component-model")]
            "component-name" => KnownCustom::ComponentName(crate::ComponentNameSectionReader::new(
                self.reader.shrink(),
            )),
            "metadata.code.branch_hint" => {
                match crate::BranchHintSectionReader::new(self.reader.shrink()) {
                    Ok(s) => KnownCustom::BranchHints(s),
                    Err(_) => KnownCustom::Unknown,
                }
            }
            "producers" => match crate::ProducersSectionReader::new(self.reader.shrink()) {
                Ok(s) => KnownCustom::Producers(s),
                Err(_) => KnownCustom::Unknown,
            },
            "dylink.0" => {
                KnownCustom::Dylink0(crate::Dylink0SectionReader::new(self.reader.shrink()))
            }
            "core" => match crate::CoreDumpSection::new(self.reader.shrink()) {
                Ok(s) => KnownCustom::CoreDump(s),
                Err(_) => KnownCustom::Unknown,
            },
            "coremodules" => match crate::CoreDumpModulesSection::new(self.reader.shrink()) {
                Ok(s) => KnownCustom::CoreDumpModules(s),
                Err(_) => KnownCustom::Unknown,
            },
            "coreinstances" => match crate::CoreDumpInstancesSection::new(self.reader.shrink()) {
                Ok(s) => KnownCustom::CoreDumpInstances(s),
                Err(_) => KnownCustom::Unknown,
            },
            "corestack" => match crate::CoreDumpStackSection::new(self.reader.shrink()) {
                Ok(s) => KnownCustom::CoreDumpStack(s),
                Err(_) => KnownCustom::Unknown,
            },
            "linking" => match crate::LinkingSectionReader::new(self.reader.shrink()) {
                Ok(s) => KnownCustom::Linking(s),
                Err(_) => KnownCustom::Unknown,
            },
            s if s.starts_with("reloc.") => {
                match crate::RelocSectionReader::new(self.reader.shrink()) {
                    Ok(s) => KnownCustom::Reloc(s),
                    Err(_) => KnownCustom::Unknown,
                }
            }
            _ => KnownCustom::Unknown,
        }
    }
}

/// Return value of [`CustomSectionReader::as_known`].
///
/// Note that this is `#[non_exhaustive]` because depending on crate features
/// this enumeration will different entries.
#[allow(missing_docs)]
#[non_exhaustive]
pub enum KnownCustom<'a> {
    Name(crate::NameSectionReader<'a>),
    #[cfg(feature = "component-model")]
    ComponentName(crate::ComponentNameSectionReader<'a>),
    BranchHints(crate::BranchHintSectionReader<'a>),
    Producers(crate::ProducersSectionReader<'a>),
    Dylink0(crate::Dylink0SectionReader<'a>),
    CoreDump(crate::CoreDumpSection<'a>),
    CoreDumpStack(crate::CoreDumpStackSection<'a>),
    CoreDumpInstances(crate::CoreDumpInstancesSection),
    CoreDumpModules(crate::CoreDumpModulesSection<'a>),
    Linking(crate::LinkingSectionReader<'a>),
    Reloc(crate::RelocSectionReader<'a>),
    Unknown,
}

impl<'a> fmt::Debug for CustomSectionReader<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomSectionReader")
            .field("name", &self.name)
            .field("data_offset", &self.data_offset())
            .field("data", &"...")
            .field("range", &self.range())
            .finish()
    }
}
