use std::str;
use std::string::{String, ToString};
use std::vec::Vec;

use crate::tokenizer as xml;

#[test]
fn text_pos_1() {
    let mut s = xml::Stream::new("text");
    s.advance(2);
    assert_eq!(s.gen_text_pos(), crate::TextPos::new(1, 3));
}

#[test]
fn text_pos_2() {
    let mut s = xml::Stream::new("text\ntext");
    s.advance(6);
    assert_eq!(s.gen_text_pos(), crate::TextPos::new(2, 2));
}

#[test]
fn text_pos_3() {
    let mut s = xml::Stream::new("текст\nтекст");
    s.advance(15);
    assert_eq!(s.gen_text_pos(), crate::TextPos::new(2, 3));
}

#[test]
fn token_size() {
    assert!(::std::mem::size_of::<Token>() <= 196);
}

#[test]
fn span_size() {
    assert!(::std::mem::size_of::<xml::StrSpan>() <= 48);
}

type Range = ::std::ops::Range<usize>;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    PI(&'a str, Option<&'a str>, Range),
    Comment(&'a str, Range),
    EntityDecl(&'a str, &'a str),
    ElementStart(&'a str, &'a str, usize),
    Attribute(&'a str, &'a str, &'a str),
    ElementEnd(ElementEnd<'a>, Range),
    Text(&'a str, Range),
    Cdata(&'a str, Range),
    Error(String),
}

#[derive(PartialEq, Debug)]
pub enum ElementEnd<'a> {
    Open,
    Close(&'a str, &'a str),
    Empty,
}

#[macro_export]
macro_rules! test {
    ($name:ident, $text:expr, $($token:expr),*) => (
        #[test]
        fn $name() {
            let tokens = collect_tokens($text);
            let mut iter = tokens.iter();
            $(
                let t = iter.next().unwrap();
                assert_eq!(*t, $token);
            )*
            assert!(iter.next().is_none());
        }
    )
}

struct EventsCollector<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> xml::XmlEvents<'a> for EventsCollector<'a> {
    fn token(&mut self, token: xml::Token<'a>) -> Result<(), crate::Error> {
        let t = match token {
            xml::Token::ProcessingInstruction(target, content, range) => {
                Token::PI(target, content, range)
            }
            xml::Token::Comment(text, range) => Token::Comment(text, range),
            xml::Token::EntityDeclaration(name, definition) => {
                Token::EntityDecl(name, definition.as_str())
            }
            xml::Token::ElementStart(prefix, local, start) => {
                Token::ElementStart(prefix, local, start)
            }
            xml::Token::Attribute(_, _, _, prefix, local, value) => {
                Token::Attribute(prefix, local, value.as_str())
            }
            xml::Token::ElementEnd(end, range) => Token::ElementEnd(
                match end {
                    xml::ElementEnd::Open => ElementEnd::Open,
                    xml::ElementEnd::Close(prefix, local) => ElementEnd::Close(prefix, local),
                    xml::ElementEnd::Empty => ElementEnd::Empty,
                },
                range,
            ),
            xml::Token::Text(text, range) => Token::Text(text, range),
            xml::Token::Cdata(text, range) => Token::Cdata(text, range),
        };
        self.tokens.push(t);
        Ok(())
    }
}

#[inline(never)]
pub fn collect_tokens(text: &str) -> Vec<Token> {
    let mut collector = EventsCollector { tokens: Vec::new() };
    if let Err(e) = xml::parse(text, true, &mut collector) {
        collector.tokens.push(Token::Error(e.to_string()));
    }
    collector.tokens
}

// CDATA

test!(
    cdata_01,
    "<p><![CDATA[content]]></p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("content", 3..22),
    Token::ElementEnd(ElementEnd::Close("", "p"), 22..26)
);

test!(
    cdata_02,
    "<p><![CDATA[&amping]]></p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("&amping", 3..22),
    Token::ElementEnd(ElementEnd::Close("", "p"), 22..26)
);

test!(
    cdata_03,
    "<p><![CDATA[&amping ]]]></p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("&amping ]", 3..24),
    Token::ElementEnd(ElementEnd::Close("", "p"), 24..28)
);

test!(
    cdata_04,
    "<p><![CDATA[&amping]] ]]></p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("&amping]] ", 3..25),
    Token::ElementEnd(ElementEnd::Close("", "p"), 25..29)
);

