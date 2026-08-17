extern crate xmlparser as xml;

use crate::token::*;

test!(cdata_01, "<p><![CDATA[content]]></p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("content", 3..22),
    Token::ElementEnd(ElementEnd::Close("", "p"), 22..26)
);

test!(cdata_02, "<p><![CDATA[&amping]]></p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("&amping", 3..22),
    Token::ElementEnd(ElementEnd::Close("", "p"), 22..26)
);

test!(cdata_03, "<p><![CDATA[&amping ]]]></p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("&amping ]", 3..24),
    Token::ElementEnd(ElementEnd::Close("", "p"), 24..28)
);

test!(cdata_04, "<p><![CDATA[&amping]] ]]></p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("&amping]] ", 3..25),
    Token::ElementEnd(ElementEnd::Close("", "p"), 25..29)
);

test!(cdata_05, "<p><![CDATA[<message>text</message>]]></p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("<message>text</message>", 3..38),
    Token::ElementEnd(ElementEnd::Close("", "p"), 38..42)
);

test!(cdata_06, "<p><![CDATA[</this is malformed!</malformed</malformed & worse>]]></p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("</this is malformed!</malformed</malformed & worse>", 3..66),
    Token::ElementEnd(ElementEnd::Close("", "p"), 66..70)
);

test!(cdata_07, "<p><![CDATA[1]]><![CDATA[2]]></p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("1", 3..16),
    Token::Cdata("2", 16..29),
    Token::ElementEnd(ElementEnd::Close("", "p"), 29..33)
);

test!(cdata_08, "<p> \n <![CDATA[data]]> \t </p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text(" \n ", 3..6),
    Token::Cdata("data", 6..22),
    Token::Text(" \t ", 22..25),
    Token::ElementEnd(ElementEnd::Close("", "p"), 25..29)
);

test!(cdata_09, "<p><![CDATA[bracket ]after]]></p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Cdata("bracket ]after", 3..29),
    Token::ElementEnd(ElementEnd::Close("", "p"), 29..33)
);

test!(cdata_err_01, "<p><![CDATA[\u{1}]]></p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Error("invalid CDATA at 1:4 cause a non-XML character '\\u{1}' found at 1:13".to_string())
);
