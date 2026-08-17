/// Ascii property lists are used in legacy settings and only support four
/// datatypes: Array, Dictionary, String and Data.
/// See [Apple
/// Documentation](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/PropertyLists/OldStylePlists/OldStylePLists.html)
/// for more info.
/// However this reader also support Integers as first class datatype.
/// This reader will accept certain ill-formed ascii plist without complaining.
/// It does not check the integrity of the plist format.
use crate::{
    error::{Error, ErrorKind},
    stream::{Event, OwnedEvent},
    Integer,
};
use std::io::Read;

pub struct AsciiReader<R: Read> {
    reader: R,
    current_pos: u64,

    /// lookahead char to avoid backtracking.
    peeked_char: Option<u8>,

    current_char: Option<u8>,
}

impl<R: Read> AsciiReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            current_pos: 0,
            peeked_char: None,
            current_char: None,
        }
    }

    pub fn into_inner(self) -> R {
        self.reader
    }

    fn error(&self, kind: ErrorKind) -> Error {
        kind.with_byte_offset(self.current_pos)
    }

    fn read_one(&mut self) -> Result<Option<u8>, Error> {
        let mut buf: [u8; 1] = [0; 1];
        match self.reader.read_exact(&mut buf) {
            Ok(()) => Ok(Some(buf[0])),
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => Ok(None),
            Err(err) => Err(self.error(ErrorKind::Io(err))),
        }
    }

    /// Consume the reader and set [`Self::current_char`] and
    /// [`Self::peeked_char`]. Returns the current character.
    fn advance(&mut self) -> Result<Option<u8>, Error> {
        self.current_char = self.peeked_char;
        self.peeked_char = self.read_one()?;

        // We need to read two chars to boot the process and fill the peeked
        // char.
        if self.current_pos == 0 {
            self.current_char = self.peeked_char;
            self.peeked_char = self.read_one()?;
        }

        if self.current_char.is_some() {
            self.current_pos += 1;
        }

        Ok(self.current_char)
    }

    /// From Apple doc:
    ///
    /// > The quotation marks can be omitted if the string is composed strictly of alphanumeric
    /// > characters and contains no white space (numbers are handled as
    /// > strings in property lists). Though the property list format uses
    /// > ASCII for strings, note that Cocoa uses Unicode. Since string
    /// > encodings vary from region to region, this representation makes the
    /// > format fragile. You may see strings containing unreadable sequences of
    /// > ASCII characters; these are used to represent Unicode characters
    ///
    /// This function will naively try to convert the string to Integer.
    fn unquoted_string_literal(&mut self, first: u8) -> Result<Option<OwnedEvent>, Error> {
        let mut acc: Vec<u8> = Vec::new();
        acc.push(first);

        while {
            match self.peeked_char {
                Some(c) => {
                    c != b' ' && c != b')' && c != b'\r' && c != b'\t' && c != b';' && c != b','
                }
                None => false,
            }
        } {
            // consuming the string itself
            self.advance()?;
            match self.current_char {
                Some(c) => acc.push(c),
                None => return Err(self.error(ErrorKind::UnclosedString)),
            }
        }

        let string_literal =
            String::from_utf8(acc).map_err(|_e| self.error(ErrorKind::InvalidUtf8AsciiStream))?;

        // Not ideal but does the trick for now
        match Integer::from_str(&string_literal) {
            Ok(i) => Ok(Some(Event::Integer(i))),
            Err(_) => Ok(Some(Event::String(string_literal.into()))),
        }
    }

    /// The process for decoding utf-16 escapes to utf-8 is:
    /// 1. Convert the 4 hex characters to utf-16 code units (u16s).
    ///    '\u006d' becomes 0x6d.
    /// 2. Based on the first code unit, determine whether another code unit is
    ///    required to form the complete code point.
    ///    "\uD83D\uDCA9" becomes `[0xd73d, 0xdca9]`
    /// 3. Convert the 1 or 2 u16 code point to utf-8.
    ///    `[0xd73d, 0xdca9]` becomes '💩'.
    ///
    /// The standard library has some useful functions behind unstable feature
    /// flags, we can simplify and optimize this a bit once they're stable.
    /// - `str_from_utf16_endian`
    /// - `is_utf16_surrogate`
    fn utf16_escape(&mut self) -> Result<String, Error> {
        let mut code_units: &mut [u16] = &mut [0u16; 2];

        let Some(code_unit) = self.utf16_code_unit()? else {
            return Err(self.error(ErrorKind::InvalidUtf16String));
        };

        code_units[0] = code_unit;

        // This is the utf-16 surrogate range, indicating another code unit is
        // necessary to form a complete code point.
        if matches!(code_unit, 0xD800..=0xDFFF) {
            self.advance_quoted_string()?;

            if self.current_char != Some(b'\\')
                || !matches!(self.peeked_char, Some(b'u' | b'U'))
            {
                return Err(self.error(ErrorKind::InvalidUtf16String));
            }

            self.advance_quoted_string()?;

            if let Some(code_unit) = self.utf16_code_unit()? {
                code_units[1] = code_unit;
            }
        } else {
            code_units = &mut code_units[0..1];
        }

        let utf8 = String::from_utf16(code_units)
            .map_err(|_| self.error(ErrorKind::InvalidUtf16String))?;

        Ok(utf8)
    }

    /// Expects the reader's next read to return the first hex character of the
    /// utf-16 hex string.
    fn utf16_code_unit(&mut self) -> Result<Option<u16>, Error> {
        let hex_chars = [
            self.advance_quoted_string()?,
            self.advance_quoted_string()?,
            self.advance_quoted_string()?,
            self.advance_quoted_string()?,
        ];

        let hex_str = std::str::from_utf8(&hex_chars)
            .map_err(|_| self.error(ErrorKind::InvalidUtf16String))?;

        let code_unit = u16::from_str_radix(hex_str, 16)
            .map_err(|_| self.error(ErrorKind::InvalidUtf16String))?;

        Ok(Some(code_unit))
    }

    #[inline]
    fn advance_quoted_string(&mut self) -> Result<u8, Error> {
        match self.advance()? {
            Some(c) => Ok(c),
            None => Err(self.error(ErrorKind::UnclosedString)),
        }
    }

    fn quoted_string_literal(&mut self, quote: u8) -> Result<Option<OwnedEvent>, Error> {
        let mut acc = String::new();

        loop {
            let c = self.advance_quoted_string()?;

            if c == quote {
                return Ok(Some(Event::String(acc.into())));
            }

            let replacement = if c == b'\\' {
                let c = self.advance_quoted_string()?;

                match c {
                    b'\\' | b'"' => c as char,
                    b'a' => '\u{7}',
                    b'b' => '\u{8}',
                    b'f' => '\u{c}',
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'U' => {
                        let utf8 = self.utf16_escape()?;
                        acc.push_str(utf8.as_str());
                        continue;
                    }
                    b'v' => '\u{b}',
                    b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' => {
                        let value = [
                            c,
                            self.advance_quoted_string()?,
                            self.advance_quoted_string()?,
                        ];

                        let value = std::str::from_utf8(&value)
                            .map_err(|_| self.error(ErrorKind::InvalidOctalString))?;

                        let value = u32::from(u16::from_str_radix(value, 8)
                            .map_err(|_| self.error(ErrorKind::InvalidOctalString))?);

                        let value = char::from_u32(value)
                            .ok_or(self.error(ErrorKind::InvalidOctalString))?;

                        map_next_step_to_unicode(value)
                    }
                    _ => return Err(self.error(ErrorKind::InvalidUtf8AsciiStream)),
                }
            } else {
                c as char
            };

            acc.push(replacement);
        }
    }

    fn line_comment(&mut self) -> Result<(), Error> {
        // Consumes up to the end of the line.
        // There's no error in this a line comment can reach the EOF and there's
        // no forbidden chars in comments.
        while {
            match self.peeked_char {
                Some(c) => c != b'\n',
                None => false,
            }
        } {
            let _ = self.advance()?;
        }

        Ok(())
    }

    fn block_comment(&mut self) -> Result<(), Error> {
        let mut latest_consume = b' ';
        while {
            latest_consume != b'*'
                || match self.advance()? {
                    Some(c) => c != b'/',
                    None => false,
                }
        } {
            latest_consume = self
                .advance()?
                .ok_or(self.error(ErrorKind::IncompleteComment))?;
        }

        Ok(())
    }

    /// Returns:
    /// - Some(string) if '/' was the first character of a string
    /// - None if '/' was the beginning of a comment.
    fn potential_comment(&mut self) -> Result<Option<OwnedEvent>, Error> {
        match self.peeked_char {
            Some(c) => match c {
                b'/' => self.line_comment().map(|()| None),
                b'*' => self.block_comment().map(|()| None),
                _ => self.unquoted_string_literal(c),
            },
            // EOF
            None => Err(self.error(ErrorKind::IncompleteComment)),
        }
    }

    /// Consumes the reader until it finds a valid Event
    /// Possible events for Ascii plists:
    ///  - `StartArray(Option<u64>)`,
    ///  - `StartDictionary(Option<u64>)`,
    ///  - `EndCollection`,
    ///  - `Data(Vec<u8>)`,
    fn read_next(&mut self) -> Result<Option<OwnedEvent>, Error> {
        while let Some(c) = self.advance()? {
            match c {
                // Single char tokens
                b'(' => return Ok(Some(Event::StartArray(None))),
                b'{' => return Ok(Some(Event::StartDictionary(None))),
                b')' | b'}' => return Ok(Some(Event::EndCollection)),
                b'\'' | b'"' => return self.quoted_string_literal(c),
                b'/' => {
                    match self.potential_comment() {
                        Ok(Some(event)) => return Ok(Some(event)),
                        Ok(None) => { /* Comment has been consumed */ }
                        Err(e) => return Err(e),
                    }
                }
                b',' | b';' | b'=' => { /* consume these without emitting anything */ }
                b' ' | b'\r' | b'\t' | b'\n' => { /* whitespace is not significant */ }
                _ => return self.unquoted_string_literal(c),
            }
        }

        Ok(None)
    }
}

