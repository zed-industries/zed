/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::unescape::unescape;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};
use xmlparser::{ElementEnd, Token, Tokenizer};

pub type Depth = usize;

// in general, these errors are just for reporting what happened, there isn't
// much value in lots of different match variants

#[derive(Debug)]
enum XmlDecodeErrorKind {
    InvalidXml(xmlparser::Error),
    InvalidEscape { esc: String },
    Custom(Cow<'static, str>),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[derive(Debug)]
pub struct XmlDecodeError {
    kind: XmlDecodeErrorKind,
}

impl Display for XmlDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            XmlDecodeErrorKind::InvalidXml(_) => write!(f, "XML parse error"),
            XmlDecodeErrorKind::InvalidEscape { esc } => write!(f, "invalid XML escape: {esc}"),
            XmlDecodeErrorKind::Custom(msg) => write!(f, "error parsing XML: {msg}"),
            XmlDecodeErrorKind::Unhandled(_) => write!(f, "error parsing XML"),
        }
    }
}

impl Error for XmlDecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            XmlDecodeErrorKind::InvalidXml(source) => Some(source as _),
            XmlDecodeErrorKind::Unhandled(source) => Some(source.as_ref() as _),
            XmlDecodeErrorKind::InvalidEscape { .. } | XmlDecodeErrorKind::Custom(..) => None,
        }
    }
}

impl XmlDecodeError {
    pub(crate) fn invalid_xml(error: xmlparser::Error) -> Self {
        Self {
            kind: XmlDecodeErrorKind::InvalidXml(error),
        }
    }

    pub(crate) fn invalid_escape(esc: impl Into<String>) -> Self {
        Self {
            kind: XmlDecodeErrorKind::InvalidEscape { esc: esc.into() },
        }
    }

    pub fn custom(msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind: XmlDecodeErrorKind::Custom(msg.into()),
        }
    }

    pub fn unhandled(error: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: XmlDecodeErrorKind::Unhandled(error.into()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Name<'a> {
    pub prefix: &'a str,
    pub local: &'a str,
}

impl Name<'_> {
    /// Check if a given name matches a tag name composed of `prefix:local` or just `local`
    pub fn matches(&self, tag_name: &str) -> bool {
        let split = tag_name.find(':');
        match split {
            None => tag_name == self.local,
            Some(idx) => {
                let (prefix, local) = tag_name.split_at(idx);
                let local = &local[1..];
                self.local == local && self.prefix == prefix
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Attr<'a> {
    name: Name<'a>,
    // attribute values can be escaped (e.g. with double quotes, so we need a Cow)
    value: Cow<'a, str>,
}

#[derive(Debug, PartialEq)]
pub struct StartEl<'a> {
    name: Name<'a>,
    attributes: Vec<Attr<'a>>,
    closed: bool,
    depth: Depth,
}

/// Xml Start Element
///
/// ```xml
/// <a:b   c="d">
///  ^^^   ^^^^^
///  name  attributes
/// ```
impl<'a> StartEl<'a> {
    pub fn depth(&self) -> Depth {
        self.depth
    }

    fn new(local: &'a str, prefix: &'a str, depth: Depth) -> Self {
        Self {
            name: Name { prefix, local },
            attributes: vec![],
            closed: false,
            depth,
        }
    }

    /// Retrieve an attribute with a given key
    ///
    /// key `prefix:local` combined as a str, joined by a `:`
    pub fn attr<'b>(&'b self, key: &'b str) -> Option<&'b str> {
        self.attributes
            .iter()
            .find(|attr| attr.name.matches(key))
            .map(|attr| attr.value.as_ref())
    }

    /// Returns whether this `StartEl` matches a given name
    /// in `prefix:local` form.
    pub fn matches(&self, pat: &str) -> bool {
        self.name.matches(pat)
    }

    /// Local component of this element's name
    ///
    /// ```xml
    /// <foo:bar>
    ///      ^^^
    /// ```
    pub fn local(&self) -> &str {
        self.name.local
    }

    /// Prefix component of this elements name (or empty string)
    /// ```xml
    /// <foo:bar>
    ///  ^^^
    /// ```
    pub fn prefix(&self) -> &str {
        self.name.prefix
    }

    /// Returns true of `el` at `depth` is a match for this `start_el`
    fn end_el(&self, el: ElementEnd<'_>, depth: Depth) -> bool {
        if depth != self.depth {
            return false;
        }
        match el {
            ElementEnd::Open => false,
            ElementEnd::Close(prefix, local) => {
                prefix.as_str() == self.name.prefix && local.as_str() == self.name.local
            }
            ElementEnd::Empty => false,
        }
    }
}

/// Xml Document abstraction
///
/// This document wraps a lazy tokenizer with depth tracking.
/// Constructing a document is essentially free.
pub struct Document<'a> {
    tokenizer: Tokenizer<'a>,
    depth: Depth,
}

impl<'a> TryFrom<&'a [u8]> for Document<'a> {
    type Error = XmlDecodeError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Document::new(
            std::str::from_utf8(value).map_err(XmlDecodeError::unhandled)?,
        ))
    }
}

