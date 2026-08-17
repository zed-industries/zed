use core::ops::Range;
use core::str;

use crate::{Error, TextPos};

type Result<T> = core::result::Result<T, Error>;

/// Extension methods for XML-subset only operations.
trait XmlCharExt {
    /// Checks if the value is within the
    /// [NameStartChar](https://www.w3.org/TR/xml/#NT-NameStartChar) range.
    fn is_xml_name_start(&self) -> bool;

    /// Checks if the value is within the
    /// [NameChar](https://www.w3.org/TR/xml/#NT-NameChar) range.
    fn is_xml_name(&self) -> bool;

    /// Checks if the value is within the
    /// [Char](https://www.w3.org/TR/xml/#NT-Char) range.
    fn is_xml_char(&self) -> bool;
}

impl XmlCharExt for char {
    #[inline]
    fn is_xml_name_start(&self) -> bool {
        // Check for ASCII first.
        if *self as u32 <= 128 {
            return matches!(*self as u8, b'A'..=b'Z' | b'a'..=b'z' | b':' | b'_');
        }

        matches!(*self as u32,
            0x0000C0..=0x0000D6
            | 0x0000D8..=0x0000F6
            | 0x0000F8..=0x0002FF
            | 0x000370..=0x00037D
            | 0x00037F..=0x001FFF
            | 0x00200C..=0x00200D
            | 0x002070..=0x00218F
            | 0x002C00..=0x002FEF
            | 0x003001..=0x00D7FF
            | 0x00F900..=0x00FDCF
            | 0x00FDF0..=0x00FFFD
            | 0x010000..=0x0EFFFF)
    }

    #[inline]
    fn is_xml_name(&self) -> bool {
        // Check for ASCII first.
        if *self as u32 <= 128 {
            return (*self as u8).is_xml_name();
        }

        matches!(*self as u32, 0x0000B7
                | 0x0000C0..=0x0000D6
                | 0x0000D8..=0x0000F6
                | 0x0000F8..=0x0002FF
                | 0x000300..=0x00036F
                | 0x000370..=0x00037D
                | 0x00037F..=0x001FFF
                | 0x00200C..=0x00200D
                | 0x00203F..=0x002040
                | 0x002070..=0x00218F
                | 0x002C00..=0x002FEF
                | 0x003001..=0x00D7FF
                | 0x00F900..=0x00FDCF
                | 0x00FDF0..=0x00FFFD
                | 0x010000..=0x0EFFFF)
    }

    #[inline]
    fn is_xml_char(&self) -> bool {
        // Does not check for surrogate code points U+D800-U+DFFF,
        // since that check was performed by Rust when the `&str` was constructed.
        if (*self as u32) < 0x20 {
            return (*self as u8).is_xml_space();
        }

        !matches!(*self as u32, 0xFFFF | 0xFFFE)
    }
}

trait XmlByteExt {
    /// Checks if byte is a space.
    ///
    /// `[ \r\n\t]`
    fn is_xml_space(&self) -> bool;

    /// Checks if byte is within the ASCII
    /// [Char](https://www.w3.org/TR/xml/#NT-Char) range.
    fn is_xml_name(&self) -> bool;
}

impl XmlByteExt for u8 {
    #[inline]
    fn is_xml_space(&self) -> bool {
        matches!(*self, b' ' | b'\t' | b'\n' | b'\r')
    }

    #[inline]
    fn is_xml_name(&self) -> bool {
        matches!(*self, b'A'..=b'Z' | b'a'..=b'z'| b'0'..=b'9'| b':' | b'_' | b'-' | b'.')
    }
}

/// A string slice.
///
/// Like `&str`, but also contains the position in the input XML
/// from which it was parsed.
#[must_use]
#[derive(Clone, Copy)]
pub struct StrSpan<'input> {
    text: &'input str,
    start: usize,
}

impl<'input> From<&'input str> for StrSpan<'input> {
    #[inline]
    fn from(text: &'input str) -> Self {
        StrSpan { text, start: 0 }
    }
}

