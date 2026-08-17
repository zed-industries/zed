/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(feature = "bench")]
extern crate test;

use serde_json::{json, Map, Value};

#[cfg(feature = "bench")]
use self::test::Bencher;

use super::{
    parse_important, parse_nth, parse_one_declaration, parse_one_rule, stylesheet_encoding,
    AtRuleParser, BasicParseError, BasicParseErrorKind, CowRcStr, DeclarationParser, Delimiter,
    EncodingSupport, ParseError, ParseErrorKind, Parser, ParserInput, ParserState,
    QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, SourceLocation, StyleSheetParser,
    ToCss, Token, TokenSerializationType, UnicodeRange,
};

macro_rules! JArray {
    ($($e: expr,)*) => { JArray![ $( $e ),* ] };
    ($($e: expr),*) => { Value::Array(vec!( $( $e.to_json() ),* )) }
}

fn almost_equals(a: &Value, b: &Value) -> bool {
    let var_name = match (a, b) {
        (Value::Number(a), Value::Number(b)) => {
            let a = a.as_f64().unwrap();
            let b = b.as_f64().unwrap();
            (a - b).abs() <= a.abs() * 1e-6
        }

        (&Value::Bool(a), &Value::Bool(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| almost_equals(a, b))
        }
        (&Value::Object(_), &Value::Object(_)) => panic!("Not implemented"),
        (&Value::Null, &Value::Null) => true,
        _ => false,
    };
    var_name
}

fn normalize(json: &mut Value) {
    match *json {
        Value::Array(ref mut list) => {
            for item in list.iter_mut() {
                normalize(item)
            }
        }
        Value::String(ref mut s) => {
            if *s == "extra-input" || *s == "empty" {
                *s = "invalid".to_string()
            }
        }
        _ => {}
    }
}

fn assert_json_eq(results: Value, mut expected: Value, message: &str) {
    normalize(&mut expected);
    if !almost_equals(&results, &expected) {
        println!(
            "{}",
            ::difference::Changeset::new(
                &serde_json::to_string_pretty(&results).unwrap(),
                &serde_json::to_string_pretty(&expected).unwrap(),
                "\n",
            )
        );
        panic!("{}", message)
    }
}

fn run_raw_json_tests<F: Fn(Value, Value)>(json_data: &str, run: F) {
    let items = match serde_json::from_str(json_data) {
        Ok(Value::Array(items)) => items,
        other => panic!("Invalid JSON: {:?}", other),
    };
    assert!(items.len() % 2 == 0);
    let mut input = None;
    for item in items.into_iter() {
        match (&input, item) {
            (&None, json_obj) => input = Some(json_obj),
            (&Some(_), expected) => {
                let input = input.take().unwrap();
                run(input, expected)
            }
        };
    }
}

fn run_json_tests<F: Fn(&mut Parser) -> Value>(json_data: &str, parse: F) {
    run_raw_json_tests(json_data, |input, expected| match input {
        Value::String(input) => {
            let mut parse_input = ParserInput::new(&input);
            let result = parse(&mut Parser::new(&mut parse_input));
            assert_json_eq(result, expected, &input);
        }
        _ => panic!("Unexpected JSON"),
    });
}

#[test]
fn component_value_list() {
    run_json_tests(
        include_str!("css-parsing-tests/component_value_list.json"),
        |input| Value::Array(component_values_to_json(input)),
    );
}

#[test]
fn one_component_value() {
    run_json_tests(
        include_str!("css-parsing-tests/one_component_value.json"),
        |input| {
            let result: Result<Value, ParseError<()>> = input.parse_entirely(|input| {
                Ok(one_component_value_to_json(input.next()?.clone(), input))
            });
            result.unwrap_or(JArray!["error", "invalid"])
        },
    );
}

#[test]
fn declaration_list() {
    run_json_tests(
        include_str!("css-parsing-tests/declaration_list.json"),
        |input| {
            Value::Array(
                RuleBodyParser::new(input, &mut JsonParser)
                    .map(|result| result.unwrap_or(JArray!["error", "invalid"]))
                    .collect(),
            )
        },
    );
}

#[test]
fn one_declaration() {
    run_json_tests(
        include_str!("css-parsing-tests/one_declaration.json"),
        |input| {
            parse_one_declaration(input, &mut JsonParser).unwrap_or(JArray!["error", "invalid"])
        },
    );
}

#[test]
fn rule_list() {
    run_json_tests(include_str!("css-parsing-tests/rule_list.json"), |input| {
        Value::Array(
            RuleBodyParser::new(input, &mut JsonParser)
                .map(|result| result.unwrap_or(JArray!["error", "invalid"]))
                .collect(),
        )
    });
}

#[test]
fn stylesheet() {
    run_json_tests(include_str!("css-parsing-tests/stylesheet.json"), |input| {
        Value::Array(
            StyleSheetParser::new(input, &mut JsonParser)
                .map(|result| result.unwrap_or(JArray!["error", "invalid"]))
                .collect(),
        )
    });
}

#[test]
fn one_rule() {
    run_json_tests(include_str!("css-parsing-tests/one_rule.json"), |input| {
        parse_one_rule(input, &mut JsonParser).unwrap_or(JArray!["error", "invalid"])
    });
}