impl<'inp> Document<'inp> {
    pub fn new(doc: &'inp str) -> Self {
        Document {
            tokenizer: Tokenizer::from(doc),
            depth: 0,
        }
    }

    /// "Depth first" iterator
    ///
    /// Unlike [`next_tag()`](ScopedDecoder::next_tag), this method returns the next
    /// start element regardless of depth. This is useful to give a pointer into the middle
    /// of a document to start reading.
    ///
    /// ```xml
    /// <Response> <-- first call returns this:
    ///    <A> <-- next call
    ///      <Nested /> <-- next call returns this
    ///      <MoreNested>hello</MoreNested> <-- then this:
    ///    </A>
    ///    <B/> <-- second call to next_tag returns this
    /// </Response>
    /// ```
    pub fn next_start_element<'a>(&'a mut self) -> Option<StartEl<'inp>> {
        next_start_element(self)
    }

    /// A scoped reader for the entire document
    pub fn root_element<'a>(&'a mut self) -> Result<ScopedDecoder<'inp, 'a>, XmlDecodeError> {
        let start_el = self
            .next_start_element()
            .ok_or_else(|| XmlDecodeError::custom("no root element"))?;
        Ok(ScopedDecoder {
            doc: self,
            start_el,
            terminated: false,
        })
    }

    /// A scoped reader for a specific tag
    ///
    /// This method is necessary for when you need to return a ScopedDecoder from a function
    /// since normally the stacked-ownership that `next_tag()` uses would prevent returning a reference
    /// to a field owned by the current function
    pub fn scoped_to<'a>(&'a mut self, start_el: StartEl<'inp>) -> ScopedDecoder<'inp, 'a> {
        ScopedDecoder {
            doc: self,
            start_el,
            terminated: false,
        }
    }
}

/// A new-type wrapper around `Token` to prevent the wrapped third party type from showing up in
/// public API
#[derive(Debug)]
pub struct XmlToken<'inp>(Token<'inp>);

/// Depth tracking iterator
///
/// ```xml
/// <a> <- startel depth 0
///   <b> <- startel depth 1
///     <c> <- startel depth 2
///     </c> <- endel depth 2
///   </b> <- endel depth 1
/// </a> <- endel depth 0
/// ```
impl<'inp> Iterator for Document<'inp> {
    type Item = Result<(XmlToken<'inp>, Depth), XmlDecodeError>;
    fn next<'a>(&'a mut self) -> Option<Result<(XmlToken<'inp>, Depth), XmlDecodeError>> {
        let tok = self.tokenizer.next()?;
        let tok = match tok {
            Err(e) => return Some(Err(XmlDecodeError::invalid_xml(e))),
            Ok(tok) => tok,
        };
        // depth bookkeeping
        match tok {
            Token::ElementEnd {
                end: ElementEnd::Close(_, _),
                ..
            } => {
                self.depth -= 1;
            }
            Token::ElementEnd {
                end: ElementEnd::Empty,
                ..
            } => self.depth -= 1,
            t @ Token::ElementStart { .. } => {
                self.depth += 1;
                // We want the startel and endel to have the same depth, but after the opener,
                // the parser will be at depth 1. Return the previous depth:
                return Some(Ok((XmlToken(t), self.depth - 1)));
            }
            _ => {}
        }
        Some(Ok((XmlToken(tok), self.depth)))
    }
}

/// XmlTag Abstraction
///
/// ScopedDecoder represents a tag-scoped view into an XML document. Methods
/// on `ScopedDecoder` return `None` when the current tag has been exhausted.
pub struct ScopedDecoder<'inp, 'a> {
    doc: &'a mut Document<'inp>,
    start_el: StartEl<'inp>,
    terminated: bool,
}

/// When a scoped decoder is dropped, its entire scope is consumed so that the
/// next read begins at the next tag at the same depth.
impl Drop for ScopedDecoder<'_, '_> {
    fn drop(&mut self) {
        for _ in self {}
    }
}

