//! Contains an implementation of pull-based XML parser.

use crate::common::{is_xml10_char, is_xml11_char, is_xml11_char_not_restricted, is_name_char, is_name_start_char, is_whitespace_char};
use crate::common::{Position, TextPosition, XmlVersion};
use crate::name::OwnedName;
use crate::namespace::NamespaceStack;
use crate::reader::config::ParserConfig2;
use crate::reader::error::SyntaxError;
use crate::reader::events::XmlEvent;
use crate::reader::indexset::AttributesSet;
use crate::reader::lexer::{Lexer, Token};
use super::{Error, ErrorKind};

use std::collections::HashMap;
use std::io::Read;

macro_rules! gen_takes(
    ($($field:ident -> $method:ident, $t:ty, $def:expr);+) => (
        $(
        impl MarkupData {
            #[inline]
            #[allow(clippy::mem_replace_option_with_none)]
            #[allow(clippy::mem_replace_with_default)]
            fn $method(&mut self) -> $t {
                std::mem::replace(&mut self.$field, $def)
            }
        }
        )+
    )
);

gen_takes!(
    name         -> take_name, String, String::new();
    ref_data     -> take_ref_data, String, String::new();

    encoding     -> take_encoding, Option<String>, None;

    element_name -> take_element_name, Option<OwnedName>, None;

    attr_name    -> take_attr_name, Option<OwnedName>, None;
    attributes   -> take_attributes, AttributesSet, AttributesSet::new()
);

mod inside_cdata;
mod inside_closing_tag_name;
mod inside_comment;
mod inside_declaration;
mod inside_doctype;
mod inside_opening_tag;
mod inside_processing_instruction;
mod inside_reference;
mod outside_tag;

static DEFAULT_VERSION: XmlVersion = XmlVersion::Version10;
static DEFAULT_STANDALONE: Option<bool> = None;

type ElementStack = Vec<OwnedName>;
pub type Result = super::Result<XmlEvent>;

/// Pull-based XML parser.
pub(crate) struct PullParser {
    config: ParserConfig2,
    lexer: Lexer,
    st: State,
    state_after_reference: State,
    buf: String,

    /// From DTD internal subset
    entities: HashMap<String, String>,

    nst: NamespaceStack,

    data: MarkupData,
    final_result: Option<Result>,
    next_event: Option<Result>,
    est: ElementStack,
    pos: Vec<TextPosition>,

    encountered: Encountered,
    inside_whitespace: bool,
    seen_prefix_separator: bool,
    pop_namespace: bool,
}

// Keeps track when XML declaration can happen
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Encountered {
    None = 0,
    AnyChars, // whitespace before <?xml is not allowed
    Declaration,
    Comment,
    Doctype,
    Element,
}

impl PullParser {
    /// Returns a new parser using the given config.
    #[inline]
    pub fn new(config: impl Into<ParserConfig2>) -> Self {
        let config = config.into();
        Self::new_with_config2(config)
    }

    #[inline]
    fn new_with_config2(config: ParserConfig2) -> Self {
        let mut lexer = Lexer::new(&config);
        if let Some(enc) = config.override_encoding {
            lexer.set_encoding(enc);
        }

        let mut pos = Vec::with_capacity(16);
        pos.push(TextPosition::new());

        Self {
            config,
            lexer,
            st: State::DocumentStart,
            state_after_reference: State::OutsideTag,
            buf: String::new(),
            entities: HashMap::new(),
            nst: NamespaceStack::default(),

            data: MarkupData {
                name: String::new(),
                doctype: None,
                version: None,
                encoding: None,
                standalone: None,
                ref_data: String::new(),
                element_name: None,
                quote: None,
                attr_name: None,
                attributes: AttributesSet::new(),
            },
            final_result: None,
            next_event: None,
            est: Vec::new(),
            pos,

            encountered: Encountered::None,
            inside_whitespace: true,
            seen_prefix_separator: false,
            pop_namespace: false,
        }
    }

    /// Checks if this parser ignores the end of stream errors.
    pub fn is_ignoring_end_of_stream(&self) -> bool { self.config.c.ignore_end_of_stream }

    /// Retrieves the Doctype from the document if any
    #[inline]
    pub fn doctype(&self) -> Option<&str> {
        self.data.doctype.as_deref()
    }