#[test]
fn stylesheet_from_bytes() {
    pub struct EncodingRs;

    impl EncodingSupport for EncodingRs {
        type Encoding = &'static encoding_rs::Encoding;

        fn utf8() -> Self::Encoding {
            encoding_rs::UTF_8
        }

        fn is_utf16_be_or_le(encoding: &Self::Encoding) -> bool {
            *encoding == encoding_rs::UTF_16LE || *encoding == encoding_rs::UTF_16BE
        }

        fn from_label(ascii_label: &[u8]) -> Option<Self::Encoding> {
            encoding_rs::Encoding::for_label(ascii_label)
        }
    }

    run_raw_json_tests(
        include_str!("css-parsing-tests/stylesheet_bytes.json"),
        |input, expected| {
            let map = match input {
                Value::Object(map) => map,
                _ => panic!("Unexpected JSON"),
            };

            let result = {
                let css = get_string(&map, "css_bytes")
                    .unwrap()
                    .chars()
                    .map(|c| {
                        assert!(c as u32 <= 0xFF);
                        c as u8
                    })
                    .collect::<Vec<u8>>();
                let protocol_encoding_label =
                    get_string(&map, "protocol_encoding").map(|s| s.as_bytes());
                let environment_encoding = get_string(&map, "environment_encoding")
                    .map(|s| s.as_bytes())
                    .and_then(EncodingRs::from_label);

                let encoding = stylesheet_encoding::<EncodingRs>(
                    &css,
                    protocol_encoding_label,
                    environment_encoding,
                );
                let (css_unicode, used_encoding, _) = encoding.decode(&css);
                let mut input = ParserInput::new(&css_unicode);
                let input = &mut Parser::new(&mut input);
                let rules = StyleSheetParser::new(input, &mut JsonParser)
                    .map(|result| result.unwrap_or(JArray!["error", "invalid"]))
                    .collect::<Vec<_>>();
                JArray![rules, used_encoding.name().to_lowercase()]
            };
            assert_json_eq(result, expected, &Value::Object(map).to_string());
        },
    );

    fn get_string<'a>(map: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
        match map.get(key) {
            Some(Value::String(s)) => Some(s),
            Some(&Value::Null) => None,
            None => None,
            _ => panic!("Unexpected JSON"),
        }
    }
}

#[test]
fn expect_no_error_token() {
    let mut input = ParserInput::new("foo 4px ( / { !bar }");
    assert!(Parser::new(&mut input).expect_no_error_token().is_ok());
    let mut input = ParserInput::new(")");
    assert!(Parser::new(&mut input).expect_no_error_token().is_err());
    let mut input = ParserInput::new("}");
    assert!(Parser::new(&mut input).expect_no_error_token().is_err());
    let mut input = ParserInput::new("(a){]");
    assert!(Parser::new(&mut input).expect_no_error_token().is_err());
    let mut input = ParserInput::new("'\n'");
    assert!(Parser::new(&mut input).expect_no_error_token().is_err());
    let mut input = ParserInput::new("url('\n'");
    assert!(Parser::new(&mut input).expect_no_error_token().is_err());
    let mut input = ParserInput::new("url(a b)");
    assert!(Parser::new(&mut input).expect_no_error_token().is_err());
    let mut input = ParserInput::new("url(\u{7F}))");
    assert!(Parser::new(&mut input).expect_no_error_token().is_err());
}

/// https://github.com/servo/rust-cssparser/issues/71
#[test]
fn outer_block_end_consumed() {
    let mut input = ParserInput::new("(calc(true))");
    let mut input = Parser::new(&mut input);
    assert!(input.expect_parenthesis_block().is_ok());
    assert!(input
        .parse_nested_block(|input| input
            .expect_function_matching("calc")
            .map_err(Into::<ParseError<()>>::into))
        .is_ok());
    println!("{:?}", input.position());
    assert!(input.next().is_err());
}

/// https://github.com/servo/rust-cssparser/issues/174
#[test]
fn bad_url_slice_out_of_bounds() {
    let mut input = ParserInput::new("url(\u{1}\\");
    let mut parser = Parser::new(&mut input);
    let result = parser.next_including_whitespace_and_comments(); // This used to panic
    assert_eq!(result, Ok(&Token::BadUrl("\u{1}\\".into())));
}

/// https://bugzilla.mozilla.org/show_bug.cgi?id=1383975
#[test]
fn bad_url_slice_not_at_char_boundary() {
    let mut input = ParserInput::new("url(9\n۰");
    let mut parser = Parser::new(&mut input);
    let result = parser.next_including_whitespace_and_comments(); // This used to panic
    assert_eq!(result, Ok(&Token::BadUrl("9\n۰".into())));
}

#[test]
fn unquoted_url_escaping() {
    let token = Token::UnquotedUrl(
        "\
         \x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\
         \x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f \
         !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]\
         ^_`abcdefghijklmnopqrstuvwxyz{|}~\x7fé\
         "
        .into(),
    );
    let serialized = token.to_css_string();
    assert_eq!(
        serialized,
        "\
         url(\
         \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 \\a \\b \\c \\d \\e \\f \\10 \
         \\11 \\12 \\13 \\14 \\15 \\16 \\17 \\18 \\19 \\1a \\1b \\1c \\1d \\1e \\1f \\20 \
         !\\\"#$%&\\'\\(\\)*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\\\]\
         ^_`abcdefghijklmnopqrstuvwxyz{|}~\\7f é\
         )\
         "
    );
    let mut input = ParserInput::new(&serialized);
    assert_eq!(Parser::new(&mut input).next(), Ok(&token));
}

#[test]
fn test_expect_url() {
    fn parse<'a>(s: &mut ParserInput<'a>) -> Result<CowRcStr<'a>, BasicParseError<'a>> {
        Parser::new(s).expect_url()
    }
    let mut input = ParserInput::new("url()");
    assert_eq!(parse(&mut input).unwrap(), "");
    let mut input = ParserInput::new("url( ");
    assert_eq!(parse(&mut input).unwrap(), "");
    let mut input = ParserInput::new("url( abc");
    assert_eq!(parse(&mut input).unwrap(), "abc");
    let mut input = ParserInput::new("url( abc \t)");
    assert_eq!(parse(&mut input).unwrap(), "abc");
    let mut input = ParserInput::new("url( 'abc' \t)");
    assert_eq!(parse(&mut input).unwrap(), "abc");
    let mut input = ParserInput::new("url(abc more stuff)");
    assert!(parse(&mut input).is_err());
    // The grammar at https://drafts.csswg.org/css-values/#urls plans for `<url-modifier>*`
    // at the position of "more stuff", but no such modifier is defined yet.
    let mut input = ParserInput::new("url('abc' more stuff)");
    assert!(parse(&mut input).is_err());
}

