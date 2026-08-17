// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate pest;

use std::collections::HashMap;

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::{state, ParseResult, Parser, ParserState, Span};

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Rule {
    json,
    object,
    pair,
    array,
    value,
    string,
    escape,
    unicode,
    hex,
    number,
    int,
    exp,
    bool,
    null,
}

struct JsonParser;

impl Parser<Rule> for JsonParser {
    // false positive: pest uses `..` as a complete range (historically)
    #[allow(clippy::almost_complete_range)]
    fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        fn json(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            value(state)
        }

        fn object(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.rule(Rule::object, |s| {
                s.sequence(|s| {
                    s.match_string("{")
                        .and_then(skip)
                        .and_then(pair)
                        .and_then(skip)
                        .and_then(|s| {
                            s.repeat(|s| {
                                s.sequence(|s| {
                                    s.match_string(",")
                                        .and_then(skip)
                                        .and_then(pair)
                                        .and_then(skip)
                                })
                            })
                        })
                        .and_then(|s| s.match_string("}"))
                })
                .or_else(|s| {
                    s.sequence(|s| {
                        s.match_string("{")
                            .and_then(skip)
                            .and_then(|s| s.match_string("}"))
                    })
                })
            })
        }

        fn pair(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.rule(Rule::pair, |s| {
                s.sequence(|s| {
                    string(s)
                        .and_then(skip)
                        .and_then(|s| s.match_string(":"))
                        .and_then(skip)
                        .and_then(value)
                })
            })
        }

        fn array(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.rule(Rule::array, |s| {
                s.sequence(|s| {
                    s.match_string("[")
                        .and_then(skip)
                        .and_then(value)
                        .and_then(skip)
                        .and_then(|s| {
                            s.repeat(|s| {
                                s.sequence(|s| {
                                    s.match_string(",")
                                        .and_then(skip)
                                        .and_then(value)
                                        .and_then(skip)
                                })
                            })
                        })
                        .and_then(|s| s.match_string("]"))
                })
                .or_else(|s| {
                    s.sequence(|s| {
                        s.match_string("[")
                            .and_then(skip)
                            .and_then(|s| s.match_string("]"))
                    })
                })
            })
        }

        fn value(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.rule(Rule::value, |s| {
                string(s)
                    .or_else(number)
                    .or_else(object)
                    .or_else(array)
                    .or_else(bool)
                    .or_else(null)
            })
        }

        fn string(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.rule(Rule::string, |s| {
                s.match_string("\"")
                    .and_then(|s| {
                        s.repeat(|s| {
                            escape(s).or_else(|s| {
                                s.sequence(|s| {
                                    s.lookahead(false, |s| {
                                        s.match_string("\"").or_else(|s| s.match_string("\\"))
                                    })
                                    .and_then(|s| s.skip(1))
                                })
                            })
                        })
                    })
                    .and_then(|pos| pos.match_string("\""))
            })
        }

        fn escape(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.sequence(|s| {
                s.match_string("\\").and_then(|s| {
                    s.match_string("\"")
                        .or_else(|s| s.match_string("\\"))
                        .or_else(|s| s.match_string("/"))
                        .or_else(|s| s.match_string("b"))
                        .or_else(|s| s.match_string("f"))
                        .or_else(|s| s.match_string("n"))
                        .or_else(|s| s.match_string("r"))
                        .or_else(|s| s.match_string("t"))
                        .or_else(unicode)
                })
            })
        }

        fn unicode(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.sequence(|s| {
                s.match_string("u")
                    .and_then(hex)
                    .and_then(hex)
                    .and_then(hex)
            })
        }

        fn hex(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state
                .match_range('0'..'9')
                .or_else(|s| s.match_range('a'..'f'))
                .or_else(|s| s.match_range('A'..'F'))
        }

        fn number(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.rule(Rule::number, |s| {
                s.sequence(|s| {
                    s.optional(|s| s.match_string("-"))
                        .and_then(int)
                        .and_then(|s| {
                            s.optional(|s| {
                                s.sequence(|s| {
                                    s.match_string(".")
                                        .and_then(|s| s.match_range('0'..'9'))
                                        .and_then(|s| s.repeat(|s| s.match_range('0'..'9')))
                                        .and_then(|s| s.optional(exp))
                                        .or_else(exp)
                                })
                            })
                        })
                })
            })
        }

        fn int(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.match_string("0").or_else(|s| {
                s.sequence(|s| {
                    s.match_range('1'..'9')
                        .and_then(|s| s.repeat(|s| s.match_range('0'..'9')))
                })
            })
        }

        fn exp(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.sequence(|s| {
                s.match_string("E")
                    .or_else(|s| s.match_string("e"))
                    .and_then(|s| {
                        s.optional(|s| s.match_string("+").or_else(|s| s.match_string("-")))
                    })
                    .and_then(int)
            })
        }

        fn bool(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.rule(Rule::bool, |s| {
                s.match_string("true").or_else(|s| s.match_string("false"))
            })
        }

        fn null(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.rule(Rule::null, |s| s.match_string("null"))
        }

