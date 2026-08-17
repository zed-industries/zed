mod attributes;
mod capturer;

use super::Mutations;
use crate::errors::RewritingError;

pub(super) use self::attributes::Attributes;
pub use self::attributes::{Attribute, AttributeNameError};
pub use self::capturer::*;

// Pub only for integration tests
pub trait Serialize {
    fn into_bytes(self, output_handler: &mut dyn FnMut(&[u8])) -> Result<(), RewritingError>;
}

macro_rules! impl_serialize {
    ($Token:ident) => {
        impl crate::rewritable_units::Serialize for $Token<'_> {
            #[inline]
            fn into_bytes(
                mut self,
                output_handler: &mut dyn FnMut(&[u8]),
            ) -> Result<(), crate::errors::RewritingError> {
                let mut encoder = crate::rewritable_units::StreamingHandlerSink::new(
                    self.encoding(),
                    output_handler,
                );
                match self.mutations.take() {
                    None => self.serialize_self(&mut encoder),
                    Some(mutations) => {
                        mutations
                            .content_before
                            .encode(&mut encoder)
                            .map_err(crate::errors::RewritingError::ContentHandlerError)?;

                        if !mutations.removed {
                            self.serialize_self(&mut encoder)?;
                        } else {
                            mutations
                                .replacement
                                .encode(&mut encoder)
                                .map_err(crate::errors::RewritingError::ContentHandlerError)?;
                        }

                        mutations
                            .content_after
                            .encode(&mut encoder)
                            .map_err(crate::errors::RewritingError::ContentHandlerError)
                    }
                }
            }
        }
    };
}

mod comment;
mod doctype;
mod end_tag;
mod start_tag;
mod text_chunk;

pub use self::comment::{Comment, CommentTextError};
pub use self::doctype::Doctype;
pub use self::end_tag::EndTag;
pub use self::start_tag::StartTag;
pub use self::text_chunk::TextChunk;

// Pub only for integration tests
#[derive(Debug)]
pub enum Token<'i> {
    TextChunk(TextChunk<'i>),
    StartTag(StartTag<'i>),
    EndTag(EndTag<'i>),
    Comment(Comment<'i>),
    Doctype(Doctype<'i>),
}

impl Serialize for Token<'_> {
    #[inline]
    fn into_bytes(self, output_handler: &mut dyn FnMut(&[u8])) -> Result<(), RewritingError> {
        match self {
            Token::TextChunk(t) => t.into_bytes(output_handler),
            Token::StartTag(t) => t.into_bytes(output_handler),
            Token::EndTag(t) => t.into_bytes(output_handler),
            Token::Comment(t) => t.into_bytes(output_handler),
            Token::Doctype(t) => t.into_bytes(output_handler),
        }
    }
}