test!(
    cdata_05,
    "<p><![CDATA[<message>text</message>]]></p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("<message>text</message>", 3..38),
    Token::ElementEnd(ElementEnd::Close("", "p"), 38..42)
);

test!(
    cdata_06,
    "<p><![CDATA[</this is malformed!</malformed</malformed & worse>]]></p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("</this is malformed!</malformed</malformed & worse>", 3..66),
    Token::ElementEnd(ElementEnd::Close("", "p"), 66..70)
);

test!(
    cdata_07,
    "<p><![CDATA[1]]><![CDATA[2]]></p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("1", 3..16),
    Token::Cdata("2", 16..29),
    Token::ElementEnd(ElementEnd::Close("", "p"), 29..33)
);

test!(
    cdata_08,
    "<p> \n <![CDATA[data]]> \t </p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text(" \n ", 3..6),
    Token::Cdata("data", 6..22),
    Token::Text(" \t ", 22..25),
    Token::ElementEnd(ElementEnd::Close("", "p"), 25..29)
);

test!(
    cdata_09,
    "<p><![CDATA[bracket ]after]]></p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("bracket ]after", 3..29),
    Token::ElementEnd(ElementEnd::Close("", "p"), 29..33)
);

test!(
    cdata_err_01,
    "<p><![CDATA[\u{1}]]></p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Error("a non-XML character '\\u{1}' found at 1:13".to_string())
);

// Comments

test!(
    comment_01,
    "<!--comment-->",
    Token::Comment("comment", 0..14)
);
test!(comment_02, "<!--<head>-->", Token::Comment("<head>", 0..13));
test!(comment_03, "<!--<!-x-->", Token::Comment("<!-x", 0..11));
test!(comment_04, "<!--<!x-->", Token::Comment("<!x", 0..10));
test!(comment_05, "<!--<<!x-->", Token::Comment("<<!x", 0..11));
test!(comment_06, "<!--<<!-x-->", Token::Comment("<<!-x", 0..12));
test!(comment_07, "<!--<x-->", Token::Comment("<x", 0..9));
test!(comment_08, "<!--<>-->", Token::Comment("<>", 0..9));
test!(comment_09, "<!--<-->", Token::Comment("<", 0..8));
test!(comment_10, "<!--<!-->", Token::Comment("<!", 0..9));
test!(comment_11, "<!---->", Token::Comment("", 0..7));

macro_rules! test_err {
    ($name:ident, $text:expr) => {
        #[test]
        fn $name() {
            let mut collector = EventsCollector { tokens: Vec::new() };
            assert!(xml::parse($text, true, &mut collector).is_err());
        }
    };
}