        fn skip(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.repeat(|s| {
                s.match_string(" ")
                    .or_else(|s| s.match_string("\t"))
                    .or_else(|s| s.match_string("\r"))
                    .or_else(|s| s.match_string("\n"))
            })
        }

        state(input, |state| match rule {
            Rule::json => json(state),
            Rule::object => object(state),
            Rule::pair => pair(state),
            Rule::array => array(state),
            Rule::value => value(state),
            Rule::string => string(state),
            Rule::escape => escape(state),
            Rule::unicode => unicode(state),
            Rule::hex => hex(state),
            Rule::number => number(state),
            Rule::int => int(state),
            Rule::exp => exp(state),
            Rule::bool => bool(state),
            Rule::null => null(state),
        })
    }
}

#[derive(Debug, PartialEq)]
enum Json<'i> {
    Null,
    Bool(bool),
    Number(f64),
    String(Span<'i>),
    Array(Vec<Json<'i>>),
    Object(HashMap<Span<'i>, Json<'i>>),
}

fn consume(pair: Pair<Rule>) -> Json {
    fn value(pair: Pair<Rule>) -> Json {
        let pair = pair.into_inner().next().unwrap();

        match pair.as_rule() {
            Rule::null => Json::Null,
            Rule::bool => match pair.as_str() {
                "false" => Json::Bool(false),
                "true" => Json::Bool(true),
                _ => unreachable!(),
            },
            Rule::number => Json::Number(pair.as_str().parse().unwrap()),
            Rule::string => Json::String(pair.as_span()),
            Rule::array => Json::Array(pair.into_inner().map(value).collect()),
            Rule::object => {
                let pairs = pair.into_inner().map(|pos| {
                    let mut pair = pos.into_inner();

                    let key = pair.next().unwrap().as_span();
                    let value = value(pair.next().unwrap());

                    (key, value)
                });

                Json::Object(pairs.collect())
            }
            _ => unreachable!(),
        }
    }

    value(pair)
}

#[test]
fn null() {
    parses_to! {
        parser: JsonParser,
        input: "null",
        rule: Rule::null,
        tokens: [
            null(0, 4)
        ]
    };
}

#[test]
fn bool() {
    parses_to! {
        parser: JsonParser,
        input: "false",
        rule: Rule::bool,
        tokens: [
            bool(0, 5)
        ]
    };
}

#[test]
fn number_zero() {
    parses_to! {
        parser: JsonParser,
        input: "0",
        rule: Rule::number,
        tokens: [
            number(0, 1)
        ]
    };
}

#[test]
fn float() {
    parses_to! {
        parser: JsonParser,
        input: "100.001",
        rule: Rule::number,
        tokens: [
            number(0, 7)
        ]
    };
}

#[test]
fn float_with_exp() {
    parses_to! {
        parser: JsonParser,
        input: "100.001E+100",
        rule: Rule::number,
        tokens: [
            number(0, 12)
        ]
    };
}

#[test]
fn number_minus_zero() {
    parses_to! {
        parser: JsonParser,
        input: "-0",
        rule: Rule::number,
        tokens: [
            number(0, 2)
        ]
    };
}

#[test]
fn string_with_escapes() {
    parses_to! {
        parser: JsonParser,
        input: "\"asd\\u0000\\\"\"",
        rule: Rule::string,
        tokens: [
            string(0, 13)
        ]
    };
}

#[test]
fn array_empty() {
    parses_to! {
        parser: JsonParser,
        input: "[ ]",
        rule: Rule::array,
        tokens: [
            array(0, 3)
        ]
    };
}

#[test]
fn array() {
    parses_to! {
        parser: JsonParser,
        input: "[0.0e1, false, null, \"a\", [0]]",
        rule: Rule::array,
        tokens: [
            array(0, 30, [
                value(1,  6, [number(1, 6)]),
                value(8, 13, [bool(8, 13)]),
                value(15, 19, [null(15, 19)]),
                value(21, 24, [string(21, 24)]),
                value(26, 29, [
                    array(26, 29, [
                        value(27, 28, [number(27, 28)])
                    ])
                ])
            ])
        ]
    };
}

#[test]
fn object() {
    parses_to! {
        parser: JsonParser,
        input: "{\"a\" : 3, \"b\" : [{}, 3]}",
        rule: Rule::object,
        tokens: [
            object(0, 24, [
                pair(1, 8, [
                    string(1, 4),
                    value(7, 8, [number(7, 8)])
                ]),
                pair(10, 23, [
                    string(10, 13),
                    value(16, 23, [
                        array(16, 23, [
                            value(17, 19, [object(17, 19)]),
                            value(21, 22, [number(21, 22)])
                        ])
                    ])
                ])
            ])
        ]
    };
}

#[test]
fn ast() {
    let input = "{\"a\": [null, true, 3.4]}";

    let ast = consume(
        JsonParser::parse(Rule::json, input)
            .unwrap()
            .next()
            .unwrap(),
    );

    if let Json::Object(pairs) = ast {
        let vals: Vec<&Json> = pairs.values().collect();

        assert_eq!(
            **vals.first().unwrap(),
            Json::Array(vec![Json::Null, Json::Bool(true), Json::Number(3.4)])
        );
    }
}
