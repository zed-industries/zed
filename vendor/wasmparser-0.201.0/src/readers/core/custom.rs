use crate::{BinaryReader, Result};
use std::ops::Range;

/// A reader for custom sections of a WebAssembly module.
#[derive(Clone)]
pub struct CustomSectionReader<'a> {
    // NB: these fields are public to the crate to make testing easier.
    pub(crate) name: &'a str,
    pub(crate) data_offset: usize,
    pub(crate) data: &'a [u8],
    pub(crate) range: Range<usize>,
}

impl<'a> CustomSectionReader<'a> {
    /// Constructs a new `CustomSectionReader` for the given data and offset.
    pub fn new(data: &'a [u8], offset: usize) -> Result<CustomSectionReader<'a>> {
        let mut reader = BinaryReader::new_with_offset(data, offset);
        let name = reader.read_string()?;
        let data_offset = reader.original_position();
        let data = reader.remaining_buffer();
        let range = reader.range();
        Ok(CustomSectionReader {
            name,
            data_offset,
            data,
            range,
        })
    }

    /// The name of the custom section.
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// The offset, relative to the start of the original module or component,
    /// that the `data` payload for this custom section starts at.
    pub fn data_offset(&self) -> usize {
        self.data_offset
    }

    /// The actual contents of the custom section.
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// The range of bytes that specify this whole custom section (including
    /// both the name of this custom section and its data) specified in
    /// offsets relative to the start of the byte stream.
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }
}

impl<'a> std::fmt::Debug for CustomSectionReader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomSectionReader")
            .field("name", &self.name)
            .field("data_offset", &self.data_offset)
            .field("data", &"...")
            .field("range", &self.range)
            .finish()
    }
}
