//! Parsing encodings from their string representation.
#![deny(unsafe_code)]
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

use crate::helper::{ContainerKind, EncodingType, Helper, NestingLevel, Primitive};
use crate::{Encoding, EncodingBox};

/// Check whether a struct or union name is a valid identifier
pub(crate) const fn verify_name(name: &str) -> bool {
    let bytes = name.as_bytes();

    if let b"?" = bytes {
        return true;
    }

    if bytes.is_empty() {
        return false;
    }

    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        if !(byte.is_ascii_alphanumeric() || byte == b'_') {
            return false;
        }
        i += 1;
    }
    true
}

/// The error that was encountered while parsing an encoding string.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ParseError {
    kind: ErrorKind,
    data: String,
    split_point: usize,
}

impl ParseError {
    pub(crate) fn new(parser: Parser<'_>, kind: ErrorKind) -> Self {
        Self {
            kind,
            data: parser.data.to_string(),
            split_point: parser.split_point,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed parsing encoding: {} at byte-index {} in {:?}",
            self.kind, self.split_point, self.data,
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum ErrorKind {
    UnexpectedEnd,
    Unknown(u8),
    UnknownAfterComplex(u8),
    ExpectedInteger,
    IntegerTooLarge,
    WrongEndArray,
    WrongEndContainer(ContainerKind),
    InvalidIdentifier(ContainerKind),
    NotAllConsumed,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd => write!(f, "unexpected end"),
            Self::Unknown(b) => {
                write!(f, "unknown encoding character {}", *b as char)
            }
            Self::UnknownAfterComplex(b) => {
                write!(f, "unknown encoding character {} after complex", *b as char,)
            }
            Self::ExpectedInteger => write!(f, "expected integer"),
            Self::IntegerTooLarge => write!(f, "integer too large"),
            Self::WrongEndArray => write!(f, "expected array to be closed"),
            Self::WrongEndContainer(kind) => {
                write!(f, "expected {kind} to be closed")
            }
            Self::InvalidIdentifier(kind) => {
                write!(f, "got invalid identifier in {kind}")
            }
            Self::NotAllConsumed => {
                write!(f, "remaining contents after parsing")
            }
        }
    }
}

type Result<T, E = ErrorKind> = core::result::Result<T, E>;

enum ParseInner {
    Empty,
    Encoding(EncodingBox),
    ContainerEnd(ContainerKind),
    ArrayEnd,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct Parser<'a> {
    data: &'a str,
    // Always "behind"/"at" the current character
    split_point: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(data: &'a str) -> Self {
        Self {
            split_point: 0,
            data,
        }
    }

    pub(crate) fn remaining(&self) -> &'a str {
        &self.data[self.split_point..]
    }

    fn peek(&self) -> Result<u8> {
        self.try_peek().ok_or(ErrorKind::UnexpectedEnd)
    }

    fn try_peek(&self) -> Option<u8> {
        self.data.as_bytes().get(self.split_point).copied()
    }

    fn try_peek2(&self) -> Option<(u8, u8)> {
        let bytes = self.data.as_bytes();
        Some((
            *bytes.get(self.split_point)?,
            *bytes.get(self.split_point + 1)?,
        ))
    }

    fn advance(&mut self) {
        self.split_point += 1;
    }

    fn rollback(&mut self) {
        self.split_point -= 1;
    }

    fn consume_while(&mut self, mut condition: impl FnMut(u8) -> bool) {
        while let Some(b) = self.try_peek() {
            if condition(b) {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.try_peek().is_none()
    }

    pub(crate) fn expect_empty(&self) -> Result<()> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(ErrorKind::NotAllConsumed)
        }
    }
}

impl Parser<'_> {
    /// Strip leading qualifiers, if any.
    pub(crate) fn strip_leading_qualifiers(&mut self) {
        // TODO: Add API for accessing and outputting qualifiers.
        #[allow(clippy::byte_char_slices)]
        const QUALIFIERS: &[u8] = &[
            b'r', // const
            b'n', // in
            b'N', // inout
            b'o', // out
            b'O', // bycopy
            b'R', // byref
            b'V', // oneway
        ];
        // TODO: b'|', // GCINVISIBLE

        self.consume_while(|b| QUALIFIERS.contains(&b));
    }

    /// Chomp until we hit a non-digit.
    ///
    /// + and - prefixes are not supported.
    fn chomp_digits(&mut self) -> Result<&str> {
        let old_split_point = self.split_point;

        // Parse first digit (which must be present).
        if !self.peek()?.is_ascii_digit() {
            return Err(ErrorKind::ExpectedInteger);
        }

        // Parse the rest, stopping if we hit a non-digit.
        self.consume_while(|b| b.is_ascii_digit());

        Ok(&self.data[old_split_point..self.split_point])
    }

    fn parse_u64(&mut self) -> Result<u64> {
        self.chomp_digits()?
            .parse()
            .map_err(|_| ErrorKind::IntegerTooLarge)
    }

    fn parse_u8(&mut self) -> Result<u8> {
        self.chomp_digits()?
            .parse()
            .map_err(|_| ErrorKind::IntegerTooLarge)
    }
}

