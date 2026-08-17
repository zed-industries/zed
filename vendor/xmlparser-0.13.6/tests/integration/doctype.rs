use crate::token::*;

test!(dtd_01, "<!DOCTYPE greeting SYSTEM \"hello.dtd\">",
    Token::EmptyDtd("greeting", Some(ExternalId::System("hello.dtd")), 0..38)
);

test!(dtd_02, "<!DOCTYPE greeting PUBLIC \"hello.dtd\" \"goodbye.dtd\">",
    Token::EmptyDtd("greeting", Some(ExternalId::Public("hello.dtd", "goodbye.dtd")), 0..52)
);

test!(dtd_03, "<!DOCTYPE greeting SYSTEM 'hello.dtd'>",
    Token::EmptyDtd("greeting", Some(ExternalId::System("hello.dtd")), 0..38)
);

test!(dtd_04, "<!DOCTYPE greeting>",
    Token::EmptyDtd("greeting", None, 0..19)
);

test!(dtd_05, "<!DOCTYPE greeting []>",
    Token::DtdStart("greeting", None, 0..20),
    Token::DtdEnd(20..22)
);

test!(dtd_06, "<!DOCTYPE greeting><a/>",
    Token::EmptyDtd("greeting", None, 0..19),
    Token::ElementStart("", "a", 19..21),
    Token::ElementEnd(ElementEnd::Empty, 21..23)
);

test!(dtd_07, "<!DOCTYPE greeting [] >",
    Token::DtdStart("greeting", None, 0..20),
    Token::DtdEnd(20..23)
);

test!(dtd_08, "<!DOCTYPE greeting [ ] >",
    Token::DtdStart("greeting", None, 0..20),
    Token::DtdEnd(21..24)
);

test!(dtd_entity_01,
"<!DOCTYPE svg [
    <!ENTITY ns_extend \"http://ns.adobe.com/Extensibility/1.0/\">
]>",
    Token::DtdStart("svg", None, 0..15),
    Token::EntityDecl(
        "ns_extend",
        EntityDefinition::EntityValue("http://ns.adobe.com/Extensibility/1.0/"),
        20..80,
    ),
    Token::DtdEnd(81..83)
);

test!(dtd_entity_02,
"<!DOCTYPE svg [
    <!ENTITY Pub-Status \"This is a pre-release of the
specification.\">
]>",
    Token::DtdStart("svg", None, 0..15),
    Token::EntityDecl(
        "Pub-Status",
        EntityDefinition::EntityValue("This is a pre-release of the\nspecification."),
        20..86,
    ),
    Token::DtdEnd(87..89)
);

test!(dtd_entity_03,
"<!DOCTYPE svg [
    <!ENTITY open-hatch SYSTEM \"http://www.textuality.com/boilerplate/OpenHatch.xml\">
]>",
    Token::DtdStart("svg", None, 0..15),
    Token::EntityDecl(
        "open-hatch",
        EntityDefinition::ExternalId(ExternalId::System("http://www.textuality.com/boilerplate/OpenHatch.xml")),
        20..101,
    ),
    Token::DtdEnd(102..104)
);

test!(dtd_entity_04,
"<!DOCTYPE svg [
    <!ENTITY open-hatch
             PUBLIC \"-//Textuality//TEXT Standard open-hatch boilerplate//EN\"
             \"http://www.textuality.com/boilerplate/OpenHatch.xml\">
]>",
    Token::DtdStart("svg", None, 0..15),
    Token::EntityDecl(
        "open-hatch",
        EntityDefinition::ExternalId(
            ExternalId::Public(
                "-//Textuality//TEXT Standard open-hatch boilerplate//EN",
                "http://www.textuality.com/boilerplate/OpenHatch.xml"
            )
        ),
        20..185,
    ),
    Token::DtdEnd(186..188)
);

// TODO: NDATA will be ignored
test!(dtd_entity_05,
"<!DOCTYPE svg [
    <!ENTITY hatch-pic SYSTEM \"../grafix/OpenHatch.gif\" NDATA gif >
]>",
    Token::DtdStart("svg", None, 0..15),
    Token::EntityDecl(
        "hatch-pic",
        EntityDefinition::ExternalId(ExternalId::System("../grafix/OpenHatch.gif")),
        20..83,
    ),
    Token::DtdEnd(84..86)
);

// TODO: unsupported data will be ignored
test!(dtd_entity_06,
"<!DOCTYPE svg [
    <!ELEMENT sgml ANY>
    <!ENTITY ns_extend \"http://ns.adobe.com/Extensibility/1.0/\">
    <!NOTATION example1SVG-rdf SYSTEM \"example1.svg.rdf\">
    <!ATTLIST img data ENTITY #IMPLIED>
]>",
    Token::DtdStart("svg", None, 0..15),
    Token::EntityDecl(
        "ns_extend",
        EntityDefinition::EntityValue("http://ns.adobe.com/Extensibility/1.0/"),
        44..104
    ),
    Token::DtdEnd(203..205)
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

    let mut p = xml::Tokenizer::from(text.as_str());
    assert_eq!(to_test_token(p.next().unwrap()), Token::DtdStart("svg", None, 0..15));
    assert_eq!(to_test_token(p.next().unwrap()), Token::DtdEnd(10016..10018));
}

test!(dtd_err_01, "<!DOCTYPEEG[<!ENTITY%ETT\u{000a}SSSSSSSS<D_IDYT;->\u{000a}<",
    Token::Error("invalid DTD at 1:1 cause expected space not 'E' at 1:10".to_string())
);

test!(dtd_err_02, "<!DOCTYPE s [<!ENTITY % name S YSTEM",
    Token::DtdStart("s", None, 0..13),
    Token::Error("invalid DTD entity at 1:14 cause invalid ExternalID".to_string())
);

test!(dtd_err_03, "<!DOCTYPE s [<!ENTITY % name B",
    Token::DtdStart("s", None, 0..13),
    Token::Error("invalid DTD entity at 1:14 cause \
                  expected '\"', ''', 'S', 'P' not 'B' at 1:30".to_string())
);

test!(dtd_err_04, "<!DOCTYPE s []",
    Token::DtdStart("s", None, 0..13),
    Token::Error("invalid DTD at 1:14 cause unexpected end of stream".to_string())
);

test!(dtd_err_05, "<!DOCTYPE s [] !",
    Token::DtdStart("s", None, 0..13),
    Token::Error("invalid DTD at 1:14 cause expected '>' not '!' at 1:16".to_string())
);