#[test]
fn nth() {
    run_json_tests(include_str!("css-parsing-tests/An+B.json"), |input| {
        input
            .parse_entirely(|i| {
                let result: Result<_, ParseError<()>> = parse_nth(i).map_err(Into::into);
                result
            })
            .ok()
            .map(|(v0, v1)| json!([v0, v1]))
            .unwrap_or(Value::Null)
    });
}

#[test]
fn parse_comma_separated_ignoring_errors() {
    let input = "red, green something, yellow, whatever, blue";
    let mut input = ParserInput::new(input);
    let mut input = Parser::new(&mut input);
    let result = input.parse_comma_separated_ignoring_errors(|input| {
        let loc = input.current_source_location();
        let ident = input.expect_ident()?;
        crate::color::parse_named_color(ident).map_err(|()| {
            loc.new_unexpected_token_error::<ParseError<()>>(Token::Ident(ident.clone()))
        })
    });
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], (255, 0, 0));
    assert_eq!(result[1], (255, 255, 0));
    assert_eq!(result[2], (0, 0, 255));
}

#[test]
fn unicode_range() {
    run_json_tests(include_str!("css-parsing-tests/urange.json"), |input| {
        let result: Result<_, ParseError<()>> = input.parse_comma_separated(|input| {
            let result = UnicodeRange::parse(input).ok().map(|r| (r.start, r.end));
            if input.is_exhausted() {
                Ok(result)
            } else {
                while input.next().is_ok() {}
                Ok(None)
            }
        });
        result
            .unwrap()
            .iter()
            .map(|v| {
                if let Some((v0, v1)) = v {
                    json!([v0, v1])
                } else {
                    Value::Null
                }
            })
            .collect::<Vec<_>>()
            .to_json()
    });
}

#[test]
fn serializer_not_preserving_comments() {
    serializer(false)
}

#[test]
fn serializer_preserving_comments() {
    serializer(true)
}

fn serializer(preserve_comments: bool) {
    run_json_tests(
        include_str!("css-parsing-tests/component_value_list.json"),
        |input| {
            fn write_to(
                mut previous_token: TokenSerializationType,
                input: &mut Parser,
                string: &mut String,
                preserve_comments: bool,
            ) {
                while let Ok(token) = if preserve_comments {
                    input.next_including_whitespace_and_comments().cloned()
                } else {
                    input.next_including_whitespace().cloned()
                } {
                    let token_type = token.serialization_type();
                    if !preserve_comments && previous_token.needs_separator_when_before(token_type)
                    {
                        string.push_str("/**/")
                    }
                    previous_token = token_type;
                    token.to_css(string).unwrap();
                    let closing_token = match token {
                        Token::Function(_) | Token::ParenthesisBlock => {
                            Some(Token::CloseParenthesis)
                        }
                        Token::SquareBracketBlock => Some(Token::CloseSquareBracket),
                        Token::CurlyBracketBlock => Some(Token::CloseCurlyBracket),
                        _ => None,
                    };
                    if let Some(closing_token) = closing_token {
                        let result: Result<_, ParseError<()>> = input.parse_nested_block(|input| {
                            write_to(previous_token, input, string, preserve_comments);
                            Ok(())
                        });
                        result.unwrap();
                        closing_token.to_css(string).unwrap();
                    }
                }
            }
            let mut serialized = String::new();
            write_to(
                TokenSerializationType::Nothing,
                input,
                &mut serialized,
                preserve_comments,
            );
            let mut input = ParserInput::new(&serialized);
            let parser = &mut Parser::new(&mut input);
            Value::Array(component_values_to_json(parser))
        },
    );
}

#[test]
fn serialize_bad_tokens() {
    let mut input = ParserInput::new("url(foo\\) b\\)ar)'ba\\'\"z\n4");
    let mut parser = Parser::new(&mut input);

    let token = parser.next().unwrap().clone();
    assert!(matches!(token, Token::BadUrl(_)));
    assert_eq!(token.to_css_string(), "url(foo\\) b\\)ar)");

    let token = parser.next().unwrap().clone();
    assert!(matches!(token, Token::BadString(_)));
    assert_eq!(token.to_css_string(), "\"ba'\\\"z");

    let token = parser.next().unwrap().clone();
    assert!(matches!(token, Token::Number { .. }));
    assert_eq!(token.to_css_string(), "4");

    assert!(parser.next().is_err());
}

