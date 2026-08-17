/*!
Tags for fragments of formatted values.
*/

pub use sval::tags::NUMBER;

/**
A tag for an atom.
*/
pub const ATOM: sval::Tag = sval::Tag::new("FMT_ATOM");
/**
A tag for punctuation.
*/
pub const PUNCT: sval::Tag = sval::Tag::new("FMT_PUNCT");
/**
A tag for an identifier.
*/
pub const IDENT: sval::Tag = sval::Tag::new("FMT_IDENT");
/**
A tag for generic text.
*/
pub const TEXT: sval::Tag = sval::Tag::new("FMT_TEXT");
/**
A tag for whitespace.
*/
pub const WS: sval::Tag = sval::Tag::new("FMT_WS");
