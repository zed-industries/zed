use core::fmt;
use core::str;
#[cfg(feature = "std")]
use std::error;


/// An XML parser errors.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Error {
    InvalidDeclaration(StreamError, TextPos),
    InvalidComment(StreamError, TextPos),
    InvalidPI(StreamError, TextPos),
    InvalidDoctype(StreamError, TextPos),
    InvalidEntity(StreamError, TextPos),
    InvalidElement(StreamError, TextPos),
    InvalidAttribute(StreamError, TextPos),
    InvalidCdata(StreamError, TextPos),
    InvalidCharData(StreamError, TextPos),
    UnknownToken(TextPos),
}

impl Error {
    /// Returns the error position.
    pub fn pos(&self) -> TextPos {
        match *self {
            Error::InvalidDeclaration(_, pos) => pos,
            Error::InvalidComment(_, pos) => pos,
            Error::InvalidPI(_, pos) => pos,
            Error::InvalidDoctype(_, pos) => pos,
            Error::InvalidEntity(_, pos) => pos,
            Error::InvalidElement(_, pos) => pos,
            Error::InvalidAttribute(_, pos) => pos,
            Error::InvalidCdata(_, pos) => pos,
            Error::InvalidCharData(_, pos) => pos,
            Error::UnknownToken(pos) => pos,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidDeclaration(ref cause, pos) => {
                write!(f, "invalid XML declaration at {} cause {}", pos, cause)
            }
            Error::InvalidComment(ref cause, pos) => {
                write!(f, "invalid comment at {} cause {}", pos, cause)
            }
            Error::InvalidPI(ref cause, pos) => {
                write!(f, "invalid processing instruction at {} cause {}", pos, cause)
            }
            Error::InvalidDoctype(ref cause, pos) => {
                write!(f, "invalid DTD at {} cause {}", pos, cause)
            }
            Error::InvalidEntity(ref cause, pos) => {
                write!(f, "invalid DTD entity at {} cause {}", pos, cause)
            }
            Error::InvalidElement(ref cause, pos) => {
                write!(f, "invalid element at {} cause {}", pos, cause)
            }
            Error::InvalidAttribute(ref cause, pos) => {
                write!(f, "invalid attribute at {} cause {}", pos, cause)
            }
            Error::InvalidCdata(ref cause, pos) => {
                write!(f, "invalid CDATA at {} cause {}", pos, cause)
            }
            Error::InvalidCharData(ref cause, pos) => {
                write!(f, "invalid character data at {} cause {}", pos, cause)
            }
            Error::UnknownToken(pos) => {
                write!(f, "unknown token at {}", pos)
            }
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        "an XML parsing error"
    }
}


/// A stream parser errors.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum StreamError {
    /// The steam ended earlier than we expected.
    ///
    /// Should only appear on invalid input data.
    /// Errors in a valid XML should be handled by errors below.
    UnexpectedEndOfStream,

    /// An invalid name.
    InvalidName,

    /// A non-XML character has occurred.
    ///
    /// Valid characters are: <https://www.w3.org/TR/xml/#char32>
    NonXmlChar(char, TextPos),

    /// An invalid/unexpected character.
    ///
    /// The first byte is an actual one, the second one is expected.
    ///
    /// We are using a single value to reduce the struct size.
    InvalidChar(u8, u8, TextPos),

    /// An invalid/unexpected character.
    ///
    /// Just like `InvalidChar`, but specifies multiple expected characters.
    InvalidCharMultiple(u8, &'static [u8], TextPos),

    /// An unexpected character instead of `"` or `'`.
    InvalidQuote(u8, TextPos),

    /// An unexpected character instead of an XML space.
    ///
    /// Includes: `' ' \n \r \t &#x20; &#x9; &#xD; &#xA;`.
    InvalidSpace(u8, TextPos),

    /// An unexpected string.
    ///
    /// Contains what string was expected.
    InvalidString(&'static str, TextPos),

    /// An invalid reference.
    InvalidReference,

    /// An invalid ExternalID in the DTD.
    InvalidExternalID,

    /// Comment cannot contain `--`.
    InvalidCommentData,

    /// Comment cannot end with `-`.
    InvalidCommentEnd,

    /// A Character Data node contains an invalid data.
    ///
    /// Currently, only `]]>` is not allowed.
    InvalidCharacterData,
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StreamError::UnexpectedEndOfStream => {
                write!(f, "unexpected end of stream")
            }
            StreamError::InvalidName => {
                write!(f, "invalid name token")
            }
            StreamError::NonXmlChar(c, pos) => {
                write!(f, "a non-XML character {:?} found at {}", c, pos)
            }
            StreamError::InvalidChar(actual, expected, pos) => {
                write!(f, "expected '{}' not '{}' at {}",
                       expected as char, actual as char, pos)
            }
            StreamError::InvalidCharMultiple(actual, ref expected, pos) => {
                let mut expected_iter = expected.iter().peekable();

                write!(f, "expected ")?;
                while let Some(&c) = expected_iter.next() {
                    write!(f, "'{}'", c as char)?;
                    if expected_iter.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, " not '{}' at {}", actual as char, pos)
            }
            StreamError::InvalidQuote(c, pos) => {
                write!(f, "expected quote mark not '{}' at {}", c as char, pos)
            }
            StreamError::InvalidSpace(c, pos) => {
                write!(f, "expected space not '{}' at {}", c as char, pos)
            }
            StreamError::InvalidString(expected, pos) => {
                write!(f, "expected '{}' at {}", expected, pos)
            }
            StreamError::InvalidReference => {
                write!(f, "invalid reference")
            }
            StreamError::InvalidExternalID => {
                write!(f, "invalid ExternalID")
            }
            StreamError::InvalidCommentData => {
                write!(f, "'--' is not allowed in comments")
            }
            StreamError::InvalidCommentEnd => {
                write!(f, "comment cannot end with '-'")
            }
            StreamError::InvalidCharacterData => {
                write!(f, "']]>' is not allowed inside a character data")
            }
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for StreamError {
    fn description(&self) -> &str {
        "an XML stream parsing error"
    }
}


/// Position in text.
///
/// Position indicates a row/line and a column in the original text. Starting from 1:1.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[allow(missing_docs)]
pub struct TextPos {
    pub row: u32,
    pub col: u32,
}

impl TextPos {
    /// Constructs a new `TextPos`.
    ///
    /// Should not be invoked manually, but rather via `Stream::gen_text_pos`.
    pub fn new(row: u32, col: u32) -> TextPos {
        TextPos { row, col }
    }
}

impl fmt::Display for TextPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}