test_err!(comment_err_01, "<!----!>");
test_err!(comment_err_02, "<!----!");
test_err!(comment_err_03, "<!----");
test_err!(comment_err_04, "<!--->");
test_err!(comment_err_05, "<!-----");
test_err!(comment_err_06, "<!-->");
test_err!(comment_err_07, "<!--");
test_err!(comment_err_08, "<!--x");
test_err!(comment_err_09, "<!--<");
test_err!(comment_err_10, "<!--<!");
test_err!(comment_err_11, "<!--<!-");
test_err!(comment_err_12, "<!--<!--");
test_err!(comment_err_13, "<!--<!--!");
test_err!(comment_err_14, "<!--<!--!>");
test_err!(comment_err_15, "<!--<!---");
test_err!(comment_err_16, "<!--<!--x");
test_err!(comment_err_17, "<!--<!--x-");
test_err!(comment_err_18, "<!--<!--x--");
test_err!(comment_err_19, "<!--<!--x-->");
test_err!(comment_err_20, "<!--<!-x");
test_err!(comment_err_21, "<!--<!-x-");
test_err!(comment_err_22, "<!--<!-x--");
test_err!(comment_err_23, "<!--<!x");
test_err!(comment_err_24, "<!--<!x-");
test_err!(comment_err_25, "<!--<!x--");
test_err!(comment_err_26, "<!--<<!--x-->");
test_err!(comment_err_27, "<!--<!<!--x-->");
test_err!(comment_err_28, "<!--<!-<!--x-->");
test_err!(comment_err_29, "<!----!->");
test_err!(comment_err_30, "<!----!x>");
test_err!(comment_err_31, "<!-----x>");
test_err!(comment_err_32, "<!----->");
test_err!(comment_err_33, "<!------>");
test_err!(comment_err_34, "<!-- --->");
test_err!(comment_err_35, "<!--a--->");

// DTD

test!(
    dtd_01,
    "<!DOCTYPE greeting SYSTEM \"hello.dtd\">",
    // nothing to parse
);

test!(
    dtd_02,
    "<!DOCTYPE greeting PUBLIC \"hello.dtd\" \"goodbye.dtd\">",
    // nothing to parse
);

test!(
    dtd_03,
    "<!DOCTYPE greeting SYSTEM 'hello.dtd'>",
    // nothing to parse
);

test!(
    dtd_04,
    "<!DOCTYPE greeting>",
    // nothing to parse
);

test!(dtd_05, "<!DOCTYPE greeting []>",);

test!(
    dtd_06,
    "<!DOCTYPE greeting><a/>",
    Token::ElementStart("", "a", 19),
    Token::ElementEnd(ElementEnd::Empty, 21..23)
);

test!(dtd_07, "<!DOCTYPE greeting [] >",);

test!(dtd_08, "<!DOCTYPE greeting [ ] >",);

test!(
    dtd_entity_01,
    "<!DOCTYPE svg [
    <!ENTITY ns_extend \"http://ns.adobe.com/Extensibility/1.0/\">
]>",
    Token::EntityDecl("ns_extend", "http://ns.adobe.com/Extensibility/1.0/")
);

test!(
    dtd_entity_02,
    "<!DOCTYPE svg [
    <!ENTITY Pub-Status \"This is a pre-release of the
specification.\">
]>",
    Token::EntityDecl("Pub-Status", "This is a pre-release of the\nspecification.")
);

test!(
    dtd_entity_03,
    "<!DOCTYPE svg [
    <!ENTITY open-hatch SYSTEM \"http://www.textuality.com/boilerplate/OpenHatch.xml\">
]>",
);

test!(
    dtd_entity_04,
    "<!DOCTYPE svg [
    <!ENTITY open-hatch
             PUBLIC \"-//Textuality//TEXT Standard open-hatch boilerplate//EN\"
             \"http://www.textuality.com/boilerplate/OpenHatch.xml\">
]>",
);

// TODO: NDATA will be ignored
test!(
    dtd_entity_05,
    "<!DOCTYPE svg [
    <!ENTITY hatch-pic SYSTEM \"../grafix/OpenHatch.gif\" NDATA gif >
]>",
);

// TODO: unsupported data will be ignored
test!(
    dtd_entity_06,
    "<!DOCTYPE svg [
    <!ELEMENT sgml ANY>
    <!ENTITY ns_extend \"http://ns.adobe.com/Extensibility/1.0/\">
    <!NOTATION example1SVG-rdf SYSTEM \"example1.svg.rdf\">
    <!ATTLIST img data ENTITY #IMPLIED>
]>",
    Token::EntityDecl("ns_extend", "http://ns.adobe.com/Extensibility/1.0/")
);