impl<'input> StrSpan<'input> {
    #[inline]
    pub fn from_substr(text: &str, start: usize, end: usize) -> StrSpan {
        debug_assert!(start <= end);
        StrSpan {
            text: &text[start..end],
            start,
        }
    }

    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.start..(self.start + self.text.len())
    }

    #[inline]
    pub fn as_str(&self) -> &'input str {
        self.text
    }

    #[inline]
    fn slice_region(&self, start: usize, end: usize) -> &'input str {
        &self.text[start..end]
    }
}

pub enum Token<'input> {
    // <?target content?>
    ProcessingInstruction(&'input str, Option<&'input str>, Range<usize>),

    // <!-- text -->
    Comment(&'input str, Range<usize>),

    // <!ENTITY ns_extend "http://test.com">
    EntityDeclaration(&'input str, StrSpan<'input>),

    // <ns:elem
    ElementStart(&'input str, &'input str, usize),

    // ns:attr="value"
    Attribute(Range<usize>, u16, u8, &'input str, &'input str, StrSpan<'input>),

    ElementEnd(ElementEnd<'input>, Range<usize>),

    // Contains text between elements including whitespaces.
    // Basically everything between `>` and `<`.
    // Except `]]>`, which is not allowed and will lead to an error.
    Text(&'input str, Range<usize>),

    // <![CDATA[text]]>
    Cdata(&'input str, Range<usize>),
}

/// `ElementEnd` token.
#[derive(Clone, Copy)]
pub enum ElementEnd<'input> {
    /// Indicates `>`
    Open,
    /// Indicates `</ns:name>`
    Close(&'input str, &'input str),
    /// Indicates `/>`
    Empty,
}

pub trait XmlEvents<'input> {
    fn token(&mut self, token: Token<'input>) -> Result<()>;
}

// document ::= prolog element Misc*
pub fn parse<'input>(
    text: &'input str,
    allow_dtd: bool,
    events: &mut dyn XmlEvents<'input>,
) -> Result<()> {
    let s = &mut Stream::new(text);

    // Skip UTF-8 BOM.
    if s.starts_with(&[0xEF, 0xBB, 0xBF]) {
        s.advance(3);
    }

    if s.starts_with(b"<?xml ") {
        parse_declaration(s)?;
    }

    parse_misc(s, events)?;

    s.skip_spaces();
    if s.starts_with(b"<!DOCTYPE") {
        if !allow_dtd {
            return Err(Error::DtdDetected);
        }

        parse_doctype(s, events)?;
        parse_misc(s, events)?;
    }

    s.skip_spaces();
    if s.curr_byte().ok() == Some(b'<') {
        parse_element(s, events)?;
    }

    parse_misc(s, events)?;

    if !s.at_end() {
        return Err(Error::UnknownToken(s.gen_text_pos()));
    }

    Ok(())
}

// Misc ::= Comment | PI | S
fn parse_misc<'input>(s: &mut Stream<'input>, events: &mut dyn XmlEvents<'input>) -> Result<()> {
    while !s.at_end() {
        s.skip_spaces();
        if s.starts_with(b"<!--") {
            parse_comment(s, events)?;
        } else if s.starts_with(b"<?") {
            parse_pi(s, events)?;
        } else {
            break;
        }
    }

    Ok(())
}

// XMLDecl ::= '<?xml' VersionInfo EncodingDecl? SDDecl? S? '?>'
//
// We don't actually return a token for the XML declaration and only validate it.
fn parse_declaration(s: &mut Stream) -> Result<()> {
    fn consume_spaces(s: &mut Stream) -> Result<()> {
        if s.starts_with_space() {
            s.skip_spaces();
        } else if !s.starts_with(b"?>") && !s.at_end() {
            return Err(Error::InvalidChar2(
                "a whitespace",
                s.curr_byte_unchecked(),
                s.gen_text_pos(),
            ));
        }

        Ok(())
    }

    s.advance(5); // <?xml
    consume_spaces(s)?;

    // The `version` "attribute" is mandatory.
    if !s.starts_with(b"version") {
        // Will trigger the InvalidString error, which is what we want.
        return s.skip_string(b"version");
    }
    let _ = parse_attribute(s)?;
    consume_spaces(s)?;

    if s.starts_with(b"encoding") {
        let _ = parse_attribute(s)?;
        consume_spaces(s)?;
    }

    if s.starts_with(b"standalone") {
        let _ = parse_attribute(s)?;
    }

    s.skip_spaces();
    s.skip_string(b"?>")?;

    Ok(())
}

// '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
fn parse_comment<'input>(s: &mut Stream<'input>, events: &mut dyn XmlEvents<'input>) -> Result<()> {
    let start = s.pos();
    s.advance(4);
    let text = s.consume_chars(|s, c| !(c == '-' && s.starts_with(b"-->")))?;
    s.skip_string(b"-->")?;

    if text.contains("--") {
        return Err(Error::InvalidComment(s.gen_text_pos_from(start)));
    }

    if text.ends_with('-') {
        return Err(Error::InvalidComment(s.gen_text_pos_from(start)));
    }

    let range = s.range_from(start);
    events.token(Token::Comment(text, range))?;

    Ok(())
}