#[test]
fn line_numbers() {
    let mut input = ParserInput::new(concat!(
        "fo\\30\r\n",
        "0o bar/*\n",
        "*/baz\r\n",
        "\n",
        "url(\r\n",
        "  u \r\n",
        ")\"a\\\r\n",
        "b\""
    ));
    let mut input = Parser::new(&mut input);
    assert_eq!(
        input.current_source_location(),
        SourceLocation { line: 0, column: 1 }
    );
    assert_eq!(
        input.next_including_whitespace(),
        Ok(&Token::Ident("fo00o".into()))
    );
    assert_eq!(
        input.current_source_location(),
        SourceLocation { line: 1, column: 3 }
    );
    assert_eq!(
        input.next_including_whitespace(),
        Ok(&Token::WhiteSpace(" "))
    );
    assert_eq!(
        input.current_source_location(),
        SourceLocation { line: 1, column: 4 }
    );
    assert_eq!(
        input.next_including_whitespace(),
        Ok(&Token::Ident("bar".into()))
    );
    assert_eq!(
        input.current_source_location(),
        SourceLocation { line: 1, column: 7 }
    );
    assert_eq!(
        input.next_including_whitespace_and_comments(),
        Ok(&Token::Comment("\n"))
    );
    assert_eq!(
        input.current_source_location(),
        SourceLocation { line: 2, column: 3 }
    );
    assert_eq!(
        input.next_including_whitespace(),
        Ok(&Token::Ident("baz".into()))
    );
    assert_eq!(
        input.current_source_location(),
        SourceLocation { line: 2, column: 6 }
    );
    let state = input.state();

    assert_eq!(
        input.next_including_whitespace(),
        Ok(&Token::WhiteSpace("\r\n\n"))
    );
    assert_eq!(
        input.current_source_location(),
        SourceLocation { line: 4, column: 1 }
    );

    assert_eq!(
        state.source_location(),
        SourceLocation { line: 2, column: 6 }
    );

    assert_eq!(
        input.next_including_whitespace(),
        Ok(&Token::UnquotedUrl("u".into()))
    );
    assert_eq!(
        input.current_source_location(),
        SourceLocation { line: 6, column: 2 }
    );

    assert_eq!(
        input.next_including_whitespace(),
        Ok(&Token::QuotedString("ab".into()))
    );
    assert_eq!(
        input.current_source_location(),
        SourceLocation { line: 7, column: 3 }
    );
    assert!(input.next_including_whitespace().is_err());
}

#[test]
fn overflow() {
    let css = r"
         2147483646
         2147483647
         2147483648
         10000000000000
         1000000000000000000000000000000000000000
         1{309 zeros}

         -2147483647
         -2147483648
         -2147483649
         -10000000000000
         -1000000000000000000000000000000000000000
         -1{309 zeros}

         3.30282347e+38
         3.40282347e+38
         3.402824e+38

         -3.30282347e+38
         -3.40282347e+38
         -3.402824e+38

    "
    .replace("{309 zeros}", &"0".repeat(309));
    let mut input = ParserInput::new(&css);
    let mut input = Parser::new(&mut input);

    assert_eq!(input.expect_integer(), Ok(2147483646));
    assert_eq!(input.expect_integer(), Ok(2147483647));
    assert_eq!(input.expect_integer(), Ok(2147483647)); // Clamp on overflow
    assert_eq!(input.expect_integer(), Ok(2147483647));
    assert_eq!(input.expect_integer(), Ok(2147483647));
    assert_eq!(input.expect_integer(), Ok(2147483647));

    assert_eq!(input.expect_integer(), Ok(-2147483647));
    assert_eq!(input.expect_integer(), Ok(-2147483648));
    assert_eq!(input.expect_integer(), Ok(-2147483648)); // Clamp on overflow
    assert_eq!(input.expect_integer(), Ok(-2147483648));
    assert_eq!(input.expect_integer(), Ok(-2147483648));
    assert_eq!(input.expect_integer(), Ok(-2147483648));

    assert_eq!(input.expect_number(), Ok(3.302_823_5e38));
    assert_eq!(input.expect_number(), Ok(f32::MAX));
    assert_eq!(input.expect_number(), Ok(f32::INFINITY));

    assert_eq!(input.expect_number(), Ok(-3.302_823_5e38));
    assert_eq!(input.expect_number(), Ok(f32::MIN));
    assert_eq!(input.expect_number(), Ok(f32::NEG_INFINITY));
}

#[test]
fn line_delimited() {
    let mut input = ParserInput::new(" { foo ; bar } baz;,");
    let mut input = Parser::new(&mut input);
    assert_eq!(input.next(), Ok(&Token::CurlyBracketBlock));
    assert!({
        let result: Result<_, ParseError<()>> =
            input.parse_until_after(Delimiter::Semicolon, |_| Ok(42));
        result
    }
    .is_err());
    assert_eq!(input.next(), Ok(&Token::Comma));
    assert!(input.next().is_err());
}