// We do not support !ELEMENT DTD token and it will be skipped.
// Previously, we were calling `Tokenizer::next` after the skip,
// which is recursive and could cause a stack overflow when there are too many sequential
// unsupported tokens.
// This tests checks that the current code do not crash with stack overflow.
#[test]
fn dtd_entity_07() {
    let mut text = "<!DOCTYPE svg [\n".to_string();
    for _ in 0..500 {
        text.push_str("<!ELEMENT sgml ANY>\n");
    }
    text.push_str("]>\n");

    let mut collector = EventsCollector { tokens: Vec::new() };
    xml::parse(&text, true, &mut collector).unwrap();
}

test!(
    dtd_err_01,
    "<!DOCTYPEEG[<!ENTITY%ETT\u{000a}SSSSSSSS<D_IDYT;->\u{000a}<",
    Token::Error("expected a whitespace not 'E' at 1:10".to_string())
);

test!(
    dtd_err_02,
    "<!DOCTYPE s [<!ENTITY % name S YSTEM",
    Token::Error("invalid ExternalID at 1:30".to_string())
);

test!(
    dtd_err_03,
    "<!DOCTYPE s [<!ENTITY % name B",
    Token::Error("expected a quote, SYSTEM or PUBLIC not 'B' at 1:30".to_string())
);

test!(
    dtd_err_04,
    "<!DOCTYPE s []",
    Token::Error("unexpected end of stream".to_string())
);

test!(
    dtd_err_05,
    "<!DOCTYPE s [] !",
    Token::Error("expected '>' not '!' at 1:16".to_string())
);

// Document

test!(document_01, "",);

test!(document_02, "    ",);

test!(document_03, " \n\t\r ",);

// BOM
test!(
    document_05,
    str::from_utf8(b"\xEF\xBB\xBF<a/>").unwrap(),
    Token::ElementStart("", "a", 3),
    Token::ElementEnd(ElementEnd::Empty, 5..7)
);

test!(
    document_06,
    str::from_utf8(b"\xEF\xBB\xBF<?xml version='1.0'?>").unwrap(),
);

test!(
    document_07,
    "<?xml version='1.0' encoding='utf-8'?>\n<!-- comment -->\n\
<!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'>",
    Token::Comment(" comment ", 39..55)
);

test!(
    document_08,
    "<?xml-stylesheet?>\n\
<!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'>",
    Token::PI("xml-stylesheet", None, 0..18)
);

test!(
    document_09,
    "<?xml version='1.0' encoding='utf-8'?>\n<?xml-stylesheet?>\n\
<!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'>",
    Token::PI("xml-stylesheet", None, 39..57)
);

// TODO: better error
test!(
    document_err_01,
    "<![CDATA[text]]>",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    document_err_02,
    " &www---------Ӥ+----------w-----www_",
    Token::Error("unknown token at 1:2".to_string())
);

test!(
    document_err_03,
    "q",
    Token::Error("unknown token at 1:1".to_string())
);

test!(
    document_err_04,
    "<!>",
    Token::Error("invalid name token at 1:2".to_string())
);

// TODO: better error
test!(
    document_err_05,
    "<!DOCTYPE greeting1><!DOCTYPE greeting2>",
    Token::Error("invalid name token at 1:22".to_string())
);

test!(
    document_err_06,
    "&#x20;",
    Token::Error("unknown token at 1:1".to_string())
);

// Elements

test!(
    element_01,
    "<a/>",
    Token::ElementStart("", "a", 0),
    Token::ElementEnd(ElementEnd::Empty, 2..4)
);