impl<'inp> ScopedDecoder<'inp, '_> {
    /// The start element for this scope
    pub fn start_el<'a>(&'a self) -> &'a StartEl<'inp> {
        &self.start_el
    }

    /// Returns the next top-level tag in this scope
    /// The returned reader will fully read the tag during its lifetime. If it is dropped without
    /// the data being read, the reader will be advanced until the matching close tag. If you read
    /// an element with `next_tag()` and you want to ignore it, simply drop the resulting `ScopeDecoder`.
    ///
    /// ```xml
    /// <Response> <-- scoped reader on this tag
    ///    <A> <-- first call to next_tag returns this
    ///      <Nested /> <-- to get inner data, call `next_tag` on the returned decoder for `A`
    ///      <MoreNested>hello</MoreNested>
    ///    </A>
    ///    <B/> <-- second call to next_tag returns this
    /// </Response>
    /// ```
    pub fn next_tag<'a>(&'a mut self) -> Option<ScopedDecoder<'inp, 'a>> {
        let next_tag = next_start_element(self)?;
        Some(self.nested_decoder(next_tag))
    }

    fn nested_decoder<'a>(&'a mut self, start_el: StartEl<'inp>) -> ScopedDecoder<'inp, 'a> {
        ScopedDecoder {
            doc: self.doc,
            start_el,
            terminated: false,
        }
    }
}

impl<'inp> Iterator for ScopedDecoder<'inp, '_> {
    type Item = Result<(XmlToken<'inp>, Depth), XmlDecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_el.closed {
            self.terminated = true;
        }
        if self.terminated {
            return None;
        }
        let (tok, depth) = match self.doc.next() {
            Some(Ok((tok, depth))) => (tok, depth),
            other => return other,
        };

        match tok.0 {
            Token::ElementEnd { end, .. } if self.start_el.end_el(end, depth) => {
                self.terminated = true;
                return None;
            }
            _ => {}
        }
        Some(Ok((tok, depth)))
    }
}

/// Load the next start element out of a depth-tagged token iterator
fn next_start_element<'a, 'inp>(
    tokens: &'a mut impl Iterator<Item = Result<(XmlToken<'inp>, Depth), XmlDecodeError>>,
) -> Option<StartEl<'inp>> {
    let mut out = StartEl::new("", "", 0);
    loop {
        match tokens.next()? {
            Ok((XmlToken(Token::ElementStart { local, prefix, .. }), depth)) => {
                out.name.local = local.as_str();
                out.name.prefix = prefix.as_str();
                out.depth = depth;
            }
            Ok((
                XmlToken(Token::Attribute {
                    prefix,
                    local,
                    value,
                    ..
                }),
                _,
            )) => out.attributes.push(Attr {
                name: Name {
                    local: local.as_str(),
                    prefix: prefix.as_str(),
                },
                value: unescape(value.as_str()).ok()?,
            }),
            Ok((
                XmlToken(Token::ElementEnd {
                    end: ElementEnd::Open,
                    ..
                }),
                _,
            )) => break,
            Ok((
                XmlToken(Token::ElementEnd {
                    end: ElementEnd::Empty,
                    ..
                }),
                _,
            )) => {
                out.closed = true;
                break;
            }
            _ => {}
        }
    }
    Some(out)
}