#[test]
fn identifier_serialization() {
    // Null bytes
    assert_eq!(Token::Ident("\0".into()).to_css_string(), "\u{FFFD}");
    assert_eq!(Token::Ident("a\0".into()).to_css_string(), "a\u{FFFD}");
    assert_eq!(Token::Ident("\0b".into()).to_css_string(), "\u{FFFD}b");
    assert_eq!(Token::Ident("a\0b".into()).to_css_string(), "a\u{FFFD}b");

    // Replacement character
    assert_eq!(Token::Ident("\u{FFFD}".into()).to_css_string(), "\u{FFFD}");
    assert_eq!(
        Token::Ident("a\u{FFFD}".into()).to_css_string(),
        "a\u{FFFD}"
    );
    assert_eq!(
        Token::Ident("\u{FFFD}b".into()).to_css_string(),
        "\u{FFFD}b"
    );
    assert_eq!(
        Token::Ident("a\u{FFFD}b".into()).to_css_string(),
        "a\u{FFFD}b"
    );

    // Number prefix
    assert_eq!(Token::Ident("0a".into()).to_css_string(), "\\30 a");
    assert_eq!(Token::Ident("1a".into()).to_css_string(), "\\31 a");
    assert_eq!(Token::Ident("2a".into()).to_css_string(), "\\32 a");
    assert_eq!(Token::Ident("3a".into()).to_css_string(), "\\33 a");
    assert_eq!(Token::Ident("4a".into()).to_css_string(), "\\34 a");
    assert_eq!(Token::Ident("5a".into()).to_css_string(), "\\35 a");
    assert_eq!(Token::Ident("6a".into()).to_css_string(), "\\36 a");
    assert_eq!(Token::Ident("7a".into()).to_css_string(), "\\37 a");
    assert_eq!(Token::Ident("8a".into()).to_css_string(), "\\38 a");
    assert_eq!(Token::Ident("9a".into()).to_css_string(), "\\39 a");

    // Letter number prefix
    assert_eq!(Token::Ident("a0b".into()).to_css_string(), "a0b");
    assert_eq!(Token::Ident("a1b".into()).to_css_string(), "a1b");
    assert_eq!(Token::Ident("a2b".into()).to_css_string(), "a2b");
    assert_eq!(Token::Ident("a3b".into()).to_css_string(), "a3b");
    assert_eq!(Token::Ident("a4b".into()).to_css_string(), "a4b");
    assert_eq!(Token::Ident("a5b".into()).to_css_string(), "a5b");
    assert_eq!(Token::Ident("a6b".into()).to_css_string(), "a6b");
    assert_eq!(Token::Ident("a7b".into()).to_css_string(), "a7b");
    assert_eq!(Token::Ident("a8b".into()).to_css_string(), "a8b");
    assert_eq!(Token::Ident("a9b".into()).to_css_string(), "a9b");

    // Dash number prefix
    assert_eq!(Token::Ident("-0a".into()).to_css_string(), "-\\30 a");
    assert_eq!(Token::Ident("-1a".into()).to_css_string(), "-\\31 a");
    assert_eq!(Token::Ident("-2a".into()).to_css_string(), "-\\32 a");
    assert_eq!(Token::Ident("-3a".into()).to_css_string(), "-\\33 a");
    assert_eq!(Token::Ident("-4a".into()).to_css_string(), "-\\34 a");
    assert_eq!(Token::Ident("-5a".into()).to_css_string(), "-\\35 a");
    assert_eq!(Token::Ident("-6a".into()).to_css_string(), "-\\36 a");
    assert_eq!(Token::Ident("-7a".into()).to_css_string(), "-\\37 a");
    assert_eq!(Token::Ident("-8a".into()).to_css_string(), "-\\38 a");
    assert_eq!(Token::Ident("-9a".into()).to_css_string(), "-\\39 a");

    // Double dash prefix
    assert_eq!(Token::Ident("--a".into()).to_css_string(), "--a");

    // Various tests
    assert_eq!(
        Token::Ident("\x01\x02\x1E\x1F".into()).to_css_string(),
        "\\1 \\2 \\1e \\1f "
    );
    assert_eq!(
        Token::Ident("\u{0080}\x2D\x5F\u{00A9}".into()).to_css_string(),
        "\u{0080}\x2D\x5F\u{00A9}"
    );
    assert_eq!(Token::Ident("\x7F\u{0080}\u{0081}\u{0082}\u{0083}\u{0084}\u{0085}\u{0086}\u{0087}\u{0088}\u{0089}\
        \u{008A}\u{008B}\u{008C}\u{008D}\u{008E}\u{008F}\u{0090}\u{0091}\u{0092}\u{0093}\u{0094}\u{0095}\u{0096}\
        \u{0097}\u{0098}\u{0099}\u{009A}\u{009B}\u{009C}\u{009D}\u{009E}\u{009F}".into()).to_css_string(),
        "\\7f \u{0080}\u{0081}\u{0082}\u{0083}\u{0084}\u{0085}\u{0086}\u{0087}\u{0088}\u{0089}\u{008A}\u{008B}\u{008C}\
        \u{008D}\u{008E}\u{008F}\u{0090}\u{0091}\u{0092}\u{0093}\u{0094}\u{0095}\u{0096}\u{0097}\u{0098}\u{0099}\
        \u{009A}\u{009B}\u{009C}\u{009D}\u{009E}\u{009F}");
    assert_eq!(
        Token::Ident("\u{00A0}\u{00A1}\u{00A2}".into()).to_css_string(),
        "\u{00A0}\u{00A1}\u{00A2}"
    );
    assert_eq!(
        Token::Ident("a0123456789b".into()).to_css_string(),
        "a0123456789b"
    );
    assert_eq!(
        Token::Ident("abcdefghijklmnopqrstuvwxyz".into()).to_css_string(),
        "abcdefghijklmnopqrstuvwxyz"
    );
    assert_eq!(
        Token::Ident("ABCDEFGHIJKLMNOPQRSTUVWXYZ".into()).to_css_string(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
    assert_eq!(
        Token::Ident("\x20\x21\x78\x79".into()).to_css_string(),
        "\\ \\!xy"
    );

    // astral symbol (U+1D306 TETRAGRAM FOR CENTRE)
    assert_eq!(
        Token::Ident("\u{1D306}".into()).to_css_string(),
        "\u{1D306}"
    );
}

trait ToJson {
    fn to_json(&self) -> Value;
}

impl<T> ToJson for T
where
    T: Clone,
    Value: From<T>,
{
    fn to_json(&self) -> Value {
        Value::from(self.clone())
    }
}

impl ToJson for CowRcStr<'_> {
    fn to_json(&self) -> Value {
        let s: &str = self;
        s.to_json()
    }
}

#[bench]
#[cfg(feature = "bench")]
fn delimiter_from_byte(b: &mut Bencher) {
    use crate::Delimiters;
    b.iter(|| {
        for _ in 0..1000 {
            for i in 0..256 {
                std::hint::black_box(Delimiters::from_byte(Some(i as u8)));
            }
        }
    })
}

#[cfg(feature = "bench")]
const BACKGROUND_IMAGE: &'static str = include_str!("big-data-url.css");

#[cfg(feature = "bench")]
#[bench]
fn unquoted_url(b: &mut Bencher) {
    b.iter(|| {
        let mut input = ParserInput::new(BACKGROUND_IMAGE);
        let mut input = Parser::new(&mut input);
        input.look_for_var_or_env_functions();

        let result = input.try_parse(|input| input.expect_url());

        assert!(result.is_ok());

        input.seen_var_or_env_functions();
        (result.is_ok(), input.seen_var_or_env_functions())
    })
}

#[cfg_attr(all(miri, feature = "skip_long_tests"), ignore)]
#[cfg(feature = "bench")]
#[bench]
fn numeric(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..1000000 {
            let mut input = ParserInput::new("10px");
            let mut input = Parser::new(&mut input);
            let _ = test::black_box(input.next());
        }
    })
}

