use crate::reader::lexer::Token;
use crate::Encoding;

use std::borrow::Cow;
use std::error::Error as _;
use std::{error, fmt, io, str};

use crate::common::{Position, TextPosition};
use crate::util;

/// Failure reason
#[derive(Debug)]
pub enum ErrorKind {
    /// This is an ill-formed XML document
    Syntax(Cow<'static, str>),
    /// Reader/writer reported an error
    Io(io::Error),
    /// The document contains bytes that are not allowed in UTF-8 strings
    Utf8(str::Utf8Error),
    /// The document ended while they were elements/comments/etc. still open
    UnexpectedEof,
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub(crate) enum SyntaxError {
    CannotRedefineXmlnsPrefix,
    CannotRedefineXmlPrefix,
    /// Recursive custom entity expanded to too many chars, it could be DoS
    EntityTooBig,
    EmptyEntity,
    NoRootElement,
    ProcessingInstructionWithoutName,
    UnbalancedRootElement,
    UnexpectedEof,
    UnexpectedOpeningTag,
    /// Missing `]]>`
    UnclosedCdata,
    UnexpectedQualifiedName(Token),
    UnexpectedTokenOutsideRoot(Token),
    UnexpectedToken(Token),
    UnexpectedTokenInEntity(Token),
    UnexpectedTokenInClosingTag(Token),
    UnexpectedTokenInOpeningTag(Token),
    InvalidQualifiedName(Box<str>),
    UnboundAttribute(Box<str>),
    UnboundElementPrefix(Box<str>),
    UnexpectedClosingTag(Box<str>),
    UnexpectedName(Box<str>),
    /// Found <?xml-like PI not at the beginning of a document,
    /// which is an error, see section 2.6 of XML 1.1 spec
    UnexpectedProcessingInstruction(Box<str>, Token),
    CannotUndefinePrefix(Box<str>),
    InvalidCharacterEntity(u32),
    InvalidDefaultNamespace(Box<str>),
    InvalidNamePrefix(Box<str>),
    InvalidNumericEntity(Box<str>),
    InvalidStandaloneDeclaration(Box<str>),
    InvalidXmlProcessingInstruction(Box<str>),
    RedefinedAttribute(Box<str>),
    UndefinedEntity(Box<str>),
    UnexpectedEntity(Box<str>),
    UnexpectedNameInsideXml(Box<str>),
    UnsupportedEncoding(Box<str>),
    /// In DTD
    UnknownMarkupDeclaration(Box<str>),
    UnexpectedXmlVersion(Box<str>),
    ConflictingEncoding(Encoding, Encoding),
    UnexpectedTokenBefore(&'static str, char),
    /// Document has more stuff than `ParserConfig` allows
    ExceededConfiguredLimit,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_cow().fmt(f)
    }
}

impl SyntaxError {
    #[inline(never)]
    #[cold]
    pub(crate) fn to_cow(&self) -> Cow<'static, str> {
        match *self {
            Self::CannotRedefineXmlnsPrefix => "Cannot redefine XMLNS prefix".into(),
            Self::CannotRedefineXmlPrefix => "Default XMLNS prefix cannot be rebound to another value".into(),
            Self::EmptyEntity => "Encountered empty entity".into(),
            Self::EntityTooBig => "Entity too big".into(),
            Self::NoRootElement => "Unexpected end of stream: no root element found".into(),
            Self::ProcessingInstructionWithoutName => "Encountered processing instruction without a name".into(),
            Self::UnbalancedRootElement => "Unexpected end of stream: still inside the root element".into(),
            Self::UnclosedCdata => "Unclosed <![CDATA[".into(),
            Self::UnexpectedEof => "Unexpected end of stream".into(),
            Self::UnexpectedOpeningTag => "'<' is not allowed in attributes".into(),
            Self::CannotUndefinePrefix(ref ln) => format!("Cannot undefine prefix '{ln}'").into(),
            Self::ConflictingEncoding(a, b) => format!("Declared encoding {a}, but uses {b}").into(),
            Self::InvalidCharacterEntity(num) => format!("Invalid character U+{num:04X}").into(),
            Self::InvalidDefaultNamespace(ref name) => format!("Namespace '{name}' cannot be default").into(),
            Self::InvalidNamePrefix(ref prefix) => format!("'{prefix}' cannot be an element name prefix").into(),
            Self::InvalidNumericEntity(ref v) => format!("Invalid numeric entity: {v}").into(),
            Self::InvalidQualifiedName(ref e) => format!("Qualified name is invalid: {e}").into(),
            Self::InvalidStandaloneDeclaration(ref value) => format!("Invalid standalone declaration value: {value}").into(),
            Self::InvalidXmlProcessingInstruction(ref name) => format!("Invalid processing instruction: <?{name}\nThe XML spec only allows \"<?xml\" at the very beginning of the file, with no whitespace, comments, or any elements before it").into(),
            Self::RedefinedAttribute(ref name) => format!("Attribute '{name}' is redefined").into(),
            Self::UnboundAttribute(ref name) => format!("Attribute {name} prefix is unbound").into(),
            Self::UnboundElementPrefix(ref name) => format!("Element {name} prefix is unbound").into(),
            Self::UndefinedEntity(ref v) => format!("Undefined entity: {v}").into(),
            Self::UnexpectedClosingTag(ref expected_got) => format!("Unexpected closing tag: {expected_got}").into(),
            Self::UnexpectedEntity(ref name) => format!("Unexpected entity: {name}").into(),
            Self::UnexpectedName(ref name) => format!("Unexpected name: {name}").into(),
            Self::UnexpectedNameInsideXml(ref name) => format!("Unexpected name inside XML declaration: {name}").into(),
            Self::UnexpectedProcessingInstruction(ref buf, token) => format!("Unexpected token inside processing instruction: <?{buf}{token}").into(),
            Self::UnexpectedQualifiedName(e) => format!("Unexpected token inside qualified name: {e}").into(),
            Self::UnexpectedToken(token) => format!("Unexpected token: {token}").into(),
            Self::UnexpectedTokenBefore(before, c) => format!("Unexpected token '{before}' before '{c}'").into(),
            Self::UnexpectedTokenInClosingTag(token) => format!("Unexpected token inside closing tag: {token}").into(),
            Self::UnexpectedTokenInEntity(token) => format!("Unexpected token inside entity: {token}").into(),
            Self::UnexpectedTokenInOpeningTag(token) => format!("Unexpected token inside opening tag: {token}").into(),
            Self::UnexpectedTokenOutsideRoot(token) => format!("Unexpected characters outside the root element: {token}").into(),
            Self::UnexpectedXmlVersion(ref version) => format!("Invalid XML version: {version}").into(),
            Self::UnknownMarkupDeclaration(ref v) => format!("Unknown markup declaration: {v}").into(),
            Self::UnsupportedEncoding(ref v) => format!("Unsupported encoding: {v}").into(),
            Self::ExceededConfiguredLimit => "This document is larger/more complex than allowed by the parser's configuration".into(),
        }
    }
}

/// An XML parsing error.
///
/// Consists of a 2D position in a document and a textual message describing the error.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Error {
    pub(crate) pos: TextPosition,
    pub(crate) kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ErrorKind::{Io, Syntax, UnexpectedEof, Utf8};