/// Returns the data element at the current position
///
/// If the current position is not a data element (and is instead a `<start-element>`) an error
/// will be returned
pub fn try_data<'a, 'inp>(
    tokens: &'a mut impl Iterator<Item = Result<(XmlToken<'inp>, Depth), XmlDecodeError>>,
) -> Result<Cow<'inp, str>, XmlDecodeError> {
    loop {
        match tokens.next().map(|opt| opt.map(|opt| opt.0)) {
            None => return Ok(Cow::Borrowed("")),
            Some(Ok(XmlToken(Token::Text { text }))) => return unescape(text.as_str()),
            Some(Ok(e @ XmlToken(Token::ElementStart { .. }))) => {
                return Err(XmlDecodeError::custom(format!(
                    "looking for a data element, found: {e:?}"
                )))
            }
            Some(Err(e)) => return Err(e),
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decode::{try_data, Attr, Depth, Document, Name, StartEl};

    // test helper to create a closed startel
    fn closed<'a>(local: &'a str, prefix: &'a str, depth: Depth) -> StartEl<'a> {
        let mut s = StartEl::new(local, prefix, depth);
        s.closed = true;
        s
    }

    #[test]
    fn scoped_tokens() {
        let xml = r#"<Response><A></A></Response>"#;
        let mut doc = Document::new(xml);
        let mut root = doc.root_element().expect("valid document");
        assert_eq!(root.start_el().local(), "Response");
        assert_eq!(root.next_tag().expect("tag exists").start_el().local(), "A");
        assert!(root.next_tag().is_none());
    }

    #[test]
    fn handle_depth_properly() {
        let xml = r#"<Response><Response></Response><A/></Response>"#;
        let mut doc = Document::new(xml);
        let mut scoped = doc.root_element().expect("valid document");
        assert_eq!(
            scoped.next_tag().unwrap().start_el(),
            &StartEl::new("Response", "", 1)
        );
        let closed_a = closed("A", "", 1);
        assert_eq!(scoped.next_tag().unwrap().start_el(), &closed_a);
        assert!(scoped.next_tag().is_none())
    }

    #[test]
    fn self_closing() {
        let xml = r#"<Response/>"#;
        let mut doc = Document::new(xml);
        let mut scoped = doc.root_element().expect("valid doc");
        assert!(scoped.start_el.closed);
        assert!(scoped.next_tag().is_none())
    }

    #[test]
    fn terminate_scope() {
        let xml = r#"<Response><Struct><A></A><Also/></Struct><More/></Response>"#;
        let mut doc = Document::new(xml);
        let mut response_iter = doc.root_element().expect("valid doc");
        let mut struct_iter = response_iter.next_tag().unwrap();
        assert_eq!(
            struct_iter.next_tag().as_ref().map(|t| t.start_el()),
            Some(&StartEl::new("A", "", 2))
        );
        // When the inner iter is dropped, it will read to the end of its scope
        // prevent accidental behavior where we didn't read a full node
        drop(struct_iter);
        assert_eq!(
            response_iter.next_tag().unwrap().start_el(),
            &closed("More", "", 1)
        );
    }

    #[test]
    fn read_data_invalid() {
        let xml = r#"<Response><A></A></Response>"#;
        let mut doc = Document::new(xml);
        let mut resp = doc.root_element().unwrap();
        try_data(&mut resp).expect_err("no data");
    }

    #[test]
    fn read_data() {
        let xml = r#"<Response>hello</Response>"#;
        let mut doc = Document::new(xml);
        let mut scoped = doc.root_element().unwrap();
        assert_eq!(try_data(&mut scoped).unwrap(), "hello");
    }

    /// Whitespace within an element is preserved
    #[test]
    fn read_data_whitespace() {
        let xml = r#"<Response> hello </Response>"#;
        let mut doc = Document::new(xml);
        let mut scoped = doc.root_element().unwrap();
        assert_eq!(try_data(&mut scoped).unwrap(), " hello ");
    }

    #[test]
    fn ignore_insignificant_whitespace() {
        let xml = r#"<Response>   <A>  </A>    </Response>"#;
        let mut doc = Document::new(xml);
        let mut resp = doc.root_element().unwrap();
        let mut a = resp.next_tag().expect("should be a");
        let data = try_data(&mut a).expect("valid");
        assert_eq!(data, "  ");
    }

    #[test]
    fn read_attributes() {
        let xml = r#"<Response xsi:type="CanonicalUser">hello</Response>"#;
        let mut tokenizer = Document::new(xml);
        let root = tokenizer.root_element().unwrap();

        assert_eq!(
            root.start_el().attributes,
            vec![Attr {
                name: Name {
                    prefix: "xsi",
                    local: "type"
                },
                value: "CanonicalUser".into()
            }]
        )
    }

    #[test]
    fn unescape_data() {
        let xml = r#"<Response key="&quot;hey&quot;>">&gt;</Response>"#;
        let mut doc = Document::new(xml);
        let mut root = doc.root_element().unwrap();
        assert_eq!(try_data(&mut root).unwrap(), ">");
        assert_eq!(root.start_el().attr("key"), Some("\"hey\">"));
    }

    #[test]
    fn nested_self_closer() {
        let xml = r#"<XmlListsInputOutput>
                <stringList/>
                <stringSet></stringSet>
        </XmlListsInputOutput>"#;
        let mut doc = Document::new(xml);
        let mut root = doc.root_element().unwrap();
        let mut string_list = root.next_tag().unwrap();
        assert_eq!(string_list.start_el(), &closed("stringList", "", 1));
        assert!(string_list.next_tag().is_none());
        drop(string_list);
        assert_eq!(
            root.next_tag().unwrap().start_el(),
            &StartEl::new("stringSet", "", 1)
        );
    }

    #[test]
    fn confusing_nested_same_name_tag() {
        // an inner b which could be confused as closing the outer b if depth
        // is not properly tracked:
        let root_tags = &["a", "b", "c", "d"];
        let xml = r#"<XmlListsInputOutput>
                <a/>
                <b>
                  <c/>
                  <b></b>
                  <here/>
                </b>
                <c></c>
                <d>more</d>
        </XmlListsInputOutput>"#;
        let mut doc = Document::new(xml);
        let mut root = doc.root_element().unwrap();
        let mut cmp = vec![];
        while let Some(tag) = root.next_tag() {
            cmp.push(tag.start_el().local().to_owned());
        }
        assert_eq!(root_tags, cmp.as_slice());
    }
}
