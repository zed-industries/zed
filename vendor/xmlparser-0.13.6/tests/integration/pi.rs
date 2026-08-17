use crate::token::*;

test!(pi_01, "<?xslt ma?>",
    Token::PI("xslt", Some("ma"), 0..11)
);

test!(pi_02, "<?xslt \t\n m?>",
    Token::PI("xslt", Some("m"), 0..13)
);

test!(pi_03, "<?xslt?>",
    Token::PI("xslt", None, 0..8)
);

test!(pi_04, "<?xslt ?>",
    Token::PI("xslt", None, 0..9)
);

test!(pi_05, "<?xml-stylesheet?>",
    Token::PI("xml-stylesheet", None, 0..18)
);

test!(pi_err_01, "<??xml \t\n m?>",
    Token::Error("invalid processing instruction at 1:1 cause invalid name token".to_string())
);

test!(declaration_01, "<?xml version=\"1.0\"?>",
    Token::Declaration("1.0", None, None, 0..21)
);

test!(declaration_02, "<?xml version='1.0'?>",
    Token::Declaration("1.0", None, None, 0..21)
);

test!(declaration_03, "<?xml version='1.0' encoding=\"UTF-8\"?>",
    Token::Declaration("1.0", Some("UTF-8"), None, 0..38)
);

test!(declaration_04, "<?xml version='1.0' encoding='UTF-8'?>",
    Token::Declaration("1.0", Some("UTF-8"), None, 0..38)
);

test!(declaration_05, "<?xml version='1.0' encoding='utf-8'?>",
    Token::Declaration("1.0", Some("utf-8"), None, 0..38)
);

test!(declaration_06, "<?xml version='1.0' encoding='EUC-JP'?>",
    Token::Declaration("1.0", Some("EUC-JP"), None, 0..39)
);

test!(declaration_07, "<?xml version='1.0' encoding='UTF-8' standalone='yes'?>",
    Token::Declaration("1.0", Some("UTF-8"), Some(true), 0..55)
);

test!(declaration_08, "<?xml version='1.0' encoding='UTF-8' standalone='no'?>",
    Token::Declaration("1.0", Some("UTF-8"), Some(false), 0..54)
);

test!(declaration_09, "<?xml version='1.0' standalone='no'?>",
    Token::Declaration("1.0", None, Some(false), 0..37)
);

test!(declaration_10, "<?xml version='1.0' standalone='no' ?>",
    Token::Declaration("1.0", None, Some(false), 0..38)
);

// Declaration with an invalid order
test!(declaration_err_01, "<?xml encoding='UTF-8' version='1.0'?>",
    Token::Error("invalid XML declaration at 1:1 cause expected 'version' at 1:7".to_string())
);

test!(declaration_err_02, "<?xml version='1.0' encoding='*invalid*'?>",
    Token::Error("invalid XML declaration at 1:1 cause expected '\'' not '*' at 1:31".to_string())
);

test!(declaration_err_03, "<?xml version='2.0'?>",
    Token::Error("invalid XML declaration at 1:1 cause expected '1.' at 1:16".to_string())
);

test!(declaration_err_04, "<?xml version='1.0' standalone='true'?>",
    Token::Error("invalid XML declaration at 1:1 cause expected 'yes', 'no' at 1:33".to_string())
);

test!(declaration_err_05, "<?xml version='1.0' yes='true'?>",
    Token::Error("invalid XML declaration at 1:1 cause expected '?>' at 1:21".to_string())
);

test!(declaration_err_06, "<?xml version='1.0' encoding='UTF-8' standalone='yes' yes='true'?>",
    Token::Error("invalid XML declaration at 1:1 cause expected '?>' at 1:55".to_string())
);

test!(declaration_err_07, "\u{000a}<?xml\u{000a}&jg'];",
    Token::Error("invalid processing instruction at 2:1 cause expected '?>' at 3:7".to_string())
);

test!(declaration_err_08, "<?xml \t\n ?m?>",
    Token::Error("invalid XML declaration at 1:1 cause expected 'version' at 2:2".to_string())
);

test!(declaration_err_09, "<?xml \t\n m?>",
    Token::Error("invalid XML declaration at 1:1 cause expected 'version' at 2:2".to_string())
);

// XML declaration allowed only at the start of the document.
test!(declaration_err_10, " <?xml version='1.0'?>",
    Token::Error("unknown token at 1:2".to_string())
);

// XML declaration allowed only at the start of the document.
test!(declaration_err_11, "<!-- comment --><?xml version='1.0'?>",
    Token::Comment(" comment ", 0..16),
    Token::Error("unknown token at 1:17".to_string())
);

// Duplicate.
test!(declaration_err_12, "<?xml version='1.0'?><?xml version='1.0'?>",
    Token::Declaration("1.0", None, None, 0..21),
    Token::Error("unknown token at 1:22".to_string())
);

test!(declaration_err_13, "<?target \u{1}content>",
    Token::Error("invalid processing instruction at 1:1 cause a non-XML character '\\u{1}' found at 1:10".to_string())
);

test!(declaration_err_14, "<?xml version='1.0'encoding='UTF-8'?>",
    Token::Error("invalid XML declaration at 1:1 cause expected space not 'e' at 1:20".to_string())
);

test!(declaration_err_15, "<?xml version='1.0' encoding='UTF-8'standalone='yes'?>",
    Token::Error("invalid XML declaration at 1:1 cause expected space not 's' at 1:37".to_string())
);

test!(declaration_err_16, "<?xml version='1.0'",
    Token::Error("invalid XML declaration at 1:1 cause expected '?>' at 1:20".to_string())
);
