use crate::{encode_section, Encode, Section, SectionId};

/// An encoder for the tag section.
///
/// # Example
///
/// ```
/// use wasm_encoder::{Module, TagSection, TagType, TagKind};
///
/// let mut tags = TagSection::new();
/// tags.tag(TagType {
///     kind: TagKind::Exception,
///     func_type_idx: 0,
/// });
///
/// let mut module = Module::new();
/// module.section(&tags);
///
/// let wasm_bytes = module.finish();
/// ```
#[derive(Clone, Default, Debug)]
pub struct TagSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl TagSection {
    /// Create a new tag section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of tags in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define a tag.
    pub fn tag(&mut self, tag_type: TagType) -> &mut Self {
        tag_type.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }
}

impl Encode for TagSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl Section for TagSection {
    fn id(&self) -> u8 {
        SectionId::Tag.into()
    }
}

/// Represents a tag kind.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagKind {
    /// The tag is an exception type.
    Exception = 0x0,
}

#[cfg(feature = "wasmparser")]
impl From<wasmparser::TagKind> for TagKind {
    fn from(kind: wasmparser::TagKind) -> Self {
        match kind {
            wasmparser::TagKind::Exception => TagKind::Exception,
        }
    }
}

/// A tag's type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagType {
    /// The kind of tag
    pub kind: TagKind,
    /// The function type this tag uses
    pub func_type_idx: u32,
}

impl Encode for TagType {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.push(self.kind as u8);
        self.func_type_idx.encode(sink);
    }
}

#[cfg(feature = "wasmparser")]
impl From<wasmparser::TagType> for TagType {
    fn from(tag_ty: wasmparser::TagType) -> Self {
        TagType {
            kind: tag_ty.kind.into(),
            func_type_idx: tag_ty.func_type_idx,
        }
    }
}