/// Check if the data matches an expected value.
///
/// The errors here aren't currently used, so they're hackily set up.
impl Parser<'_> {
    fn expect_byte(&mut self, byte: u8) -> Option<()> {
        if self.try_peek()? == byte {
            self.advance();
            Some(())
        } else {
            None
        }
    }

    fn expect_one_of_str<'a>(&mut self, strings: impl IntoIterator<Item = &'a str>) -> Option<()> {
        for s in strings {
            if self.remaining().starts_with(s) {
                for b in s.as_bytes() {
                    self.expect_byte(*b).unwrap();
                }
                return Some(());
            }
        }
        None
    }

    fn expect_u64(&mut self, int: u64) -> Option<()> {
        if self.parse_u64().ok()? == int {
            Some(())
        } else {
            None
        }
    }

    fn expect_u8(&mut self, int: u8) -> Option<()> {
        if self.parse_u8().ok()? == int {
            Some(())
        } else {
            None
        }
    }

    pub(crate) fn expect_encoding(&mut self, enc: &Encoding, level: NestingLevel) -> Option<()> {
        match enc.helper() {
            Helper::Primitive(primitive) => {
                self.expect_one_of_str(primitive.equivalents().iter().map(|p| p.to_str()))?;

                if primitive == Primitive::Object && self.try_peek() == Some(b'"') {
                    self.advance();
                    self.consume_while(|b| b != b'"');
                    self.expect_byte(b'"')?;
                }
                Some(())
            }
            Helper::BitField(size, Some((offset, t))) => {
                self.expect_byte(b'b')?;
                self.expect_u64(*offset)?;
                self.expect_encoding(t, level.bitfield())?;
                self.expect_u8(size)
            }
            Helper::BitField(size, None) => {
                self.expect_byte(b'b')?;
                self.expect_u8(size)
            }
            Helper::Indirection(kind, t) => {
                self.expect_byte(kind.prefix_byte())?;
                self.expect_encoding(t, level.indirection(kind))
            }
            Helper::Array(len, item) => {
                self.expect_byte(b'[')?;
                self.expect_u64(len)?;
                self.expect_encoding(item, level.array())?;
                self.expect_byte(b']')
            }
            Helper::Container(kind, name, items) => {
                self.expect_byte(kind.start_byte())?;
                self.expect_one_of_str([name])?;
                if let Some(level) = level.container_include_fields() {
                    self.expect_byte(b'=')?;
                    // Parse as equal if the container is empty
                    if items.is_empty() {
                        loop {
                            match self.parse_inner().ok()? {
                                ParseInner::Empty => {
                                    // Require the container to have an end
                                    return None;
                                }
                                ParseInner::Encoding(_) => {}
                                ParseInner::ContainerEnd(parsed_kind) => {
                                    if parsed_kind == kind {
                                        return Some(());
                                    } else {
                                        return None;
                                    }
                                }
                                ParseInner::ArrayEnd => {
                                    return None;
                                }
                            }
                        }
                    }
                    // Parse as equal if the string's container is empty
                    if self.try_peek() == Some(kind.end_byte()) {
                        self.advance();
                        return Some(());
                    }
                    for item in items {
                        self.expect_encoding(item, level)?;
                    }
                }
                self.expect_byte(kind.end_byte())
            }
            Helper::NoneInvalid => Some(()),
        }
    }
}