    #[inline(never)]
    fn set_encountered(&mut self, new_encounter: Encountered) -> Option<Result> {
        if new_encounter <= self.encountered {
            return None;
        }
        let prev_enc = self.encountered;
        self.encountered = new_encounter;

        // If declaration was not parsed and we have encountered an element,
        // emit this declaration as the next event.
        if prev_enc == Encountered::None {
            self.push_pos();
            Some(Ok(XmlEvent::StartDocument {
                version: DEFAULT_VERSION,
                encoding: self.lexer.encoding().to_string(),
                standalone: DEFAULT_STANDALONE,
            }))
        } else {
            None
        }
    }
}

impl Position for PullParser {
    /// Returns the position of the last event produced by the parser
    #[inline]
    fn position(&self) -> TextPosition {
        self.pos.first().copied().unwrap_or_else(TextPosition::new)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    OutsideTag,
    InsideOpeningTag(OpeningTagSubstate),
    InsideClosingTag(ClosingTagSubstate),
    InsideProcessingInstruction(ProcessingInstructionSubstate),
    InsideComment,
    InsideCData,
    InsideDeclaration(DeclarationSubstate),
    InsideDoctype(DoctypeSubstate),
    InsideReference,
    DocumentStart,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DoctypeSubstate {
    Outside,
    String,
    InsideName,
    BeforeEntityName,
    EntityName,
    BeforeEntityValue,
    EntityValue,
    NumericReferenceStart,
    NumericReference,
    /// expansion
    PEReferenceInValue,
    PEReferenceInDtd,
    /// name definition
    PEReferenceDefinitionStart,
    PEReferenceDefinition,
    SkipDeclaration,
    Comment,
}

#[derive(Copy, Clone, PartialEq)]
pub enum OpeningTagSubstate {
    InsideName,

    InsideTag,

    InsideAttributeName,
    AfterAttributeName,

    InsideAttributeValue,
    AfterAttributeValue,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ClosingTagSubstate {
    CTInsideName,
    CTAfterName,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ProcessingInstructionSubstate {
    PIInsideName,
    PIInsideData,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DeclarationSubstate {
    BeforeVersion,
    InsideVersion,
    AfterVersion,

    InsideVersionValue,
    AfterVersionValue,

    BeforeEncoding,
    InsideEncoding,
    AfterEncoding,

    InsideEncodingValue,
    AfterEncodingValue,

    BeforeStandaloneDecl,
    InsideStandaloneDecl,
    AfterStandaloneDecl,

    InsideStandaloneDeclValue,
    AfterStandaloneDeclValue,
}

#[derive(Copy, Clone, PartialEq)]
enum QualifiedNameTarget {
    AttributeNameTarget,
    OpeningTagNameTarget,
    ClosingTagNameTarget,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum QuoteToken {
    SingleQuoteToken,
    DoubleQuoteToken,
}

impl QuoteToken {
    #[inline]
    fn from_token(t: Token) -> Option<Self> {
        match t {
            Token::SingleQuote => Some(Self::SingleQuoteToken),
            Token::DoubleQuote => Some(Self::DoubleQuoteToken),
            _ => {
                debug_assert!(false);
                None
            },
        }
    }

    const fn as_token(self) -> Token {
        match self {
            Self::SingleQuoteToken => Token::SingleQuote,
            Self::DoubleQuoteToken => Token::DoubleQuote,
        }
    }
}

struct MarkupData {
    name: String,     // used for processing instruction name
    ref_data: String,  // used for reference content

    doctype: Option<String>, // keeps a copy of the original doctype
    version: Option<XmlVersion>,  // used for XML declaration version
    encoding: Option<String>,  // used for XML declaration encoding
    standalone: Option<bool>,  // used for XML declaration standalone parameter

    element_name: Option<OwnedName>,  // used for element name

    quote: Option<QuoteToken>,  // used to hold opening quote for attribute value
    attr_name: Option<OwnedName>,  // used to hold attribute name
    attributes: AttributesSet,   // used to hold all accumulated attributes
}

impl PullParser {
    /// Returns next event read from the given buffer.
    ///
    /// This method should be always called with the same buffer. If you call it
    /// providing different buffers each time, the result will be undefined.
    pub fn next<R: Read>(&mut self, r: &mut R) -> Result {
        if let Some(ref ev) = self.final_result {
            return ev.clone();
        }

        if let Some(ev) = self.next_event.take() {
            return ev;
        }

        if self.pop_namespace {
            self.pop_namespace = false;
            self.nst.pop();
        }

        loop {
            debug_assert!(self.next_event.is_none());
            debug_assert!(!self.pop_namespace);

            // While lexer gives us Ok(maybe_token) -- we loop.
            // Upon having a complete XML-event -- we return from the whole function.
            match self.lexer.next_token(r) {
                Ok(Token::Eof) => {
                    // Forward pos to the lexer head
                    self.next_pos();
                    return self.handle_eof();
                },
                Ok(token) => match self.dispatch_token(token) {
                    None => continue,
                    Some(Ok(xml_event)) => {
                        self.next_pos();
                        return Ok(xml_event);
                    },
                    Some(Err(xml_error)) => {
                        self.next_pos();
                        return self.set_final_result(Err(xml_error));
                    },
                },
                Err(lexer_error) => {
                    self.next_pos();
                    return self.set_final_result(Err(lexer_error));
                },
            }
        }
    }

    /// Handle end of stream
    #[cold]
    fn handle_eof(&mut self) -> std::result::Result<XmlEvent, super::Error> {
        let ev = if self.depth() == 0 {
            if self.encountered == Encountered::Element && self.st == State::OutsideTag {  // all is ok
                Ok(XmlEvent::EndDocument)
            } else if self.encountered < Encountered::Element {
                self.error(SyntaxError::NoRootElement)
            } else {  // self.st != State::OutsideTag
                self.error(SyntaxError::UnexpectedEof)  // TODO: add expected hint?
            }
        } else if self.config.c.ignore_end_of_stream {
            self.final_result = None;
            self.lexer.reset_eof_handled();
            return self.error(SyntaxError::UnbalancedRootElement);
        } else {
            self.error(SyntaxError::UnbalancedRootElement)
        };
        self.set_final_result(ev)
    }

    // This function is to be called when a terminal event is reached.
    // The function sets up the `self.final_result` into `Some(result)` and return `result`.
    #[inline]
    fn set_final_result(&mut self, result: Result) -> Result {
        self.final_result = Some(result.clone());
        result
    }

    #[cold]
    fn error(&self, e: SyntaxError) -> Result {
        Err(Error {
            pos: self.lexer.position(),
            kind: ErrorKind::Syntax(e.to_cow()),
        })
    }

    #[inline]
    fn next_pos(&mut self) {
        // unfortunately calls to next_pos will never be perfectly balanced with push_pos,
        // at very least because parse errors and EOF can happen unexpectedly without a prior push.
        if !self.pos.is_empty() {
            if self.pos.len() > 1 {
                self.pos.remove(0);
            } else {
                self.pos[0] = self.lexer.position();
            }
        }
    }

    #[inline]
    #[track_caller]
    fn push_pos(&mut self) {
        debug_assert!(self.pos.len() != self.pos.capacity(), "You've found a bug in xml-rs, caused by calls to push_pos() in states that don't end up emitting events.
            This case is ignored in release mode, and merely causes document positions to be out of sync.
            Please file a bug and include the XML document that triggers this assert.");

        // it has capacity preallocated for more than it ever needs, so this reduces code size
        if self.pos.len() != self.pos.capacity() {
            self.pos.push(self.lexer.position());
        } else if self.pos.len() > 1 {
            self.pos.remove(0); // this mitigates the excessive push_pos() call
        }
    }

    #[inline(never)]
    fn dispatch_token(&mut self, t: Token) -> Option<Result> {
        match self.st {
            State::OutsideTag                     => self.outside_tag(t),
            State::InsideOpeningTag(s)            => self.inside_opening_tag(t, s),
            State::InsideClosingTag(s)            => self.inside_closing_tag_name(t, s),
            State::InsideReference                => self.inside_reference(t),
            State::InsideComment                  => self.inside_comment(t),
            State::InsideCData                    => self.inside_cdata(t),
            State::InsideProcessingInstruction(s) => self.inside_processing_instruction(t, s),
            State::InsideDoctype(s)               => self.inside_doctype(t, s),
            State::InsideDeclaration(s)           => self.inside_declaration(t, s),
            State::DocumentStart                  => self.document_start(t),
        }
    }

    #[inline]
    fn depth(&self) -> usize {
        self.est.len()
    }

    #[inline]
    fn buf_has_data(&self) -> bool {
        !self.buf.is_empty()
    }

    #[inline]
    fn take_buf(&mut self) -> String {
        std::mem::take(&mut self.buf)
    }

    #[inline]
    fn into_state(&mut self, st: State, ev: Option<Result>) -> Option<Result> {
        self.st = st;
        ev
    }

    #[inline]
    fn into_state_continue(&mut self, st: State) -> Option<Result> {
        self.into_state(st, None)
    }

    #[inline]
    fn into_state_emit(&mut self, st: State, ev: Result) -> Option<Result> {
        self.into_state(st, Some(ev))
    }

    /// Dispatches tokens in order to process qualified name. If qualified name cannot be parsed,
    /// an error is returned.
    ///
    /// # Parameters
    /// * `t`       --- next token;
    /// * `on_name` --- a callback which is executed when whitespace is encountered.
    fn read_qualified_name<F>(&mut self, t: Token, target: QualifiedNameTarget, on_name: F) -> Option<Result>
      where F: Fn(&mut Self, Token, OwnedName) -> Option<Result> {

        let invoke_callback = move |this: &mut Self, t| {
            let name = this.take_buf();
            this.seen_prefix_separator = false;
            match name.parse() {
                Ok(name) => on_name(this, t, name),
                Err(()) => Some(this.error(SyntaxError::InvalidQualifiedName(name.into()))),
            }
        };

        match t {
            // There can be only one colon, and not as the first character
            Token::Character(':') if self.buf_has_data() && !self.seen_prefix_separator => {
                self.buf.push(':');
                self.seen_prefix_separator = true;
                None
            },

            Token::Character(c) if c != ':' && (self.buf.is_empty() && is_name_start_char(c) ||
                                          self.buf_has_data() && is_name_char(c)) => {
                if self.buf.len() > self.config.max_name_length {
                    return Some(self.error(SyntaxError::ExceededConfiguredLimit));
                }
                self.buf.push(c);
                None
            },

            Token::EqualsSign if target == QualifiedNameTarget::AttributeNameTarget => invoke_callback(self, t),

            Token::EmptyTagEnd if target == QualifiedNameTarget::OpeningTagNameTarget => invoke_callback(self, t),

            Token::TagEnd if target == QualifiedNameTarget::OpeningTagNameTarget ||
                      target == QualifiedNameTarget::ClosingTagNameTarget => invoke_callback(self, t),

            Token::Character(c) if is_whitespace_char(c) => invoke_callback(self, t),

            _ => Some(self.error(SyntaxError::UnexpectedQualifiedName(t))),
        }
    }

    /// Dispatches tokens in order to process attribute value.
    ///
    /// # Parameters
    /// * `t`        --- next token;
    /// * `on_value` --- a callback which is called when terminating quote is encountered.
    fn read_attribute_value<F>(&mut self, t: Token, on_value: F) -> Option<Result>
      where F: Fn(&mut Self, String) -> Option<Result> {
        match t {
            Token::Character(c) if self.data.quote.is_none() && is_whitespace_char(c) => None, // skip leading whitespace

            Token::DoubleQuote | Token::SingleQuote => match self.data.quote {
                None => {  // Entered attribute value
                    self.data.quote = QuoteToken::from_token(t);
                    None
                },
                Some(q) if q.as_token() == t => {
                    self.data.quote = None;
                    let value = self.take_buf();
                    on_value(self, value)
                },
                _ => {
                    if let Token::Character(c) = t {
                        if !self.is_valid_xml_char_not_restricted(c) {
                            return Some(self.error(SyntaxError::InvalidCharacterEntity(c as u32)));
                        }
                    }
                    if self.buf.len() > self.config.max_attribute_length {
                        return Some(self.error(SyntaxError::ExceededConfiguredLimit));
                    }
                    t.push_to_string(&mut self.buf);
                    None
                },
            },

            Token::ReferenceStart if self.data.quote.is_some() => {
                self.state_after_reference = self.st;
                self.into_state_continue(State::InsideReference)
            },

            Token::OpeningTagStart | Token::ProcessingInstructionStart => Some(self.error(SyntaxError::UnexpectedOpeningTag)),

            Token::Character(c) if !self.is_valid_xml_char_not_restricted(c) => {
                Some(self.error(SyntaxError::InvalidCharacterEntity(c as u32)))
            },

            // Every character except " and ' and < is okay
            _ if self.data.quote.is_some() => {
                if self.buf.len() > self.config.max_attribute_length {
                    return Some(self.error(SyntaxError::ExceededConfiguredLimit));
                }
                t.push_to_string(&mut self.buf);
                None
            },

            _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
        }
    }

    fn emit_start_element(&mut self, emit_end_element: bool) -> Option<Result> {
        let mut name = self.data.take_element_name()?;
        let mut attributes = self.data.take_attributes().into_vec();

        // check whether the name prefix is bound and fix its namespace
        match self.nst.get(name.borrow().prefix_repr()) {
            Some("") => name.namespace = None, // default namespace
            Some(ns) => name.namespace = Some(ns.into()),
            None => return Some(self.error(SyntaxError::UnboundElementPrefix(name.to_string().into()))),
        }

        // check and fix accumulated attributes prefixes
        for attr in &mut attributes {
            if let Some(ref pfx) = attr.name.prefix {
                let new_ns = match self.nst.get(pfx) {
                    Some("") => None, // default namespace
                    Some(ns) => Some(ns.into()),
                    None => return Some(self.error(SyntaxError::UnboundAttribute(attr.name.to_string().into()))),
                };
                attr.name.namespace = new_ns;
            }
        }

        if emit_end_element {
            self.pop_namespace = true;
            self.next_event = Some(Ok(XmlEvent::EndElement {
                name: name.clone()
            }));
        } else {
            self.est.push(name.clone());
        }
        let namespace = self.nst.squash();
        self.into_state_emit(State::OutsideTag, Ok(XmlEvent::StartElement {
            name,
            attributes,
            namespace
        }))
    }

    fn emit_end_element(&mut self) -> Option<Result> {
        let mut name = self.data.take_element_name()?;

        // check whether the name prefix is bound and fix its namespace
        match self.nst.get(name.borrow().prefix_repr()) {
            Some("") => name.namespace = None, // default namespace
            Some(ns) => name.namespace = Some(ns.into()),
            None => return Some(self.error(SyntaxError::UnboundElementPrefix(name.to_string().into()))),
        }

        let op_name = self.est.pop()?;

        if name == op_name {
            self.pop_namespace = true;
            self.into_state_emit(State::OutsideTag, Ok(XmlEvent::EndElement { name }))
        } else {
            Some(self.error(SyntaxError::UnexpectedClosingTag(format!("{name} != {op_name}").into())))
        }
    }

    #[inline]
    fn is_valid_xml_char(&self, c: char) -> bool {
        if Some(XmlVersion::Version11) == self.data.version {
            is_xml11_char(c)
        } else {
            is_xml10_char(c)
        }
    }

    #[inline]
    fn is_valid_xml_char_not_restricted(&self, c: char) -> bool {
        if Some(XmlVersion::Version11) == self.data.version {
            is_xml11_char_not_restricted(c)
        } else {
            is_xml10_char(c)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::attribute::OwnedAttribute;
    use crate::common::TextPosition;
    use crate::name::OwnedName;
    use crate::reader::events::XmlEvent;
    use crate::reader::parser::PullParser;
    use crate::reader::ParserConfig;
    use std::io::BufReader;

    fn new_parser() -> PullParser {
        PullParser::new(ParserConfig::new())
    }

    macro_rules! expect_event(
        ($r:expr, $p:expr, $t:pat) => (
            match $p.next(&mut $r) {
                $t => {}
                e => panic!("Unexpected event: {e:?}\nExpected: {}", stringify!($t))
            }
        );
        ($r:expr, $p:expr, $t:pat => $c:expr ) => (
            match $p.next(&mut $r) {
                $t if $c => {}
                e => panic!("Unexpected event: {e:?}\nExpected: {} if {}", stringify!($t), stringify!($c))
            }
        )
    );

    macro_rules! test_data(
        ($d:expr) => ({
            static DATA: &'static str = $d;
            let r = BufReader::new(DATA.as_bytes());
            let p = new_parser();
            (r, p)
        })
    );

    #[test]
    fn issue_3_semicolon_in_attribute_value() {
        let (mut r, mut p) = test_data!(r#"
            <a attr="zzz;zzz" />
        "#);

        expect_event!(r, p, Ok(XmlEvent::StartDocument { .. }));
        expect_event!(r, p, Ok(XmlEvent::StartElement { ref name, ref attributes, ref namespace }) =>
            *name == OwnedName::local("a") &&
             attributes.len() == 1 &&
             attributes[0] == OwnedAttribute::new(OwnedName::local("attr"), "zzz;zzz") &&
             namespace.is_essentially_empty()
        );
        expect_event!(r, p, Ok(XmlEvent::EndElement { ref name }) => *name == OwnedName::local("a"));
        expect_event!(r, p, Ok(XmlEvent::EndDocument));
    }

    #[test]
    fn issue_140_entity_reference_inside_tag() {
        let (mut r, mut p) = test_data!(r"
            <bla>&#9835;</bla>
        ");

        expect_event!(r, p, Ok(XmlEvent::StartDocument { .. }));
        expect_event!(r, p, Ok(XmlEvent::StartElement { ref name, .. }) => *name == OwnedName::local("bla"));
        expect_event!(r, p, Ok(XmlEvent::Characters(ref s)) => s == "\u{266b}");
        expect_event!(r, p, Ok(XmlEvent::EndElement { ref name, .. }) => *name == OwnedName::local("bla"));
        expect_event!(r, p, Ok(XmlEvent::EndDocument));
    }

    #[test]
    fn issue_220_comment() {
        let (mut r, mut p) = test_data!(r"<x><!-- <!--></x>");
        expect_event!(r, p, Ok(XmlEvent::StartDocument { .. }));
        expect_event!(r, p, Ok(XmlEvent::StartElement { .. }));
        expect_event!(r, p, Ok(XmlEvent::EndElement { .. }));
        expect_event!(r, p, Ok(XmlEvent::EndDocument));

        let (mut r, mut p) = test_data!(r"<x><!-- <!---></x>");
        expect_event!(r, p, Ok(XmlEvent::StartDocument { .. }));
        expect_event!(r, p, Ok(XmlEvent::StartElement { .. }));
        expect_event!(r, p, Err(_)); // ---> is forbidden in comments

        let (mut r, mut p) = test_data!(r"<x><!--<text&x;> <!--></x>");
        p.config.c.ignore_comments = false;
        expect_event!(r, p, Ok(XmlEvent::StartDocument { .. }));
        expect_event!(r, p, Ok(XmlEvent::StartElement { .. }));
        expect_event!(r, p, Ok(XmlEvent::Comment(s)) => s == "<text&x;> <!");
        expect_event!(r, p, Ok(XmlEvent::EndElement { .. }));
        expect_event!(r, p, Ok(XmlEvent::EndDocument));
    }

    #[test]
    fn malformed_declaration_attrs() {
        let (mut r, mut p) = test_data!(r#"<?xml version x="1.0"?>"#);
        expect_event!(r, p, Err(_));

        let (mut r, mut p) = test_data!(r#"<?xml version="1.0" version="1.0"?>"#);
        expect_event!(r, p, Err(_));

        let (mut r, mut p) = test_data!(r#"<?xml version="1.0"encoding="utf-8"?>"#);
        expect_event!(r, p, Err(_));

        let (mut r, mut p) = test_data!(r#"<?xml version="1.0"standalone="yes"?>"#);
        expect_event!(r, p, Err(_));

        let (mut r, mut p) = test_data!(r#"<?xml version="1.0" encoding="utf-8"standalone="yes"?>"#);
        expect_event!(r, p, Err(_));
    }

    #[test]
    fn opening_tag_in_attribute_value() {
        use crate::reader::error::{SyntaxError, Error, ErrorKind};

        let (mut r, mut p) = test_data!(r#"
            <a attr="zzz<zzz" />
        "#);

        expect_event!(r, p, Ok(XmlEvent::StartDocument { .. }));
        expect_event!(r, p, Err(ref e) =>
            *e == Error {
                kind: ErrorKind::Syntax(SyntaxError::UnexpectedOpeningTag.to_cow()),
                pos: TextPosition { row: 1, column: 24 }
            }
        );
    }

    #[test]
    fn processing_instruction_in_attribute_value() {
        use crate::reader::error::{SyntaxError, Error, ErrorKind};

        let (mut r, mut p) = test_data!(r#"
            <y F="<?abc"><x G="/">
        "#);

        expect_event!(r, p, Ok(XmlEvent::StartDocument { .. }));
        expect_event!(r, p, Err(ref e) =>
            *e == Error {
                kind: ErrorKind::Syntax(SyntaxError::UnexpectedOpeningTag.to_cow()),
                pos: TextPosition { row: 1, column: 18 }
            }
        );
    }

    #[test]
    fn reference_err() {
        let (mut r, mut p) = test_data!(r"
            <a>&&amp;</a>
        ");

        expect_event!(r, p, Ok(XmlEvent::StartDocument { .. }));
        expect_event!(r, p, Ok(XmlEvent::StartElement { .. }));
        expect_event!(r, p, Err(_));
    }

    #[test]
    fn state_size() {
        assert_eq!(2, std::mem::size_of::<super::State>());
        assert_eq!(1, std::mem::size_of::<super::DoctypeSubstate>());
    }
}