// PI       ::= '<?' PITarget (S (Char* - (Char* '?>' Char*)))? '?>'
// PITarget ::= Name - (('X' | 'x') ('M' | 'm') ('L' | 'l'))
fn parse_pi<'input>(s: &mut Stream<'input>, events: &mut dyn XmlEvents<'input>) -> Result<()> {
    if s.starts_with(b"<?xml ") {
        return Err(Error::UnexpectedDeclaration(s.gen_text_pos()));
    }

    let start = s.pos();
    s.advance(2);
    let target = s.consume_name()?;
    s.skip_spaces();
    let content = s.consume_chars(|s, c| !(c == '?' && s.starts_with(b"?>")))?;
    let content = if !content.is_empty() {
        Some(content)
    } else {
        None
    };

    s.skip_string(b"?>")?;

    let range = s.range_from(start);
    events.token(Token::ProcessingInstruction(target, content, range))?;
    Ok(())
}

fn parse_doctype<'input>(s: &mut Stream<'input>, events: &mut dyn XmlEvents<'input>) -> Result<()> {
    let start = s.pos();
    parse_doctype_start(s)?;
    s.skip_spaces();

    if s.curr_byte() == Ok(b'>') {
        s.advance(1);
        return Ok(());
    }

    s.advance(1); // [
    while !s.at_end() {
        s.skip_spaces();
        if s.starts_with(b"<!ENTITY") {
            parse_entity_decl(s, events)?;
        } else if s.starts_with(b"<!--") {
            parse_comment(s, events)?;
        } else if s.starts_with(b"<?") {
            parse_pi(s, events)?;
        } else if s.starts_with(b"]") {
            // DTD ends with ']' S? '>', therefore we have to skip possible spaces.
            s.advance(1);
            s.skip_spaces();
            match s.curr_byte() {
                Ok(b'>') => {
                    s.advance(1);
                    break;
                }
                Ok(c) => {
                    return Err(Error::InvalidChar2("'>'", c, s.gen_text_pos()));
                }
                Err(_) => {
                    return Err(Error::UnexpectedEndOfStream);
                }
            }
        } else if s.starts_with(b"<!ELEMENT")
            || s.starts_with(b"<!ATTLIST")
            || s.starts_with(b"<!NOTATION")
        {
            if consume_decl(s).is_err() {
                let pos = s.gen_text_pos_from(start);
                return Err(Error::UnknownToken(pos));
            }
        } else {
            return Err(Error::UnknownToken(s.gen_text_pos()));
        }
    }

    Ok(())
}

// doctypedecl ::= '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
fn parse_doctype_start(s: &mut Stream) -> Result<()> {
    s.advance(9);

    s.consume_spaces()?;
    s.skip_name()?;
    s.skip_spaces();

    let _ = parse_external_id(s)?;
    s.skip_spaces();

    let c = s.curr_byte()?;
    if c != b'[' && c != b'>' {
        return Err(Error::InvalidChar2("'[' or '>'", c, s.gen_text_pos()));
    }

    Ok(())
}