        write!(f, "{} ", self.pos)?;
        match &self.kind {
            Io(io_error) => io_error.fmt(f),
            Utf8(reason) => reason.fmt(f),
            Syntax(msg) => f.write_str(msg),
            UnexpectedEof => f.write_str("Unexpected EOF"),
        }
    }
}

impl Position for Error {
    #[inline]
    fn position(&self) -> TextPosition { self.pos }
}

impl Error {
    /// Returns a reference to a message which is contained inside this error.
    #[cold]
    #[doc(hidden)]
    #[allow(deprecated)]
    #[must_use]
    pub fn msg(&self) -> &str {
        use self::ErrorKind::{Io, Syntax, UnexpectedEof, Utf8};
        match &self.kind {
            Io(io_error) => io_error.description(),
            Utf8(reason) => reason.description(),
            Syntax(msg) => msg.as_ref(),
            UnexpectedEof => "Unexpected EOF",
        }
    }

    /// Failure reason
    #[must_use]
    #[inline]
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl error::Error for Error {
    #[allow(deprecated)]
    #[cold]
    fn description(&self) -> &str { self.msg() }
}

impl<'a, P, M> From<(&'a P, M)> for Error where P: Position, M: Into<Cow<'static, str>> {
    #[cold]
    fn from(orig: (&'a P, M)) -> Self {
        Self {
            pos: orig.0.position(),
            kind: ErrorKind::Syntax(orig.1.into()),
        }
    }
}

impl From<util::CharReadError> for Error {
    #[cold]
    fn from(e: util::CharReadError) -> Self {
        use crate::util::CharReadError::{Io, UnexpectedEof, Utf8};
        Self {
            pos: TextPosition::new(),
            kind: match e {
                UnexpectedEof => ErrorKind::UnexpectedEof,
                Utf8(reason) => ErrorKind::Utf8(reason),
                Io(io_error) => ErrorKind::Io(io_error),
            },
        }
    }
}

impl From<io::Error> for Error {
    #[cold]
    fn from(e: io::Error) -> Self {
        Self {
            pos: TextPosition::new(),
            kind: ErrorKind::Io(e),
        }
    }
}

impl Clone for ErrorKind {
    #[cold]
    fn clone(&self) -> Self {
        use self::ErrorKind::{Io, Syntax, UnexpectedEof, Utf8};
        match self {
            UnexpectedEof => UnexpectedEof,
            Utf8(reason) => Utf8(*reason),
            Io(io_error) => Io(io::Error::new(io_error.kind(), io_error.to_string())),
            Syntax(msg) => Syntax(msg.clone()),
        }
    }
}
impl PartialEq for ErrorKind {
    #[allow(deprecated)]
    fn eq(&self, other: &Self) -> bool {
        use self::ErrorKind::{Io, Syntax, UnexpectedEof, Utf8};
        match (self, other) {
            (UnexpectedEof, UnexpectedEof) => true,
            (Utf8(left), Utf8(right)) => left == right,
            (Io(left), Io(right)) =>
                left.kind() == right.kind() &&
                left.description() == right.description(),
            (Syntax(left), Syntax(right)) =>
                left == right,

            (_, _) => false,
        }
    }
}
impl Eq for ErrorKind {}

#[test]
fn err_size() {
    assert!(std::mem::size_of::<SyntaxError>() <= 24);
}