impl<R: Read> Iterator for AsciiReader<R> {
    type Item = Result<OwnedEvent, Error>;

    fn next(&mut self) -> Option<Result<OwnedEvent, Error>> {
        self.read_next().transpose()
    }
}

/// Maps NeXTSTEP encoding to Unicode, see:
/// - <https://github.com/fonttools/openstep-plist/blob/master/src/openstep_plist/parser.pyx#L87-L106>
/// - <ftp://ftp.unicode.org/Public/MAPPINGS/VENDORS/NEXT/NEXTSTEP.TXT>
fn map_next_step_to_unicode(c: char) -> char {
    const NEXT_UNICODE_MAPPING: &[char] = &[
        '\u{A0}', '\u{C0}', '\u{C1}', '\u{C2}', '\u{C3}', '\u{C4}', '\u{C5}', '\u{C7}', '\u{C8}',
        '\u{C9}', '\u{CA}', '\u{CB}', '\u{CC}', '\u{CD}', '\u{CE}', '\u{CF}', '\u{D0}', '\u{D1}',
        '\u{D2}', '\u{D3}', '\u{D4}', '\u{D5}', '\u{D6}', '\u{D9}', '\u{DA}', '\u{DB}', '\u{DC}',
        '\u{DD}', '\u{DE}', '\u{B5}', '\u{D7}', '\u{F7}', '\u{A9}', '\u{A1}', '\u{A2}', '\u{A3}',
        '\u{2044}', '\u{A5}', '\u{192}', '\u{A7}', '\u{A4}', '\u{2019}', '\u{201C}', '\u{AB}',
        '\u{2039}', '\u{203A}', '\u{FB01}', '\u{FB02}', '\u{AE}', '\u{2013}', '\u{2020}',
        '\u{2021}', '\u{B7}', '\u{A6}', '\u{B6}', '\u{2022}', '\u{201A}', '\u{201E}', '\u{201D}',
        '\u{BB}', '\u{2026}', '\u{2030}', '\u{AC}', '\u{BF}', '\u{B9}', '\u{2CB}', '\u{B4}',
        '\u{2C6}', '\u{2DC}', '\u{AF}', '\u{2D8}', '\u{2D9}', '\u{A8}', '\u{B2}', '\u{2DA}',
        '\u{B8}', '\u{B3}', '\u{2DD}', '\u{2DB}', '\u{2C7}', '\u{2014}', '\u{B1}', '\u{BC}',
        '\u{BD}', '\u{BE}', '\u{E0}', '\u{E1}', '\u{E2}', '\u{E3}', '\u{E4}', '\u{E5}', '\u{E7}',
        '\u{E8}', '\u{E9}', '\u{EA}', '\u{EB}', '\u{EC}', '\u{C6}', '\u{ED}', '\u{AA}', '\u{EE}',
        '\u{EF}', '\u{F0}', '\u{F1}', '\u{141}', '\u{D8}', '\u{152}', '\u{BA}', '\u{F2}', '\u{F3}',
        '\u{F4}', '\u{F5}', '\u{F6}', '\u{E6}', '\u{F9}', '\u{FA}', '\u{FB}', '\u{131}', '\u{FC}',
        '\u{FD}', '\u{142}', '\u{F8}', '\u{153}', '\u{DF}', '\u{FE}', '\u{FF}', '\u{FFFD}',
        '\u{FFFD}',
    ];

    let index = c as usize;

    if index < 128 || index > 0xff {
        return c;
    }

    NEXT_UNICODE_MAPPING[index - 128]
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
    use crate::stream::Event::*;

    #[test]
    fn empty_test() {
        let plist = "";
        let streaming_parser = AsciiReader::new(plist.as_bytes());
        let events: Vec<Event> = streaming_parser.map(|e| e.unwrap()).collect();
        assert_eq!(events, &[]);
    }

    #[test]
    fn streaming_sample() {
        let reader = File::open("./tests/data/ascii-sample.plist").unwrap();
        let streaming_parser = AsciiReader::new(reader);
        let events: Vec<Event> = streaming_parser.map(|e| e.unwrap()).collect();

        let comparison = &[
            StartDictionary(None),
            String("KeyName1".into()),
            String("Value1".into()),
            String("AnotherKeyName".into()),
            String("Value2".into()),
            String("Something".into()),
            StartArray(None),
            String("ArrayItem1".into()),
            String("ArrayItem2".into()),
            String("ArrayItem3".into()),
            EndCollection,
            String("Key4".into()),
            String("0.10".into()),
            String("KeyFive".into()),
            StartDictionary(None),
            String("Dictionary2Key1".into()),
            String("Something".into()),
            String("AnotherKey".into()),
            String("Somethingelse".into()),
            EndCollection,
            EndCollection,
        ];

        assert_eq!(events, comparison);
    }

    #[test]
    fn utf8_strings() {
        let plist = "{ names = (Léa, François, Żaklina, 王芳); }";
        let streaming_parser = AsciiReader::new(plist.as_bytes());
        let events: Vec<Event> = streaming_parser.map(|e| e.unwrap()).collect();

        let comparison = &[
            StartDictionary(None),
            String("names".into()),
            StartArray(None),
            String("Léa".into()),
            String("François".into()),
            String("Żaklina".into()),
            String("王芳".into()),
            EndCollection,
            EndCollection,
        ];

        assert_eq!(events, comparison);
    }

    #[test]
    fn invalid_utf16_escapes() {
        let plist = br#"{
            key1 = "\U123";
            key2 = "\UD83D";
            key3 = "\u0080";
        }"#;
        let streaming_parser = AsciiReader::new(&plist[..]);
        let events: Vec<Result<Event, Error>> = streaming_parser.collect();

        // key1's value
        assert!(events[2].is_err());
        // key2's value
        assert!(events[4].is_err());
        // key3's value
        assert!(events[6].is_err());
    }

    #[test]
    fn invalid_octal_escapes() {
        let plist = br#"{
            key1 = "\1";
            key2 = "\12";
        }"#;
        let streaming_parser = AsciiReader::new(&plist[..]);
        let events: Vec<Result<Event, Error>> = streaming_parser.collect();

        // key1's value
        assert!(events[2].is_err());
        // key2's value
        assert!(events[4].is_err());
    }

    #[test]
    fn escaped_sequences_in_strings() {
        let plist = br#"{
            key1 = "va\"lue";
            key2 = 'va"lue';
            key3 = "va\a\b\f\n\r\t\v\"\nlue";
            key4 = "a\012b";
            key5 = "\\UD83D\\UDCA9";
            key6 = "\UD83D\UDCA9";
            key7 = "\U0080";
            key8 = "\200\377";
        }"#;
        let streaming_parser = AsciiReader::new(&plist[..]);
        let events: Vec<Event> = streaming_parser.map(|e| e.unwrap()).collect();

        let comparison = &[
            StartDictionary(None),
            String("key1".into()),
            String(r#"va"lue"#.into()),
            String("key2".into()),
            String(r#"va"lue"#.into()),
            String("key3".into()),
            String("va\u{7}\u{8}\u{c}\n\r\t\u{b}\"\nlue".into()),
            String("key4".into()),
            String("a\nb".into()),
            String("key5".into()),
            String("\\UD83D\\UDCA9".into()),
            String("key6".into()),
            String("💩".into()),
            String("key7".into()),
            String("\u{80}".into()),
            String("key8".into()),
            String("\u{a0}\u{fffd}".into()),
            EndCollection,
        ];

        assert_eq!(events, comparison);
    }

    #[test]
    fn integers_and_strings() {
        let plist = b"{ name = James, age = 42 }";
        let streaming_parser = AsciiReader::new(&plist[..]);
        let events: Vec<Event> = streaming_parser.map(|e| e.unwrap()).collect();

        let comparison = &[
            StartDictionary(None),
            String("name".into()),
            String("James".into()),
            String("age".into()),
            Integer(42.into()),
            EndCollection,
        ];

        assert_eq!(events, comparison);
    }

    #[test]
    fn netnewswire_pbxproj() {
        let reader = File::open("./tests/data/netnewswire.pbxproj").unwrap();
        let streaming_parser = AsciiReader::new(reader);

        // Ensure that we don't fail when reading the file
        let events: Vec<Event> = streaming_parser.map(|e| e.unwrap()).collect();

        assert!(!events.is_empty());
    }

    // TODO: This should return `Err` after the first string
    #[test]
    fn multiple_unquoted_strings() {
        let plist = b"not a plist";
        let streaming_parser = AsciiReader::new(&plist[..]);
        let events: Vec<Event> = streaming_parser.map(|e| e.unwrap()).collect();

        let comparison = &[
            String("not".into()),
            String("a".into()),
            String("plist".into()),
        ];

        assert_eq!(events, comparison);
    }
}
