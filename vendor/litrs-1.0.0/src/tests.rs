use crate::Literal;


#[test]
fn empty() {
    assert_err!(Literal, "", Empty, None);
}

#[test]
fn invalid_literals() {
    assert_err_single!(Literal::parse("."), InvalidLiteral, None);
    assert_err_single!(Literal::parse("+"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("-"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("e"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("e8"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("f32"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("foo"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("inf"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("nan"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("NaN"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("NAN"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("_2.7"), InvalidLiteral, None);
    assert_err_single!(Literal::parse(".5"), InvalidLiteral, None);
}

#[test]
fn misc() {
    assert_err_single!(Literal::parse("0x44.5"), UnexpectedChar, 4..6);
    assert_err_single!(Literal::parse("a"), InvalidLiteral, None);
    assert_err_single!(Literal::parse(";"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("0;"), UnexpectedChar, 1);
    assert_err_single!(Literal::parse(" 0"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("0 "), UnexpectedChar, 1);
    assert_err_single!(Literal::parse("_"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("_3"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("a_123"), InvalidLiteral, None);
    assert_err_single!(Literal::parse("B_123"), InvalidLiteral, None);
}

macro_rules! assert_no_panic {
    ($input:expr) => {
        let arr = $input;
        let input = std::str::from_utf8(&arr).expect("not unicode");
        let res = std::panic::catch_unwind(move || {
            let _ = Literal::parse(input);
            let _ = crate::BoolLit::parse(input);
            let _ = crate::IntegerLit::parse(input);
            let _ = crate::FloatLit::parse(input);
            let _ = crate::CharLit::parse(input);
            let _ = crate::StringLit::parse(input);
            let _ = crate::ByteLit::parse(input);
            let _ = crate::ByteStringLit::parse(input);
        });

        if let Err(e) = res {
            println!("\n!!! panic for: {:?}", input);
            std::panic::resume_unwind(e);
        }
    };
}

#[test]
#[ignore]
fn never_panic_up_to_3() {
    for a in 0..128 {
        assert_no_panic!([a]);
        for b in 0..128 {
            assert_no_panic!([a, b]);
            for c in 0..128 {
                assert_no_panic!([a, b, c]);
            }
        }
    }
}

// This test takes super long in debug mode, but in release mode it's fine.
#[test]
#[ignore]
fn never_panic_len_4() {
    for a in 0..128 {
        for b in 0..128 {
            for c in 0..128 {
                for d in 0..128 {
                    assert_no_panic!([a, b, c, d]);
                }
            }
        }
    }
}

#[cfg(feature = "proc-macro2")]
#[test]
fn proc_macro() {
    use std::convert::TryFrom;

    use proc_macro2::{
        self as pm2, Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree,
    };

    use crate::{
        err::TokenKind, BoolLit, ByteLit, ByteStringLit, CharLit, FloatLit, IntegerLit, StringLit,
    };


    macro_rules! assert_invalid_token {
        ($input:expr, expected: $expected:path, actual: $actual:path $(,)?) => {
            let err = $input.unwrap_err();
            if err.expected != $expected {
                panic!(
                    "err.expected was expected to be {:?}, but is {:?}",
                    $expected, err.expected,
                );
            }
            if err.actual != $actual {
                panic!(
                    "err.actual was expected to be {:?}, but is {:?}",
                    $actual, err.actual
                );
            }
        };
    }


    let pm_u16_lit = pm2::Literal::u16_suffixed(2700);
    let pm_i16_lit = pm2::Literal::i16_unsuffixed(3912);
    let pm_f32_lit = pm2::Literal::f32_unsuffixed(3.14);
    let pm_f64_lit = pm2::Literal::f64_suffixed(99.3);
    let pm_string_lit = pm2::Literal::string("hello 🦊");
    let pm_bytestr_lit = pm2::Literal::byte_string(b"hello \nfoxxo");
    let pm_char_lit = pm2::Literal::character('🦀');

    let u16_lit = Literal::parse("2700u16".to_string()).unwrap();
    let i16_lit = Literal::parse("3912".to_string()).unwrap();
    let f32_lit = Literal::parse("3.14".to_string()).unwrap();
    let f64_lit = Literal::parse("99.3f64".to_string()).unwrap();
    let string_lit = Literal::parse(r#""hello 🦊""#.to_string()).unwrap();
    let bytestr_lit = Literal::parse(r#"b"hello \nfoxxo""#.to_string()).unwrap();
    let char_lit = Literal::parse("'🦀'".to_string()).unwrap();

    assert_eq!(Literal::from(&pm_u16_lit), u16_lit);
    assert_eq!(Literal::from(&pm_i16_lit), i16_lit);
    assert_eq!(Literal::from(&pm_f32_lit), f32_lit);
    assert_eq!(Literal::from(&pm_f64_lit), f64_lit);
    assert_eq!(Literal::from(&pm_string_lit), string_lit);
    assert_eq!(Literal::from(&pm_bytestr_lit), bytestr_lit);
    assert_eq!(Literal::from(&pm_char_lit), char_lit);


    let group = TokenTree::from(Group::new(Delimiter::Brace, TokenStream::new()));
    let punct = TokenTree::from(Punct::new(':', Spacing::Alone));
    let ident = TokenTree::from(Ident::new("peter", Span::call_site()));

    assert_eq!(
        Literal::try_from(TokenTree::Literal(pm2::Literal::string("hello 🦊"))).unwrap(),
        Literal::String(StringLit::parse(r#""hello 🦊""#.to_string()).unwrap()),
    );
    assert_invalid_token!(
        Literal::try_from(punct.clone()),
        expected: TokenKind::Literal,
        actual: TokenKind::Punct,
    );
    assert_invalid_token!(
        Literal::try_from(group.clone()),
        expected: TokenKind::Literal,
        actual: TokenKind::Group,
    );
    assert_invalid_token!(
        Literal::try_from(ident.clone()),
        expected: TokenKind::Literal,
        actual: TokenKind::Ident,
    );


    assert_eq!(Literal::from(IntegerLit::try_from(pm_u16_lit.clone()).unwrap()), u16_lit);
    assert_eq!(Literal::from(IntegerLit::try_from(pm_i16_lit.clone()).unwrap()), i16_lit);
    assert_eq!(Literal::from(FloatLit::try_from(pm_f32_lit.clone()).unwrap()), f32_lit);
    assert_eq!(Literal::from(FloatLit::try_from(pm_f64_lit.clone()).unwrap()), f64_lit);
    assert_eq!(Literal::from(StringLit::try_from(pm_string_lit.clone()).unwrap()), string_lit);
    assert_eq!(
        Literal::from(ByteStringLit::try_from(pm_bytestr_lit.clone()).unwrap()),
        bytestr_lit,
    );
    assert_eq!(Literal::from(CharLit::try_from(pm_char_lit.clone()).unwrap()), char_lit);

    assert_invalid_token!(
        StringLit::try_from(pm_u16_lit.clone()),
        expected: TokenKind::StringLit,
        actual: TokenKind::IntegerLit,
    );
    assert_invalid_token!(
        StringLit::try_from(pm_f32_lit.clone()),
        expected: TokenKind::StringLit,
        actual: TokenKind::FloatLit,
    );
    assert_invalid_token!(
        ByteLit::try_from(pm_bytestr_lit.clone()),
        expected: TokenKind::ByteLit,
        actual: TokenKind::ByteStringLit,
    );
    assert_invalid_token!(
        ByteLit::try_from(pm_i16_lit.clone()),
        expected: TokenKind::ByteLit,
        actual: TokenKind::IntegerLit,
    );
    assert_invalid_token!(
        IntegerLit::try_from(pm_string_lit.clone()),
        expected: TokenKind::IntegerLit,
        actual: TokenKind::StringLit,
    );
    assert_invalid_token!(
        IntegerLit::try_from(pm_char_lit.clone()),
        expected: TokenKind::IntegerLit,
        actual: TokenKind::CharLit,
    );


    assert_eq!(
        Literal::from(IntegerLit::try_from(TokenTree::from(pm_u16_lit.clone())).unwrap()),
        u16_lit,
    );
    assert_eq!(
        Literal::from(IntegerLit::try_from(TokenTree::from(pm_i16_lit.clone())).unwrap()),
        i16_lit,
    );
    assert_eq!(
        Literal::from(FloatLit::try_from(TokenTree::from(pm_f32_lit.clone())).unwrap()),
        f32_lit,
    );
    assert_eq!(
        Literal::from(FloatLit::try_from(TokenTree::from(pm_f64_lit.clone())).unwrap()),
        f64_lit,
    );
    assert_eq!(
        Literal::from(StringLit::try_from(TokenTree::from(pm_string_lit.clone())).unwrap()),
        string_lit,
    );
    assert_eq!(
        Literal::from(ByteStringLit::try_from(TokenTree::from(pm_bytestr_lit.clone())).unwrap()),
        bytestr_lit,
    );
    assert_eq!(
        Literal::from(CharLit::try_from(TokenTree::from(pm_char_lit.clone())).unwrap()),
        char_lit,
    );

    assert_invalid_token!(
        StringLit::try_from(TokenTree::from(pm_u16_lit.clone())),
        expected: TokenKind::StringLit,
        actual: TokenKind::IntegerLit,
    );
    assert_invalid_token!(
        StringLit::try_from(TokenTree::from(pm_f32_lit.clone())),
        expected: TokenKind::StringLit,
        actual: TokenKind::FloatLit,
    );
    assert_invalid_token!(
        BoolLit::try_from(TokenTree::from(pm_bytestr_lit.clone())),
        expected: TokenKind::BoolLit,
        actual: TokenKind::ByteStringLit,
    );
    assert_invalid_token!(
        BoolLit::try_from(TokenTree::from(pm_i16_lit.clone())),
        expected: TokenKind::BoolLit,
        actual: TokenKind::IntegerLit,
    );
    assert_invalid_token!(
        IntegerLit::try_from(TokenTree::from(pm_string_lit.clone())),
        expected: TokenKind::IntegerLit,
        actual: TokenKind::StringLit,
    );
    assert_invalid_token!(
        IntegerLit::try_from(TokenTree::from(pm_char_lit.clone())),
        expected: TokenKind::IntegerLit,
        actual: TokenKind::CharLit,
    );

    assert_invalid_token!(
        StringLit::try_from(TokenTree::from(group)),
        expected: TokenKind::StringLit,
        actual: TokenKind::Group,
    );
    assert_invalid_token!(
        BoolLit::try_from(TokenTree::from(punct)),
        expected: TokenKind::BoolLit,
        actual: TokenKind::Punct,
    );
    assert_invalid_token!(
        FloatLit::try_from(TokenTree::from(ident)),
        expected: TokenKind::FloatLit,
        actual: TokenKind::Ident,
    );
}

#[cfg(feature = "proc-macro2")]
#[test]
fn bool_try_from_tt() {
    use std::convert::TryFrom;

    use proc_macro2::{Ident, Span, TokenTree};

    use crate::BoolLit;


    let ident = |s: &str| Ident::new(s, Span::call_site());

    assert_eq!(BoolLit::try_from(TokenTree::Ident(ident("true"))).unwrap(), BoolLit::True);
    assert_eq!(BoolLit::try_from(TokenTree::Ident(ident("false"))).unwrap(), BoolLit::False);

    assert!(BoolLit::try_from(TokenTree::Ident(ident("falsex"))).is_err());
    assert!(BoolLit::try_from(TokenTree::Ident(ident("_false"))).is_err());
    assert!(BoolLit::try_from(TokenTree::Ident(ident("False"))).is_err());
    assert!(BoolLit::try_from(TokenTree::Ident(ident("True"))).is_err());
    assert!(BoolLit::try_from(TokenTree::Ident(ident("ltrue"))).is_err());


    assert_eq!(
        Literal::try_from(TokenTree::Ident(ident("true"))).unwrap(),
        Literal::Bool(BoolLit::True),
    );
    assert_eq!(
        Literal::try_from(TokenTree::Ident(ident("false"))).unwrap(),
        Literal::Bool(BoolLit::False),
    );

    assert!(Literal::try_from(TokenTree::Ident(ident("falsex"))).is_err());
    assert!(Literal::try_from(TokenTree::Ident(ident("_false"))).is_err());
    assert!(Literal::try_from(TokenTree::Ident(ident("False"))).is_err());
    assert!(Literal::try_from(TokenTree::Ident(ident("True"))).is_err());
    assert!(Literal::try_from(TokenTree::Ident(ident("ltrue"))).is_err());
}

#[cfg(feature = "proc-macro2")]
#[test]
fn invalid_token_display() {
    use crate::{err::TokenKind, InvalidToken};

    let span = crate::err::Span::Two(proc_macro2::Span::call_site());
    assert_eq!(
        InvalidToken {
            actual: TokenKind::StringLit,
            expected: TokenKind::FloatLit,
            span,
        }.to_string(),
        r#"expected a float literal (e.g. `3.14`), but found a string literal (e.g. "Ferris")"#,
    );

    assert_eq!(
        InvalidToken {
            actual: TokenKind::Punct,
            expected: TokenKind::Literal,
            span,
        }.to_string(),
        r#"expected a literal, but found a punctuation character"#,
    );
}