// ExternalID ::= 'SYSTEM' S SystemLiteral | 'PUBLIC' S PubidLiteral S SystemLiteral
fn parse_external_id(s: &mut Stream) -> Result<bool> {
    let v = if s.starts_with(b"SYSTEM") || s.starts_with(b"PUBLIC") {
        let start = s.pos();
        s.advance(6);
        let id = s.slice_back(start);

        s.consume_spaces()?;
        let quote = s.consume_quote()?;
        let _ = s.consume_bytes(|c| c != quote);
        s.consume_byte(quote)?;

        if id == "SYSTEM" {
            // Ok
        } else {
            s.consume_spaces()?;
            let quote = s.consume_quote()?;
            let _ = s.consume_bytes(|c| c != quote);
            s.consume_byte(quote)?;
        }

        true
    } else {
        false
    };

    Ok(v)
}

// EntityDecl  ::= GEDecl | PEDecl
// GEDecl      ::= '<!ENTITY' S Name S EntityDef S? '>'
// PEDecl      ::= '<!ENTITY' S '%' S Name S PEDef S? '>'
fn parse_entity_decl<'input>(
    s: &mut Stream<'input>,
    events: &mut dyn XmlEvents<'input>,
) -> Result<()> {
    s.advance(8);
    s.consume_spaces()?;

    let is_ge = if s.try_consume_byte(b'%') {
        s.consume_spaces()?;
        false
    } else {
        true
    };

    let name = s.consume_name()?;
    s.consume_spaces()?;
    if let Some(definition) = parse_entity_def(s, is_ge)? {
        events.token(Token::EntityDeclaration(name, definition))?;
    }
    s.skip_spaces();
    s.consume_byte(b'>')?;

    Ok(())
}

// EntityDef   ::= EntityValue | (ExternalID NDataDecl?)
// PEDef       ::= EntityValue | ExternalID
// EntityValue ::= '"' ([^%&"] | PEReference | Reference)* '"' |  "'" ([^%&']
//                             | PEReference | Reference)* "'"
// ExternalID  ::= 'SYSTEM' S SystemLiteral | 'PUBLIC' S PubidLiteral S SystemLiteral
// NDataDecl   ::= S 'NDATA' S Name
fn parse_entity_def<'input>(
    s: &mut Stream<'input>,
    is_ge: bool,
) -> Result<Option<StrSpan<'input>>> {
    let c = s.curr_byte()?;
    match c {
        b'"' | b'\'' => {
            let quote = s.consume_quote()?;
            let start = s.pos();
            s.skip_bytes(|c| c != quote);
            let value = s.slice_back_span(start);
            s.consume_byte(quote)?;
            Ok(Some(value))
        }
        b'S' | b'P' => {
            if parse_external_id(s)? {
                if is_ge {
                    s.skip_spaces();
                    if s.starts_with(b"NDATA") {
                        s.advance(5);
                        s.consume_spaces()?;
                        s.skip_name()?;
                        // TODO: NDataDecl is not supported
                    }
                }

                Ok(None)
            } else {
                Err(Error::InvalidExternalID(s.gen_text_pos()))
            }
        }
        _ => {
            let pos = s.gen_text_pos();
            Err(Error::InvalidChar2("a quote, SYSTEM or PUBLIC", c, pos))
        }
    }
}

fn consume_decl(s: &mut Stream) -> Result<()> {
    s.skip_bytes(|c| c != b'>');
    s.consume_byte(b'>')?;
    Ok(())
}

