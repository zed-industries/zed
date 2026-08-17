use std::str;

use crate::token::*;

test!(document_01, "", );

test!(document_02, "    ", );

test!(document_03, " \n\t\r ", );

// BOM
test!(document_05, str::from_utf8(b"\xEF\xBB\xBF<a/>").unwrap(),
    Token::ElementStart("", "a", 3..5),
    Token::ElementEnd(ElementEnd::Empty, 5..7)
);

test!(document_06, str::from_utf8(b"\xEF\xBB\xBF<?xml version='1.0'?>").unwrap(),
    Token::Declaration("1.0", None, None, 3..24)
);

test!(document_07, "<?xml version='1.0' encoding='utf-8'?>\n<!-- comment -->\n\
<!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'>",
    Token::Declaration("1.0", Some("utf-8"), None, 0..38),
    Token::Comment(" comment ", 39..55),
    Token::EmptyDtd("svg", Some(ExternalId::Public(
        "-//W3C//DTD SVG 1.1//EN",
        "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"
    )), 56..154)
);

test!(document_08, "<?xml-stylesheet?>\n\
<!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'>",
    Token::PI("xml-stylesheet", None, 0..18),
    Token::EmptyDtd("svg", Some(ExternalId::Public(
        "-//W3C//DTD SVG 1.1//EN",
        "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"
    )), 19..117)
);

test!(document_09, "<?xml version='1.0' encoding='utf-8'?>\n<?xml-stylesheet?>\n\
<!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'>",
    Token::Declaration("1.0", Some("utf-8"), None, 0..38),
    Token::PI("xml-stylesheet", None, 39..57),
    Token::EmptyDtd("svg", Some(ExternalId::Public(
        "-//W3C//DTD SVG 1.1//EN",
        "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"
    )), 58..156)
);

test!(document_err_01, "<![CDATA[text]]>",
    Token::Error("unknown token at 1:1".to_string())
);

test!(document_err_02, " &www---------Ӥ+----------w-----www_",
    Token::Error("unknown token at 1:2".to_string())
);

test!(document_err_03, "q",
    Token::Error("unknown token at 1:1".to_string())
);

test!(document_err_04, "<!>",
    Token::Error("unknown token at 1:1".to_string())
);

test!(document_err_05, "<!DOCTYPE greeting1><!DOCTYPE greeting2>",
    Token::EmptyDtd("greeting1", None, 0..20),
    Token::Error("unknown token at 1:21".to_string())
);

test!(document_err_06, "&#x20;",
    Token::Error("unknown token at 1:1".to_string())
);

#[test]
fn parse_fragment_1() {
    let s = "<p/><p/>";
    let mut p = xml::Tokenizer::from_fragment(s, 0..s.len());

    match p.next().unwrap().unwrap() {
        xml::Token::ElementStart { local, .. } => assert_eq!(local.as_str(), "p"),
        _ => panic!(),
    }

    match p.next().unwrap().unwrap() {
        xml::Token::ElementEnd { .. } => {}
        _ => panic!(),
    }

    match p.next().unwrap().unwrap() {
        xml::Token::ElementStart { local, .. } => assert_eq!(local.as_str(), "p"),
        _ => panic!(),
    }
}