struct JsonParser;

#[cfg_attr(all(miri, feature = "skip_long_tests"), ignore)]
#[test]
fn no_stack_overflow_multiple_nested_blocks() {
    let mut input: String = "{{".into();
    for _ in 0..20 {
        let dup = input.clone();
        input.push_str(&dup);
    }
    let mut input = ParserInput::new(&input);
    let mut input = Parser::new(&mut input);
    while input.next().is_ok() {}
}

impl<'i> DeclarationParser<'i> for JsonParser {
    type Declaration = Value;
    type Error = ();

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        _declaration_start: &ParserState,
    ) -> Result<Value, ParseError<'i, ()>> {
        let mut value = vec![];
        let mut important = false;
        loop {
            let start = input.state();
            if let Ok(mut token) = input.next_including_whitespace().cloned() {
                // Hack to deal with css-parsing-tests assuming that
                // `!important` in the middle of a declaration value is OK.
                // This can never happen per spec
                // (even CSS Variables forbid top-level `!`)
                if token == Token::Delim('!') {
                    input.reset(&start);
                    if parse_important(input).is_ok() && input.is_exhausted() {
                        important = true;
                        break;
                    }
                    input.reset(&start);
                    token = input.next_including_whitespace().unwrap().clone();
                }
                value.push(one_component_value_to_json(token, input));
            } else {
                break;
            }
        }
        Ok(JArray!["declaration", name, value, important,])
    }
}

impl<'i> AtRuleParser<'i> for JsonParser {
    type Prelude = Vec<Value>;
    type AtRule = Value;
    type Error = ();

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Vec<Value>, ParseError<'i, ()>> {
        let prelude = vec![
            "at-rule".to_json(),
            name.to_json(),
            Value::Array(component_values_to_json(input)),
        ];
        match_ignore_ascii_case! { &*name,
            "charset" => {
                Err(input.new_error(BasicParseErrorKind::AtRuleInvalid(name.clone())))
            },
            _ => Ok(prelude),
        }
    }

    fn rule_without_block(
        &mut self,
        mut prelude: Vec<Value>,
        _: &ParserState,
    ) -> Result<Value, ()> {
        prelude.push(Value::Null);
        Ok(Value::Array(prelude))
    }

    fn parse_block<'t>(
        &mut self,
        mut prelude: Vec<Value>,
        _: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Value, ParseError<'i, ()>> {
        prelude.push(Value::Array(component_values_to_json(input)));
        Ok(Value::Array(prelude))
    }
}

impl<'i> QualifiedRuleParser<'i> for JsonParser {
    type Prelude = Vec<Value>;
    type QualifiedRule = Value;
    type Error = ();

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Vec<Value>, ParseError<'i, ()>> {
        Ok(component_values_to_json(input))
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Vec<Value>,
        _: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Value, ParseError<'i, ()>> {
        Ok(JArray![
            "qualified rule",
            prelude,
            component_values_to_json(input),
        ])
    }
}

impl RuleBodyItemParser<'_, Value, ()> for JsonParser {
    fn parse_qualified(&self) -> bool {
        true
    }
    fn parse_declarations(&self) -> bool {
        true
    }
}

fn component_values_to_json(input: &mut Parser) -> Vec<Value> {
    let mut values = vec![];
    while let Ok(token) = input.next_including_whitespace().cloned() {
        values.push(one_component_value_to_json(token, input));
    }
    values
}

fn one_component_value_to_json(token: Token, input: &mut Parser) -> Value {
    fn numeric(value: f32, int_value: Option<i32>, has_sign: bool) -> Vec<Value> {
        vec![
            Token::Number {
                value,
                int_value,
                has_sign,
            }
            .to_css_string()
            .to_json(),
            match int_value {
                Some(i) => i.to_json(),
                None => value.to_json(),
            },
            match int_value {
                Some(_) => "integer",
                None => "number",
            }
            .to_json(),
        ]
    }

    fn nested(input: &mut Parser) -> Vec<Value> {
        let result: Result<_, ParseError<()>> =
            input.parse_nested_block(|input| Ok(component_values_to_json(input)));
        result.unwrap()
    }

    match token {
        Token::Ident(value) => JArray!["ident", value],
        Token::AtKeyword(value) => JArray!["at-keyword", value],
        Token::Hash(value) => JArray!["hash", value, "unrestricted"],
        Token::IDHash(value) => JArray!["hash", value, "id"],
        Token::QuotedString(value) => JArray!["string", value],
        Token::UnquotedUrl(value) => JArray!["url", value],
        Token::Delim('\\') => "\\".to_json(),
        Token::Delim(value) => value.to_string().to_json(),

        Token::Number {
            value,
            int_value,
            has_sign,
        } => Value::Array({
            let mut v = vec!["number".to_json()];
            v.extend(numeric(value, int_value, has_sign));
            v
        }),
        Token::Percentage {
            unit_value,
            int_value,
            has_sign,
        } => Value::Array({
            let mut v = vec!["percentage".to_json()];
            v.extend(numeric(unit_value * 100., int_value, has_sign));
            v
        }),
        Token::Dimension {
            value,
            int_value,
            has_sign,
            unit,
        } => Value::Array({
            let mut v = vec!["dimension".to_json()];
            v.extend(numeric(value, int_value, has_sign));
            v.push(unit.to_json());
            v
        }),

        Token::WhiteSpace(_) => " ".to_json(),
        Token::Comment(_) => "/**/".to_json(),
        Token::Colon => ":".to_json(),
        Token::Semicolon => ";".to_json(),
        Token::Comma => ",".to_json(),
        Token::IncludeMatch => "~=".to_json(),
        Token::DashMatch => "|=".to_json(),
        Token::PrefixMatch => "^=".to_json(),
        Token::SuffixMatch => "$=".to_json(),
        Token::SubstringMatch => "*=".to_json(),
        Token::CDO => "<!--".to_json(),
        Token::CDC => "-->".to_json(),

        Token::Function(name) => Value::Array({
            let mut v = vec!["function".to_json(), name.to_json()];
            v.extend(nested(input));
            v
        }),
        Token::ParenthesisBlock => Value::Array({
            let mut v = vec!["()".to_json()];
            v.extend(nested(input));
            v
        }),
        Token::SquareBracketBlock => Value::Array({
            let mut v = vec!["[]".to_json()];
            v.extend(nested(input));
            v
        }),
        Token::CurlyBracketBlock => Value::Array({
            let mut v = vec!["{}".to_json()];
            v.extend(nested(input));
            v
        }),
        Token::BadUrl(_) => JArray!["error", "bad-url"],
        Token::BadString(_) => JArray!["error", "bad-string"],
        Token::CloseParenthesis => JArray!["error", ")"],
        Token::CloseSquareBracket => JArray!["error", "]"],
        Token::CloseCurlyBracket => JArray!["error", "}"],
    }
}

