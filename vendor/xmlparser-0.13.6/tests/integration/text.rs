use crate::token::*;

test!(text_01, "<p>text</p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text("text", 3..7),
    Token::ElementEnd(ElementEnd::Close("", "p"), 7..11)
);

test!(text_02, "<p> text </p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text(" text ", 3..9),
    Token::ElementEnd(ElementEnd::Close("", "p"), 9..13)
);

// 欄 is EF A4 9D. And EF can be mistreated for UTF-8 BOM.
test!(text_03, "<p>欄</p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text("欄", 3..6),
    Token::ElementEnd(ElementEnd::Close("", "p"), 6..10)
);

test!(text_04, "<p> </p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text(" ", 3..4),
    Token::ElementEnd(ElementEnd::Close("", "p"), 4..8)
);

test!(text_05, "<p> \r\n\t </p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text(" \r\n\t ", 3..8),
    Token::ElementEnd(ElementEnd::Close("", "p"), 8..12)
);

test!(text_06, "<p>&#x20;</p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text("&#x20;", 3..9),
    Token::ElementEnd(ElementEnd::Close("", "p"), 9..13)
);

test!(text_07, "<p>]></p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Text("]>", 3..5),
    Token::ElementEnd(ElementEnd::Close("", "p"), 5..9)
);

test!(text_err_01, "<p>]]></p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Error("invalid character data at 1:4 cause ']]>' is not allowed inside a character data".to_string())
);

test!(text_err_02, "<p>\u{0c}</p>",
    Token::ElementStart("", "p", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Error("invalid character data at 1:4 cause a non-XML character '\\u{c}' found at 1:4".to_string())
);