impl Parser<'_> {
    fn parse_container(&mut self, kind: ContainerKind) -> Result<(&str, Vec<EncodingBox>)> {
        let old_split_point = self.split_point;

        // Parse name until hits `=` or `}`/`)`
        let has_items = loop {
            let b = self.try_peek().ok_or(ErrorKind::WrongEndContainer(kind))?;
            if b == b'=' {
                break true;
            } else if b == kind.end_byte() {
                break false;
            }
            self.advance();
        };

        let s = &self.data[old_split_point..self.split_point];

        if !verify_name(s) {
            return Err(ErrorKind::InvalidIdentifier(kind));
        }

        if has_items {
            self.advance();
        }

        let mut items = Vec::new();
        // Parse items until hits end
        loop {
            match self.parse_inner()? {
                ParseInner::Empty => {
                    return Err(ErrorKind::WrongEndContainer(kind));
                }
                ParseInner::Encoding(enc) => {
                    items.push(enc);
                }
                ParseInner::ContainerEnd(parsed_kind) => {
                    if parsed_kind == kind {
                        return Ok((s, items));
                    } else {
                        return Err(ErrorKind::Unknown(parsed_kind.end_byte()));
                    }
                }
                ParseInner::ArrayEnd => {
                    return Err(ErrorKind::Unknown(b']'));
                }
            }
        }
    }

    pub(crate) fn parse_encoding_or_none(&mut self) -> Result<EncodingBox> {
        match self.parse_inner()? {
            ParseInner::Empty => Ok(EncodingBox::None),
            ParseInner::Encoding(enc) => Ok(enc),
            ParseInner::ContainerEnd(kind) => Err(ErrorKind::Unknown(kind.end_byte())),
            ParseInner::ArrayEnd => Err(ErrorKind::Unknown(b']')),
        }
    }

    fn parse_inner(&mut self) -> Result<ParseInner> {
        if self.is_empty() {
            return Ok(ParseInner::Empty);
        }
        let b = self.peek()?;
        self.advance();

        Ok(ParseInner::Encoding(match b {
            b'c' => EncodingBox::Char,
            b's' => EncodingBox::Short,
            b'i' => EncodingBox::Int,
            b'l' => EncodingBox::Long,
            b'q' => EncodingBox::LongLong,
            b'C' => EncodingBox::UChar,
            b'S' => EncodingBox::UShort,
            b'I' => EncodingBox::UInt,
            b'L' => EncodingBox::ULong,
            b'Q' => EncodingBox::ULongLong,
            b'f' => EncodingBox::Float,
            b'd' => EncodingBox::Double,
            b'D' => EncodingBox::LongDouble,
            b'j' => {
                let res = match self.peek()? {
                    b'f' => EncodingBox::FloatComplex,
                    b'd' => EncodingBox::DoubleComplex,
                    b'D' => EncodingBox::LongDoubleComplex,
                    b => return Err(ErrorKind::UnknownAfterComplex(b)),
                };
                self.advance();
                res
            }
            b'B' => EncodingBox::Bool,
            b'v' => EncodingBox::Void,
            b'*' => EncodingBox::String,
            b'@' => match self.try_peek() {
                // Special handling for blocks
                Some(b'?') => {
                    self.advance();
                    EncodingBox::Block
                }
                // Parse class name if present
                Some(b'"') => {
                    self.advance();
                    self.consume_while(|b| b != b'"');
                    self.expect_byte(b'"').ok_or(ErrorKind::UnexpectedEnd)?;
                    EncodingBox::Object
                }
                _ => EncodingBox::Object,
            },
            b'#' => EncodingBox::Class,
            b':' => EncodingBox::Sel,
            b'?' => EncodingBox::Unknown,

            b'b' => {
                let size_or_offset = self.parse_u64()?;
                if let Some((size, ty)) = self.try_parse_bitfield_gnustep()? {
                    let offset = size_or_offset;
                    EncodingBox::BitField(size, Some(Box::new((offset, ty))))
                } else {
                    let size = size_or_offset
                        .try_into()
                        .map_err(|_| ErrorKind::IntegerTooLarge)?;
                    EncodingBox::BitField(size, None)
                }
            }
            b'^' => EncodingBox::Pointer(Box::new(match self.parse_inner()? {
                ParseInner::Empty => EncodingBox::None,
                ParseInner::Encoding(enc) => enc,
                ParseInner::ContainerEnd(_) | ParseInner::ArrayEnd => {
                    self.rollback();
                    EncodingBox::None
                }
            })),
            b'A' => EncodingBox::Atomic(Box::new(match self.parse_inner()? {
                ParseInner::Empty => EncodingBox::None,
                ParseInner::Encoding(enc) => enc,
                ParseInner::ContainerEnd(_) | ParseInner::ArrayEnd => {
                    self.rollback();
                    EncodingBox::None
                }
            })),
            b'[' => {
                let len = self.parse_u64()?;
                match self.parse_inner()? {
                    ParseInner::Empty => {
                        return Err(ErrorKind::WrongEndArray);
                    }
                    ParseInner::Encoding(item) => {
                        self.expect_byte(b']').ok_or(ErrorKind::WrongEndArray)?;
                        EncodingBox::Array(len, Box::new(item))
                    }
                    ParseInner::ArrayEnd => EncodingBox::Array(len, Box::new(EncodingBox::None)),
                    ParseInner::ContainerEnd(kind) => {
                        return Err(ErrorKind::Unknown(kind.end_byte()))
                    }
                }
            }
            b']' => {
                return Ok(ParseInner::ArrayEnd);
            }
            b'{' => {
                let kind = ContainerKind::Struct;
                let (name, items) = self.parse_container(kind)?;
                EncodingBox::Struct(name.to_string(), items)
            }
            b'}' => {
                return Ok(ParseInner::ContainerEnd(ContainerKind::Struct));
            }
            b'(' => {
                let kind = ContainerKind::Union;
                let (name, items) = self.parse_container(kind)?;
                EncodingBox::Union(name.to_string(), items)
            }
            b')' => {
                return Ok(ParseInner::ContainerEnd(ContainerKind::Union));
            }
            b => return Err(ErrorKind::Unknown(b)),
        }))
    }

    fn try_parse_bitfield_gnustep(&mut self) -> Result<Option<(u8, EncodingBox)>> {
        if let Some((b1, b2)) = self.try_peek2() {
            // Try to parse the encoding.
            //
            // The encoding is always an integral type.
            let ty = match b1 {
                b'c' => EncodingBox::Char,
                b's' => EncodingBox::Short,
                b'i' => EncodingBox::Int,
                b'l' => EncodingBox::Long,
                b'q' => EncodingBox::LongLong,
                b'C' => EncodingBox::UChar,
                b'S' => EncodingBox::UShort,
                b'I' => EncodingBox::UInt,
                b'L' => EncodingBox::ULong,
                b'Q' => EncodingBox::ULongLong,
                b'B' => EncodingBox::Bool,
                _ => return Ok(None),
            };
            // And then check if a digit follows that (which the size would
            // always contain).
            if !b2.is_ascii_digit() {
                return Ok(None);
            }
            // We have a size; so let's advance...
            self.advance();
            // ...and parse it for real.
            let size = self.parse_u8()?;
            Ok(Some((size, ty)))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn parse_container() {
        const KIND: ContainerKind = ContainerKind::Struct;

        #[track_caller]
        fn assert_name(enc: &str, expected: Result<(&str, Vec<EncodingBox>)>) {
            let mut parser = Parser::new(enc);
            assert_eq!(parser.parse_container(KIND), expected);
        }

        assert_name("abc=}", Ok(("abc", vec![])));
        assert_name(
            "abc=ii}",
            Ok(("abc", vec![EncodingBox::Int, EncodingBox::Int])),
        );
        assert_name("_=}.a'", Ok(("_", vec![])));
        assert_name("abc}def", Ok(("abc", vec![])));
        assert_name("=def}", Err(ErrorKind::InvalidIdentifier(KIND)));
        assert_name(".=def}", Err(ErrorKind::InvalidIdentifier(KIND)));
        assert_name("}xyz", Err(ErrorKind::InvalidIdentifier(KIND)));
        assert_name("", Err(ErrorKind::WrongEndContainer(KIND)));
        assert_name("abc", Err(ErrorKind::WrongEndContainer(KIND)));
        assert_name("abc)def", Err(ErrorKind::WrongEndContainer(KIND)));
    }

    #[test]
    fn parse_bitfield() {
        #[track_caller]
        fn assert_bitfield(enc: &str, expected: Result<EncodingBox>) {
            let mut parser = Parser::new(enc);
            assert_eq!(
                parser
                    .parse_encoding_or_none()
                    .and_then(|enc| parser.expect_empty().map(|()| enc)),
                expected
            );
        }

        assert_bitfield("b8", Ok(EncodingBox::BitField(8, None)));
        assert_bitfield("b8C", Err(ErrorKind::NotAllConsumed));
        assert_bitfield(
            "b8C4",
            Ok(EncodingBox::BitField(
                4,
                Some(Box::new((8, EncodingBox::UChar))),
            )),
        );

        assert_bitfield(
            "{s=b8C}",
            Ok(EncodingBox::Struct(
                "s".into(),
                vec![EncodingBox::BitField(8, None), EncodingBox::UChar],
            )),
        );

        assert_bitfield("b2000", Err(ErrorKind::IntegerTooLarge));
        assert_bitfield(
            "b2000c100",
            Ok(EncodingBox::BitField(
                100,
                Some(Box::new((2000, EncodingBox::Char))),
            )),
        );
        assert_bitfield("b2000C257", Err(ErrorKind::IntegerTooLarge));
    }

    #[test]
    fn parse_closing() {
        let mut parser = Parser::new("]");
        assert_eq!(
            parser.parse_encoding_or_none(),
            Err(ErrorKind::Unknown(b']'))
        );
    }
}