/// A previous version of procedural-masquerade had a bug where it
/// would normalize consecutive whitespace to a single space,
/// including in string literals.
#[test]
fn procedural_masquerade_whitespace() {
    ascii_case_insensitive_phf_map! {
        map -> () = {
            "  \t\n" => ()
        }
    }
    assert_eq!(map::get("  \t\n"), Some(&()));
    assert_eq!(map::get(" "), None);

    match_ignore_ascii_case! { "  \t\n",
        " " => panic!("1"),
        "  \t\n" => {},
        _ => panic!("2"),
    }

    match_ignore_ascii_case! { " ",
        "  \t\n" => panic!("3"),
        " " => {},
        _ => panic!("4"),
    }
}

#[test]
fn parse_until_before_stops_at_delimiter_or_end_of_input() {
    // For all j and k, inputs[i].1[j] should parse the same as inputs[i].1[k]
    // when we use delimiters inputs[i].0.
    let inputs = vec![
        (
            Delimiter::Bang | Delimiter::Semicolon,
            // Note that the ';extra' is fine, because the ';' acts the same as
            // the end of input.
            vec!["token stream;extra", "token stream!", "token stream"],
        ),
        (Delimiter::Bang | Delimiter::Semicolon, vec![";", "!", ""]),
    ];
    for equivalent in inputs {
        for (j, x) in equivalent.1.iter().enumerate() {
            for y in equivalent.1[j + 1..].iter() {
                let mut ix = ParserInput::new(x);
                let mut ix = Parser::new(&mut ix);

                let mut iy = ParserInput::new(y);
                let mut iy = Parser::new(&mut iy);

                let _ = ix.parse_until_before::<_, _, ()>(equivalent.0, |ix| {
                    iy.parse_until_before::<_, _, ()>(equivalent.0, |iy| {
                        loop {
                            let ox = ix.next();
                            let oy = iy.next();
                            assert_eq!(ox, oy);
                            if ox.is_err() {
                                break;
                            }
                        }
                        Ok(())
                    })
                });
            }
        }
    }
}

#[test]
fn parser_maintains_current_line() {
    let mut input = ParserInput::new("ident ident;\nident ident ident;\nident");
    let mut parser = Parser::new(&mut input);
    assert_eq!(parser.current_line(), "ident ident;");
    assert_eq!(parser.next(), Ok(&Token::Ident("ident".into())));
    assert_eq!(parser.next(), Ok(&Token::Ident("ident".into())));
    assert_eq!(parser.next(), Ok(&Token::Semicolon));

    assert_eq!(parser.next(), Ok(&Token::Ident("ident".into())));
    assert_eq!(parser.current_line(), "ident ident ident;");
    assert_eq!(parser.next(), Ok(&Token::Ident("ident".into())));
    assert_eq!(parser.next(), Ok(&Token::Ident("ident".into())));
    assert_eq!(parser.next(), Ok(&Token::Semicolon));

    assert_eq!(parser.next(), Ok(&Token::Ident("ident".into())));
    assert_eq!(parser.current_line(), "ident");
}

#[test]
fn cdc_regression_test() {
    let mut input = ParserInput::new("-->x");
    let mut parser = Parser::new(&mut input);
    parser.skip_cdc_and_cdo();
    assert_eq!(parser.next(), Ok(&Token::Ident("x".into())));
    assert_eq!(
        parser.next(),
        Err(BasicParseError {
            kind: BasicParseErrorKind::EndOfInput,
            location: SourceLocation { line: 0, column: 5 }
        })
    );
}

#[test]
fn parse_entirely_reports_first_error() {
    #[derive(PartialEq, Debug)]
    enum E {
        Foo,
    }
    let mut input = ParserInput::new("ident");
    let mut parser = Parser::new(&mut input);
    let result: Result<(), _> = parser.parse_entirely(|p| Err(p.new_custom_error(E::Foo)));
    assert_eq!(
        result,
        Err(ParseError {
            kind: ParseErrorKind::Custom(E::Foo),
            location: SourceLocation { line: 0, column: 1 },
        })
    );
}