// element ::= EmptyElemTag | STag content ETag
// '<' Name (S Attribute)* S? '>'
fn parse_element<'input>(s: &mut Stream<'input>, events: &mut dyn XmlEvents<'input>) -> Result<()> {
    let start = s.pos();
    s.advance(1); // <
    let (prefix, local) = s.consume_qname()?;
    events.token(Token::ElementStart(prefix, local, start))?;

    let mut open = false;
    while !s.at_end() {
        let has_space = s.starts_with_space();
        s.skip_spaces();
        let start = s.pos();
        match s.curr_byte()? {
            b'/' => {
                s.advance(1);
                s.consume_byte(b'>')?;
                let range = s.range_from(start);
                events.token(Token::ElementEnd(ElementEnd::Empty, range))?;
                break;
            }
            b'>' => {
                s.advance(1);
                let range = s.range_from(start);
                events.token(Token::ElementEnd(ElementEnd::Open, range))?;
                open = true;
                break;
            }
            _ => {
                // An attribute must be preceded with a whitespace.
                if !has_space {
                    // Will always trigger an error. Which is what we want.
                    s.consume_spaces()?;
                }

                // Manual inlining of `parse_attribute` for performance.
                // We cannot mark `parse_attribute` as `#[inline(always)]`
                // because it will blow up the binary size.
                let (prefix, local) = s.consume_qname()?;
                let qname_end = s.pos();
                let qname_len = u16::try_from(qname_end - start).unwrap_or(u16::MAX);
                s.consume_eq()?;
                let eq_len = u8::try_from(s.pos() - qname_end).unwrap_or(u8::MAX);
                let quote = s.consume_quote()?;
                let quote_c = quote as char;
                // The attribute value must not contain the < character.
                let value_start = s.pos();
                s.skip_chars(|_, c| c != quote_c && c != '<')?;
                let value = s.slice_back_span(value_start);
                s.consume_byte(quote)?;
                let end = s.pos();
                events.token(Token::Attribute(start..end, qname_len, eq_len, prefix, local, value))?;
            }
        }
    }

    if open {
        parse_content(s, events)?;
    }

    Ok(())
}

// Attribute ::= Name Eq AttValue
fn parse_attribute<'input>(
    s: &mut Stream<'input>,
) -> Result<(&'input str, &'input str, StrSpan<'input>)> {
    let (prefix, local) = s.consume_qname()?;
    s.consume_eq()?;
    let quote = s.consume_quote()?;
    let quote_c = quote as char;
    // The attribute value must not contain the < character.
    let value_start = s.pos();
    s.skip_chars(|_, c| c != quote_c && c != '<')?;
    let value = s.slice_back_span(value_start);
    s.consume_byte(quote)?;
    Ok((prefix, local, value))
}

// content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
pub fn parse_content<'input>(
    s: &mut Stream<'input>,
    events: &mut dyn XmlEvents<'input>,
) -> Result<()> {
    while !s.at_end() {
        match s.curr_byte() {
            Ok(b'<') => match s.next_byte() {
                Ok(b'!') => {
                    if s.starts_with(b"<!--") {
                        parse_comment(s, events)?;
                    } else if s.starts_with(b"<![CDATA[") {
                        parse_cdata(s, events)?;
                    } else {
                        return Err(Error::UnknownToken(s.gen_text_pos()));
                    }
                }
                Ok(b'?') => parse_pi(s, events)?,
                Ok(b'/') => {
                    parse_close_element(s, events)?;
                    break;
                }
                Ok(_) => parse_element(s, events)?,
                Err(_) => return Err(Error::UnknownToken(s.gen_text_pos())),
            },
            Ok(_) => parse_text(s, events)?,
            Err(_) => return Err(Error::UnknownToken(s.gen_text_pos())),
        }
    }

    Ok(())
}

// CDSect  ::= CDStart CData CDEnd
// CDStart ::= '<![CDATA['
// CData   ::= (Char* - (Char* ']]>' Char*))
// CDEnd   ::= ']]>'
fn parse_cdata<'input>(s: &mut Stream<'input>, events: &mut dyn XmlEvents<'input>) -> Result<()> {
    let start = s.pos();
    s.advance(9); // <![CDATA[
    let text = s.consume_chars(|s, c| !(c == ']' && s.starts_with(b"]]>")))?;
    s.skip_string(b"]]>")?;
    let range = s.range_from(start);
    events.token(Token::Cdata(text, range))?;
    Ok(())
}

