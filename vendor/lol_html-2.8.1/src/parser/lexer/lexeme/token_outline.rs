use crate::base::{Align, Range};
use crate::html::{LocalNameHash, Namespace, TextType};
use crate::parser::AttributeBuffer;

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct AttributeOutline {
    pub name: Range,
    pub value: Range,
    pub raw_range: Range,
}

impl Align for AttributeOutline {
    #[inline]
    fn align(&mut self, offset: usize) {
        self.name.align(offset);
        self.value.align(offset);
        self.raw_range.align(offset);
    }
}

#[derive(Debug)]
pub(crate) enum TagTokenOutline {
    StartTag {
        name: Range,
        name_hash: LocalNameHash,
        ns: Namespace,
        attributes: AttributeBuffer,
        self_closing: bool,
    },

    EndTag {
        name: Range,
        name_hash: LocalNameHash,
    },
}

#[derive(Debug)]
pub struct DoctypeTokenOutline {
    pub name: Option<Range>,
    pub public_id: Option<Range>,
    pub system_id: Option<Range>,
    pub force_quirks: bool,
}

#[derive(Debug)]
pub(crate) enum NonTagContentTokenOutline {
    Text(TextType),
    Comment(Range),
    Doctype(Box<DoctypeTokenOutline>),
    Eof,
}

impl Align for TagTokenOutline {
    #[inline]
    fn align(&mut self, offset: usize) {
        match self {
            Self::StartTag {
                name, attributes, ..
            } => {
                name.align(offset);
                attributes.as_mut_slice().align(offset);
            }
            Self::EndTag { name, .. } => name.align(offset),
        }
    }
}

impl Align for NonTagContentTokenOutline {
    #[inline]
    fn align(&mut self, offset: usize) {
        match self {
            Self::Comment(text) => text.align(offset),
            Self::Doctype(doctype) => {
                doctype.name.align(offset);
                doctype.public_id.align(offset);
                doctype.system_id.align(offset);
            }
            _ => (),
        }
    }
}