test!(
    element_02,
    "<a></a>",
    Token::ElementStart("", "a", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::ElementEnd(ElementEnd::Close("", "a"), 3..7)
);

test!(
    element_03,
    "  \t  <a/>   \n ",
    Token::ElementStart("", "a", 5),
    Token::ElementEnd(ElementEnd::Empty, 7..9)
);

test!(
    element_04,
    "  \t  <b><a/></b>   \n ",
    Token::ElementStart("", "b", 5),
    Token::ElementEnd(ElementEnd::Open, 7..8),
    Token::ElementStart("", "a", 8),
    Token::ElementEnd(ElementEnd::Empty, 10..12),
    Token::ElementEnd(ElementEnd::Close("", "b"), 12..16)
);

test!(
    element_06,
    "<俄语 լեզու=\"ռուսերեն\">данные</俄语>",
    Token::ElementStart("", "俄语", 0),
    Token::Attribute("", "լեզու", "ռուսերեն"),
    Token::ElementEnd(ElementEnd::Open, 37..38),
    Token::Text("данные", 38..50),
    Token::ElementEnd(ElementEnd::Close("", "俄语"), 50..59)
);

test!(
    element_07,
    "<svg:circle></svg:circle>",
    Token::ElementStart("svg", "circle", 0),
    Token::ElementEnd(ElementEnd::Open, 11..12),
    Token::ElementEnd(ElementEnd::Close("svg", "circle"), 12..25)
);

test!(
    element_08,
    "<:circle/>",
    Token::ElementStart("", "circle", 0),
    Token::ElementEnd(ElementEnd::Empty, 8..10)
);

test!(
    element_err_01,
    "<>",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_02,
    "</",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_03,
    "</a",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_04,
    "<a x='test' /",
    Token::ElementStart("", "a", 0),
    Token::Attribute("", "x", "test"),
    Token::Error("unexpected end of stream".to_string())
);

test!(
    element_err_05,
    "<<",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_06,
    "< a",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_07,
    "< ",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_08,
    "<&#x9;",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_09,
    "<a></a></a>",
    Token::ElementStart("", "a", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::ElementEnd(ElementEnd::Close("", "a"), 3..7),
    Token::Error("unknown token at 1:8".to_string())
);

test!(
    element_err_10,
    "<a/><a/>",
    Token::ElementStart("", "a", 0),
    Token::ElementEnd(ElementEnd::Empty, 2..4),
    Token::Error("unknown token at 1:5".to_string())
);

test!(
    element_err_11,
    "<a></br/></a>",
    Token::ElementStart("", "a", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Error("expected '>' not '/' at 1:8".to_string())
);

test!(
    element_err_12,
    "<svg:/>",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_13,
    "\
<root>
</root>
</root>",
    Token::ElementStart("", "root", 0),
    Token::ElementEnd(ElementEnd::Open, 5..6),
    Token::Text("\n", 6..7),
    Token::ElementEnd(ElementEnd::Close("", "root"), 7..14),
    Token::Error("unknown token at 3:1".to_string())
);

test!(
    element_err_14,
    "<-svg/>",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_15,
    "<svg:-svg/>",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_16,
    "<svg::svg/>",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_17,
    "<svg:s:vg/>",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_18,
    "<::svg/>",
    Token::Error("invalid name token at 1:2".to_string())
);

test!(
    element_err_19,
    "<a><",
    Token::ElementStart("", "a", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Error("unknown token at 1:4".to_string())
);

test!(
    attribute_01,
    "<a ax=\"test\"/>",
    Token::ElementStart("", "a", 0),
    Token::Attribute("", "ax", "test"),
    Token::ElementEnd(ElementEnd::Empty, 12..14)
);

test!(
    attribute_02,
    "<a ax='test'/>",
    Token::ElementStart("", "a", 0),
    Token::Attribute("", "ax", "test"),
    Token::ElementEnd(ElementEnd::Empty, 12..14)
);

test!(
    attribute_03,
    "<a b='test1' c=\"test2\"/>",
    Token::ElementStart("", "a", 0),
    Token::Attribute("", "b", "test1"),
    Token::Attribute("", "c", "test2"),
    Token::ElementEnd(ElementEnd::Empty, 22..24)
);

test!(
    attribute_04,
    "<a b='\"test1\"' c=\"'test2'\"/>",
    Token::ElementStart("", "a", 0),
    Token::Attribute("", "b", "\"test1\""),
    Token::Attribute("", "c", "'test2'"),
    Token::ElementEnd(ElementEnd::Empty, 26..28)
);

test!(
    attribute_05,
    "<c a=\"test1' c='test2\" b='test1\" c=\"test2'/>",
    Token::ElementStart("", "c", 0),
    Token::Attribute("", "a", "test1' c='test2"),
    Token::Attribute("", "b", "test1\" c=\"test2"),
    Token::ElementEnd(ElementEnd::Empty, 42..44)
);

test!(
    attribute_06,
    "<c   a   =    'test1'     />",
    Token::ElementStart("", "c", 0),
    Token::Attribute("", "a", "test1"),
    Token::ElementEnd(ElementEnd::Empty, 26..28)
);

test!(
    attribute_07,
    "<c q:a='b'/>",
    Token::ElementStart("", "c", 0),
    Token::Attribute("q", "a", "b"),
    Token::ElementEnd(ElementEnd::Empty, 10..12)
);

test!(
    attribute_err_01,
    "<c az=test>",
    Token::ElementStart("", "c", 0),
    Token::Error("expected a quote not 't' at 1:7".to_string())
);

test!(
    attribute_err_02,
    "<c a>",
    Token::ElementStart("", "c", 0),
    Token::Error("expected \'=\' not \'>\' at 1:5".to_string())
);

test!(
    attribute_err_03,
    "<c a/>",
    Token::ElementStart("", "c", 0),
    Token::Error("expected '=' not '/' at 1:5".to_string())
);

test!(
    attribute_err_04,
    "<c a='b' q/>",
    Token::ElementStart("", "c", 0),
    Token::Attribute("", "a", "b"),
    Token::Error("expected '=' not '/' at 1:11".to_string())
);

test!(
    attribute_err_05,
    "<c a='<'/>",
    Token::ElementStart("", "c", 0),
    Token::Error("expected ''' not '<' at 1:7".to_string())
);

test!(
    attribute_err_06,
    "<c a='\u{1}'/>",
    Token::ElementStart("", "c", 0),
    Token::Error("a non-XML character '\\u{1}' found at 1:7".to_string())
);

test!(
    attribute_err_07,
    "<c a='v'b='v'/>",
    Token::ElementStart("", "c", 0),
    Token::Attribute("", "a", "v"),
    Token::Error("expected a whitespace not 'b' at 1:9".to_string())
);

// PI

test!(pi_01, "<?xslt ma?>", Token::PI("xslt", Some("ma"), 0..11));

test!(
    pi_02,
    "<?xslt \t\n m?>",
    Token::PI("xslt", Some("m"), 0..13)
);

test!(pi_03, "<?xslt?>", Token::PI("xslt", None, 0..8));

test!(pi_04, "<?xslt ?>", Token::PI("xslt", None, 0..9));

test!(
    pi_05,
    "<?xml-stylesheet?>",
    Token::PI("xml-stylesheet", None, 0..18)
);

test!(
    pi_err_01,
    "<??xml \t\n m?>",
    Token::Error("invalid name token at 1:3".to_string())
);

test!(declaration_01, "<?xml version=\"1.0\"?>",);

test!(declaration_02, "<?xml version='1.0'?>",);

test!(declaration_03, "<?xml version='1.0' encoding=\"UTF-8\"?>",);

test!(declaration_04, "<?xml version='1.0' encoding='UTF-8'?>",);

test!(declaration_05, "<?xml version='1.0' encoding='utf-8'?>",);

test!(declaration_06, "<?xml version='1.0' encoding='EUC-JP'?>",);

test!(
    declaration_07,
    "<?xml version='1.0' encoding='UTF-8' standalone='yes'?>",
);

test!(
    declaration_08,
    "<?xml version='1.0' encoding='UTF-8' standalone='no'?>",
);

test!(declaration_09, "<?xml version='1.0' standalone='no'?>",);

test!(declaration_10, "<?xml version='1.0' standalone='no' ?>",);

// Declaration with an invalid order
test!(
    declaration_err_01,
    "<?xml encoding='UTF-8' version='1.0'?>",
    Token::Error("expected 'version' at 1:7".to_string())
);

test!(
    declaration_err_05,
    "<?xml version='1.0' yes='true'?>",
    Token::Error("expected '?>' at 1:21".to_string())
);

test!(
    declaration_err_06,
    "<?xml version='1.0' encoding='UTF-8' standalone='yes' yes='true'?>",
    Token::Error("expected '?>' at 1:55".to_string())
);

test!(
    declaration_err_07,
    "\u{000a}<?xml\u{000a}&jg'];",
    Token::Error("expected '?>' at 3:7".to_string())
);

test!(
    declaration_err_08,
    "<?xml \t\n ?m?>",
    Token::Error("expected 'version' at 2:2".to_string())
);

test!(
    declaration_err_09,
    "<?xml \t\n m?>",
    Token::Error("expected 'version' at 2:2".to_string())
);

// XML declaration allowed only at the start of the document.
test!(
    declaration_err_10,
    " <?xml version='1.0'?>",
    Token::Error("unexpected XML declaration at 1:2".to_string())
);

// XML declaration allowed only at the start of the document.
test!(
    declaration_err_11,
    "<!-- comment --><?xml version='1.0'?>",
    Token::Comment(" comment ", 0..16),
    Token::Error("unexpected XML declaration at 1:17".to_string())
);

// Duplicate.
test!(
    declaration_err_12,
    "<?xml version='1.0'?><?xml version='1.0'?>",
    Token::Error("unexpected XML declaration at 1:22".to_string())
);

test!(
    declaration_err_13,
    "<?target \u{1}content>",
    Token::Error("a non-XML character '\\u{1}' found at 1:10".to_string())
);

test!(
    declaration_err_14,
    "<?xml version='1.0'encoding='UTF-8'?>",
    Token::Error("expected a whitespace not 'e' at 1:20".to_string())
);

test!(
    declaration_err_15,
    "<?xml version='1.0' encoding='UTF-8'standalone='yes'?>",
    Token::Error("expected a whitespace not 's' at 1:37".to_string())
);

test!(
    declaration_err_16,
    "<?xml version='1.0'",
    Token::Error("expected '?>' at 1:20".to_string())
);

// Text

test!(
    text_01,
    "<p>text</p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text("text", 3..7),
    Token::ElementEnd(ElementEnd::Close("", "p"), 7..11)
);

test!(
    text_02,
    "<p> text </p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text(" text ", 3..9),
    Token::ElementEnd(ElementEnd::Close("", "p"), 9..13)
);

// 欄 is EF A4 9D. And EF can be mistreated for UTF-8 BOM.
test!(
    text_03,
    "<p>欄</p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text("欄", 3..6),
    Token::ElementEnd(ElementEnd::Close("", "p"), 6..10)
);

test!(
    text_04,
    "<p> </p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text(" ", 3..4),
    Token::ElementEnd(ElementEnd::Close("", "p"), 4..8)
);

test!(
    text_05,
    "<p> \r\n\t </p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text(" \r\n\t ", 3..8),
    Token::ElementEnd(ElementEnd::Close("", "p"), 8..12)
);

test!(
    text_06,
    "<p>&#x20;</p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text("&#x20;", 3..9),
    Token::ElementEnd(ElementEnd::Close("", "p"), 9..13)
);

test!(
    text_07,
    "<p>]></p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text("]>", 3..5),
    Token::ElementEnd(ElementEnd::Close("", "p"), 5..9)
);

test!(
    text_err_01,
    "<p>]]></p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Error("']]>' at 1:7 is not allowed inside a character data".to_string())
);

test!(
    text_err_02,
    "<p>\u{0c}</p>",
    Token::ElementStart("", "p", 0),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Error("a non-XML character '\\u{c}' found at 1:4".to_string())
);