// '</' Name S? '>'
fn parse_close_element<'input>(
    s: &mut Stream<'input>,
    events: &mut dyn XmlEvents<'input>,
) -> Result<()> {
    let start = s.pos();
    s.advance(2); // </

    let (prefix, tag_name) = s.consume_qname()?;
    s.skip_spaces();
    s.consume_byte(b'>')?;

    let range = s.range_from(start);
    events.token(Token::ElementEnd(
        ElementEnd::Close(prefix, tag_name),
        range,
    ))?;
    Ok(())
}

fn parse_text<'input>(s: &mut Stream<'input>, events: &mut dyn XmlEvents<'input>) -> Result<()> {
    let start = s.pos();
    let text = s.consume_chars(|_, c| c != '<')?;

    // According to the spec, `]]>` must not appear inside a Text node.
    // https://www.w3.org/TR/xml/#syntax
    //
    // Search for `>` first, since it's a bit faster than looking for `]]>`.
    if text.contains('>') && text.contains("]]>") {
        return Err(Error::InvalidCharacterData(s.gen_text_pos()));
    }

    let range = s.range_from(start);
    events.token(Token::Text(text, range))?;
    Ok(())
}

/// Representation of the [Reference](https://www.w3.org/TR/xml/#NT-Reference) value.
#[derive(Clone, Copy)]
pub enum Reference<'input> {
    /// An entity reference.
    ///
    /// <https://www.w3.org/TR/xml/#NT-EntityRef>
    Entity(&'input str),

    /// A character reference.
    ///
    /// <https://www.w3.org/TR/xml/#NT-CharRef>
    Char(char),
}

#[derive(Clone)]
pub struct Stream<'input> {
    pos: usize,
    end: usize,
    span: StrSpan<'input>,
}

impl<'input> Stream<'input> {
    #[inline]
    pub fn new(text: &'input str) -> Self {
        Stream {
            pos: 0,
            end: text.len(),
            span: text.into(),
        }
    }