#[test]
fn parse_sourcemapping_comments() {
    let tests = vec![
        ("/*# sourceMappingURL=here*/", Some("here")),
        ("/*# sourceMappingURL=here  */", Some("here")),
        ("/*@ sourceMappingURL=here*/", Some("here")),
        (
            "/*@ sourceMappingURL=there*/ /*# sourceMappingURL=here*/",
            Some("here"),
        ),
        ("/*# sourceMappingURL=here there  */", Some("here")),
        ("/*# sourceMappingURL=  here  */", Some("")),
        ("/*# sourceMappingURL=*/", Some("")),
        ("/*# sourceMappingUR=here  */", None),
        ("/*! sourceMappingURL=here  */", None),
        ("/*# sourceMappingURL = here  */", None),
        ("/*   # sourceMappingURL=here   */", None),
    ];

    for test in tests {
        let mut input = ParserInput::new(test.0);
        let mut parser = Parser::new(&mut input);
        while parser.next_including_whitespace().is_ok() {}
        assert_eq!(parser.current_source_map_url(), test.1);
    }
}

#[test]
fn parse_sourceurl_comments() {
    let tests = vec![
        ("/*# sourceURL=here*/", Some("here")),
        ("/*# sourceURL=here  */", Some("here")),
        ("/*@ sourceURL=here*/", Some("here")),
        ("/*@ sourceURL=there*/ /*# sourceURL=here*/", Some("here")),
        ("/*# sourceURL=here there  */", Some("here")),
        ("/*# sourceURL=  here  */", Some("")),
        ("/*# sourceURL=*/", Some("")),
        ("/*# sourceMappingUR=here  */", None),
        ("/*! sourceURL=here  */", None),
        ("/*# sourceURL = here  */", None),
        ("/*   # sourceURL=here   */", None),
    ];

    for test in tests {
        let mut input = ParserInput::new(test.0);
        let mut parser = Parser::new(&mut input);
        while parser.next_including_whitespace().is_ok() {}
        assert_eq!(parser.current_source_url(), test.1);
    }
}

#[cfg_attr(all(miri, feature = "skip_long_tests"), ignore)]
#[test]
fn roundtrip_percentage_token() {
    fn test_roundtrip(value: &str) {
        let mut input = ParserInput::new(value);
        let mut parser = Parser::new(&mut input);
        let token = parser.next().unwrap();
        assert_eq!(token.to_css_string(), value);
    }
    // Test simple number serialization
    for i in 0..101 {
        test_roundtrip(&format!("{}%", i));
        for j in 0..10 {
            if j != 0 {
                test_roundtrip(&format!("{}.{}%", i, j));
            }
            for k in 1..10 {
                test_roundtrip(&format!("{}.{}{}%", i, j, k));
            }
        }
    }
}

#[test]
fn utf16_columns() {
    // This particular test serves two purposes.  First, it checks
    // that the column number computations are correct.  Second, it
    // checks that tokenizer code paths correctly differentiate
    // between the different UTF-8 encoding bytes.  In particular
    // different leader bytes and continuation bytes are treated
    // differently, so we make sure to include all lengths in the
    // tests, using the string "QΡ✈🆒".  Also, remember that because
    // the column is in units of UTF-16, the 4-byte sequence results
    // in two columns.
    let tests = vec![
        ("", 1),
        ("ascii", 6),
        ("/*QΡ✈🆒*/", 10),
        ("'QΡ✈🆒*'", 9),
        ("\"\\\"'QΡ✈🆒*'", 12),
        ("\\Q\\Ρ\\✈\\🆒", 10),
        ("QΡ✈🆒", 6),
        ("QΡ✈🆒\\Q\\Ρ\\✈\\🆒", 15),
        ("newline\r\nQΡ✈🆒", 6),
        ("url(QΡ✈🆒\\Q\\Ρ\\✈\\🆒)", 20),
        ("url(QΡ✈🆒)", 11),
        ("url(\r\nQΡ✈🆒\\Q\\Ρ\\✈\\🆒)", 16),
        ("url(\r\nQΡ✈🆒\\Q\\Ρ\\✈\\🆒", 15),
        ("url(\r\nQΡ✈🆒\\Q\\Ρ\\✈\\🆒 x", 17),
        ("QΡ✈🆒()", 8),
        // Test that under/over-flow of current_line_start_position is
        // handled properly; see the special case in consume_4byte_intro.
        ("🆒", 3),
    ];

    for test in tests {
        let mut input = ParserInput::new(test.0);
        let mut parser = Parser::new(&mut input);

        // Read all tokens.
        loop {
            match parser.next() {
                Err(BasicParseError {
                    kind: BasicParseErrorKind::EndOfInput,
                    ..
                }) => {
                    break;
                }
                Err(_) => {
                    // should this be an explicit panic instead?
                    unreachable!();
                }
                Ok(_) => {}
            };
        }

        // Check the resulting column.
        assert_eq!(parser.current_source_location().column, test.1);
    }
}

#[test]
fn servo_define_css_keyword_enum() {
    macro_rules! define_css_keyword_enum {
        (pub enum $name:ident { $($variant:ident = $css:pat,)+ }) => {
            #[derive(PartialEq, Debug)]
            pub enum $name {
                $($variant),+
            }

            impl $name {
                pub fn from_ident(ident: &str) -> Result<$name, ()> {
                    match_ignore_ascii_case! { ident,
                        $($css => Ok($name::$variant),)+
                        _ => Err(())
                    }
                }
            }
        }
    }
    define_css_keyword_enum! {
        pub enum UserZoom {
            Zoom = "zoom",
            Fixed = "fixed",
        }
    }

    assert_eq!(UserZoom::from_ident("fixed"), Ok(UserZoom::Fixed));
}