    #[inline]
    pub fn from_substr(text: &'input str, fragment: Range<usize>) -> Self {
        Stream {
            pos: fragment.start,
            end: fragment.end,
            span: text.into(),
        }
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn at_end(&self) -> bool {
        self.pos >= self.end
    }

    #[inline]
    pub fn curr_byte(&self) -> Result<u8> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.curr_byte_unchecked())
    }

    #[inline]
    pub fn curr_byte_unchecked(&self) -> u8 {
        self.span.text.as_bytes()[self.pos]
    }

    #[inline]
    fn next_byte(&self) -> Result<u8> {
        if self.pos + 1 >= self.end {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.span.as_str().as_bytes()[self.pos + 1])
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.end);
        self.pos += n;
    }

    #[inline]
    fn starts_with(&self, text: &[u8]) -> bool {
        self.span.text.as_bytes()[self.pos..self.end].starts_with(text)
    }

    fn consume_byte(&mut self, c: u8) -> Result<()> {
        let curr = self.curr_byte()?;
        if curr != c {
            return Err(Error::InvalidChar(c, curr, self.gen_text_pos()));
        }

        self.advance(1);
        Ok(())
    }

    // Unlike `consume_byte()` will not return any errors.
    fn try_consume_byte(&mut self, c: u8) -> bool {
        match self.curr_byte() {
            Ok(b) if b == c => {
                self.advance(1);
                true
            }
            _ => false,
        }
    }

    fn skip_string(&mut self, text: &'static [u8]) -> Result<()> {
        if !self.starts_with(text) {
            let pos = self.gen_text_pos();

            // Assume that all input `text` are valid UTF-8 strings, so unwrap is safe.
            let expected = str::from_utf8(text).unwrap();

            return Err(Error::InvalidString(expected, pos));
        }

        self.advance(text.len());
        Ok(())
    }

    #[inline]
    fn consume_bytes<F: Fn(u8) -> bool>(&mut self, f: F) -> &'input str {
        let start = self.pos;
        self.skip_bytes(f);
        self.slice_back(start)
    }

    fn skip_bytes<F: Fn(u8) -> bool>(&mut self, f: F) {
        while !self.at_end() && f(self.curr_byte_unchecked()) {
            self.advance(1);
        }
    }

    #[inline]
    fn consume_chars<F>(&mut self, f: F) -> Result<&'input str>
    where
        F: Fn(&Stream, char) -> bool,
    {
        let start = self.pos;
        self.skip_chars(f)?;
        Ok(self.slice_back(start))
    }

    #[inline]
    fn skip_chars<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(&Stream, char) -> bool,
    {
        for c in self.chars() {
            if !c.is_xml_char() {
                return Err(Error::NonXmlChar(c, self.gen_text_pos()));
            } else if f(self, c) {
                self.advance(c.len_utf8());
            } else {
                break;
            }
        }

        Ok(())
    }

    #[inline]
    fn chars(&self) -> str::Chars<'input> {
        self.span.as_str()[self.pos..self.end].chars()
    }

    #[inline]
    fn slice_back(&self, pos: usize) -> &'input str {
        self.span.slice_region(pos, self.pos)
    }

    #[inline]
    fn slice_back_span(&self, pos: usize) -> StrSpan<'input> {
        StrSpan::from_substr(self.span.text, pos, self.pos)
    }

    #[inline]
    fn range_from(&self, start: usize) -> Range<usize> {
        start..self.pos
    }

    #[inline]
    fn skip_spaces(&mut self) {
        while self.starts_with_space() {
            self.advance(1);
        }
    }

    #[inline]
    fn starts_with_space(&self) -> bool {
        !self.at_end() && self.curr_byte_unchecked().is_xml_space()
    }

    // Like `skip_spaces()`, but checks that first char is actually a space.
    fn consume_spaces(&mut self) -> Result<()> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        if !self.starts_with_space() {
            return Err(Error::InvalidChar2(
                "a whitespace",
                self.curr_byte_unchecked(),
                self.gen_text_pos(),
            ));
        }

        self.skip_spaces();
        Ok(())
    }

    /// Consumes according to: <https://www.w3.org/TR/xml/#NT-Reference>
    pub fn try_consume_reference(&mut self) -> Option<Reference<'input>> {
        let start = self.pos();

        // Consume reference on a substream.
        let mut s = self.clone();
        let result = s.consume_reference()?;

        // If the current data is a reference than advance the current stream
        // by number of bytes read by substream.
        self.advance(s.pos() - start);
        Some(result)
    }

    #[inline(never)]
    fn consume_reference(&mut self) -> Option<Reference<'input>> {
        if !self.try_consume_byte(b'&') {
            return None;
        }

        let reference = if self.try_consume_byte(b'#') {
            let (value, radix) = if self.try_consume_byte(b'x') {
                let value =
                    self.consume_bytes(|c| matches!(c, b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f'));
                (value, 16)
            } else {
                let value = self.consume_bytes(|c| c.is_ascii_digit());
                (value, 10)
            };

            let n = u32::from_str_radix(value, radix).ok()?;

            let c = char::from_u32(n).unwrap_or('\u{FFFD}');
            if !c.is_xml_char() {
                return None;
            }

            Reference::Char(c)
        } else {
            let name = self.consume_name().ok()?;
            match name {
                "quot" => Reference::Char('"'),
                "amp" => Reference::Char('&'),
                "apos" => Reference::Char('\''),
                "lt" => Reference::Char('<'),
                "gt" => Reference::Char('>'),
                _ => Reference::Entity(name),
            }
        };

        self.consume_byte(b';').ok()?;

        Some(reference)
    }

    /// Consumes according to: <https://www.w3.org/TR/xml/#NT-Name>
    fn consume_name(&mut self) -> Result<&'input str> {
        let start = self.pos();
        self.skip_name()?;

        let name = self.slice_back(start);
        if name.is_empty() {
            return Err(Error::InvalidName(self.gen_text_pos_from(start)));
        }

        Ok(name)
    }

    /// The same as `consume_name()`, but does not return a consumed name.
    fn skip_name(&mut self) -> Result<()> {
        let start = self.pos();
        let mut iter = self.chars();
        if let Some(c) = iter.next() {
            if c.is_xml_name_start() {
                self.advance(c.len_utf8());
            } else {
                return Err(Error::InvalidName(self.gen_text_pos_from(start)));
            }
        }

        for c in iter {
            if c.is_xml_name() {
                self.advance(c.len_utf8());
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Consumes a qualified XML name and returns it.
    ///
    /// Consumes according to: <https://www.w3.org/TR/xml-names/#ns-qualnames>
    #[inline(never)]
    fn consume_qname(&mut self) -> Result<(&'input str, &'input str)> {
        let start = self.pos();

        let mut splitter = None;

        while !self.at_end() {
            // Check for ASCII first for performance reasons.
            let b = self.curr_byte_unchecked();
            if b < 128 {
                if b == b':' {
                    if splitter.is_none() {
                        splitter = Some(self.pos());
                        self.advance(1);
                    } else {
                        // Multiple `:` is an error.
                        return Err(Error::InvalidName(self.gen_text_pos_from(start)));
                    }
                } else if b.is_xml_name() {
                    self.advance(1);
                } else {
                    break;
                }
            } else {
                // Fallback to Unicode code point.
                match self.chars().nth(0) {
                    Some(c) if c.is_xml_name() => {
                        self.advance(c.len_utf8());
                    }
                    _ => break,
                }
            }
        }

        let (prefix, local) = if let Some(splitter) = splitter {
            let prefix = self.span.slice_region(start, splitter);
            let local = self.slice_back(splitter + 1);
            (prefix, local)
        } else {
            let local = self.slice_back(start);
            // Slice an empty prefix. This way we can preserve attribute start position.
            (self.span.slice_region(start, start), local)
        };

        // Prefix must start with a `NameStartChar`.
        if let Some(c) = prefix.chars().nth(0) {
            if !c.is_xml_name_start() {
                return Err(Error::InvalidName(self.gen_text_pos_from(start)));
            }
        }

        // Local name must start with a `NameStartChar`.
        if let Some(c) = local.chars().nth(0) {
            if !c.is_xml_name_start() {
                return Err(Error::InvalidName(self.gen_text_pos_from(start)));
            }
        } else {
            // If empty - error.
            return Err(Error::InvalidName(self.gen_text_pos_from(start)));
        }

        Ok((prefix, local))
    }

    fn consume_eq(&mut self) -> Result<()> {
        self.skip_spaces();
        self.consume_byte(b'=')?;
        self.skip_spaces();

        Ok(())
    }

    fn consume_quote(&mut self) -> Result<u8> {
        let c = self.curr_byte()?;
        if c == b'\'' || c == b'"' {
            self.advance(1);
            Ok(c)
        } else {
            Err(Error::InvalidChar2("a quote", c, self.gen_text_pos()))
        }
    }

    /// Calculates a current absolute position.
    ///
    /// This operation is very expensive. Use only for errors.
    #[inline(never)]
    pub fn gen_text_pos(&self) -> TextPos {
        let text = self.span.as_str();
        let end = self.pos;

        let row = Self::calc_curr_row(text, end);
        let col = Self::calc_curr_col(text, end);
        TextPos::new(row, col)
    }

    /// Calculates an absolute position at `pos`.
    ///
    /// This operation is very expensive. Use only for errors.
    #[inline(never)]
    pub fn gen_text_pos_from(&self, pos: usize) -> TextPos {
        let mut s = self.clone();
        s.pos = core::cmp::min(pos, s.span.as_str().len());
        s.gen_text_pos()
    }

    fn calc_curr_row(text: &str, end: usize) -> u32 {
        let mut row = 1;
        for c in &text.as_bytes()[..end] {
            if *c == b'\n' {
                row += 1;
            }
        }

        row
    }

    fn calc_curr_col(text: &str, end: usize) -> u32 {
        let mut col = 1;
        for c in text[..end].chars().rev() {
            if c == '\n' {
                break;
            } else {
                col += 1;
            }
        }

        col
    }
}
