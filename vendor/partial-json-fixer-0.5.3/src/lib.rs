//! Partial JSON fixer
//!
//! This is a zero dependency partial json fixer.
//! It is very lenient, and will accept some erroneous JSON too. For example, {key: "value"} would be valid.
//!
//! This can be used to parse partial json coming from a stream.

use std::{fmt::Display, str::CharIndices};

/// Takes a partial JSON string, kinda parses it and returns a complete JSON object
/// The JSON is tokenized and parsed. It can then be converted to a string with `.to_string()`
/// method
pub fn fix_json_parse(partial_json: &str) -> JResult<JsonValue> {
    let tokenizer = JsonTokenizer::new(partial_json);
    let parser = JsonParser::new(tokenizer);

    let value = parser.parse()?;
    Ok(value)
}

/// Takes a partial JSON string, kinda parses it and returns a complete JSON string.
/// This function keeps the JSON as a string, goes through it and analyzes the brackets, strings,
/// etc, to determine the missing stuff, and adds it.
/// This approach is likely faster than the parsing appraoch (TODO: benchmmark maybe)
/// It's assumed that the given JSON would **always** be a valid incomplete JSON.
pub fn fix_json(partial_json: &str) -> String {
    enum Wrapper {
        Brace,
        SquareBracket,
        Quote,
        Escape,
        ObjectKey,
        ObjectValue,
    }
    let chars = partial_json.chars();
    let mut wrappers = vec![];
    for c in chars {
        match wrappers.last() {
            Some(Wrapper::Quote) => {
                if c == '"' {
                    wrappers.pop();
                } else if c == '\\' {
                    wrappers.push(Wrapper::Escape);
                }
            }
            Some(Wrapper::Escape) => {
                wrappers.pop(); // get out of escape mode 
            }
            _ => {
                match c {
                    '{' => {
                        wrappers.push(Wrapper::Brace);
                    }
                    '}' => {
                        wrappers.pop(); // we assume it's correct JSON
                        if matches!(wrappers.last(), Some(Wrapper::ObjectValue)) {
                            wrappers.pop();
                        }
                    }
                    '[' => {
                        wrappers.push(Wrapper::SquareBracket);
                    }
                    ']' => {
                        wrappers.pop();
                        if matches!(wrappers.last(), Some(Wrapper::ObjectValue)) {
                            wrappers.pop();
                        }
                    }
                    '"' => {
                        if matches!(wrappers.last(), Some(Wrapper::Brace)) {
                            wrappers.push(Wrapper::ObjectKey);
                        } else if matches!(wrappers.last(), Some(Wrapper::ObjectValue)) {
                            wrappers.pop();
                        }
                        wrappers.push(Wrapper::Quote);
                    },
                    ':' => {
                        wrappers.pop(); // pop ObjectKey
                        wrappers.push(Wrapper::ObjectValue);
                    },
                    ',' => {
                        if matches!(wrappers.last(), Some(Wrapper::ObjectValue)) {
                            wrappers.pop();
                        }
                    },
                    w if w.is_whitespace() => {},
                    _ => {
                        // non whitespace
                        if matches!(wrappers.last(), Some(Wrapper::ObjectValue)) {
                            wrappers.pop();
                        }
                    }
                }
            }
        }
    }

    let end_index = if partial_json.trim_end().ends_with(',') {
        partial_json.rfind(',').unwrap()
    } else {
        partial_json.len()
    };

    let mut final_json = partial_json[0..end_index].to_string();
    while let Some(wrapper) = wrappers.pop() {
        match wrapper {
            Wrapper::Brace => {
                final_json.push('}');
                if matches!(wrappers.last(), Some(Wrapper::ObjectValue)) {
                    wrappers.pop();
                }
            },
            Wrapper::SquareBracket => {
                final_json.push(']');
                if matches!(wrappers.last(), Some(Wrapper::ObjectValue)) {
                    wrappers.pop();
                }
            },
            Wrapper::Quote => {
                final_json.push('"');
                if matches!(wrappers.last(), Some(Wrapper::ObjectValue)) {
                    wrappers.pop();
                }
            },
            Wrapper::Escape => {
                final_json.push('\\');
            },
            Wrapper::ObjectKey => {
                final_json.push_str(": null");
            },
            Wrapper::ObjectValue => {
                final_json.push_str(" null");
            },
        }
    }

    // todo: remove traiiling comma

    final_json
}

struct JsonParser<'a> {
    tokenizer: JsonTokenizer<'a>,
}

impl<'a> JsonParser<'a> {
    fn new(tokenizer: JsonTokenizer<'a>) -> Self {
        Self { tokenizer }
    }

    fn parse(mut self) -> JResult<JsonValue<'a>> {
        let (_errors, value) = self.parse_value()?;
        Ok(value)
    }

    fn parse_value(&mut self) -> JResult<(Vec<JsonError>, JsonValue<'a>)> {
        let token = self.tokenizer.next().ok_or(JsonError::UnexpectedEnd)?;

        match token.kind {
            JsonTokenKind::Null | JsonTokenKind::String | JsonTokenKind::Number => {
                Ok((vec![], JsonValue::Unit(self.token_as_unit(&token))))
            }
            JsonTokenKind::OpeningBrace => Ok((vec![], JsonValue::Object(self.parse_object()?))),
            JsonTokenKind::OpeningSquareBracket => {
                Ok((vec![], JsonValue::Array(self.parse_array()?)))
            }
            JsonTokenKind::Comma
            | JsonTokenKind::Colon
            | JsonTokenKind::ClosingBrace
            | JsonTokenKind::ClosingSquareBracket => Err(JsonError::ExpectedToken {
                got: token,
                expected: None,
            }),
        }
    }

    fn token_as_unit(&self, token: &JsonToken) -> JsonUnit<'a> {
        let source = self.tokenizer.span_source(&token);
        if source.starts_with("\"") {
            return JsonUnit::String(source.trim_matches('"'));
        }
        if source == "true" {
            return JsonUnit::True;
        }
        if source == "false" {
            return JsonUnit::False;
        }
        if source.parse::<isize>().is_ok() {
            return JsonUnit::Number(source);
        }
        return JsonUnit::Null;
    }

    fn parse_unit(&mut self) -> JResult<JsonUnit<'a>> {
        let t = self.tokenizer.next().ok_or(JsonError::UnexpectedEnd)?;
        match t.kind {
            JsonTokenKind::String | JsonTokenKind::Number => Ok(self.token_as_unit(&t)),
            _ => Err(JsonError::ExpectedToken {
                got: t,
                expected: None,
            }),
        }
    }

    fn parse_array(&mut self) -> JResult<JsonArray<'a>> {
        let mut members = vec![];
        loop {
            if self.tokenizer.is_next_closing_square_bracket() || self.tokenizer.is_on_last() {
                break;
            }
            if let Ok((_errors, value)) = self.parse_value() {
                members.push(value);

                match self.tokenizer.next() {
                    Some(token) if matches!(token.kind, JsonTokenKind::ClosingSquareBracket) => {
                        break;
                    }
                    Some(token) if matches!(token.kind, JsonTokenKind::Comma) => {}
                    Some(token) => {
                        return Err(JsonError::ExpectedToken {
                            got: token,
                            expected: Some(JsonTokenKind::ClosingSquareBracket),
                        })
                    }
                    None => {}
                }
            } else {
                break;
            }
        }
        Ok(JsonArray { members })
    }

    fn parse_object(&mut self) -> JResult<JsonObject<'a>> {
        let mut values = vec![];
        loop {
            if self.tokenizer.is_next_closing_brace() || self.tokenizer.is_on_last() {
                break;
            }
            let key = self.parse_unit();
            if key.is_err() {
                break;
            }
            let key = key.unwrap();
            // parse colon
            if self.tokenizer.next().is_none() {
                values.push((key, JsonValue::Null));
                break;
            }
            let value = self.parse_value();
            if value.is_err() {
                values.push((key, JsonValue::Null));
                break;
            }
            let (_errors, value) = value.unwrap();
            values.push((key, value));

            match self.tokenizer.next() {
                Some(token) if matches!(token.kind, JsonTokenKind::ClosingBrace) => {
                    break;
                }
                Some(token) if matches!(token.kind, JsonTokenKind::Comma) => {}
                Some(token) => {
                    return Err(JsonError::ExpectedToken {
                        got: token,
                        expected: Some(JsonTokenKind::ClosingBrace),
                    })
                }
                None => {}
            }
        }
        Ok(JsonObject { values })
    }
}

type JResult<T> = Result<T, JsonError>;

#[derive(Debug)]
pub enum JsonError {
    UnexpectedEnd,
    ExpectedToken {
        got: JsonToken,
        expected: Option<JsonTokenKind>,
    },
}
impl std::error::Error for JsonError {}

impl Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonError::UnexpectedEnd => write!(f, "Unexpected end of input"),
            JsonError::ExpectedToken { got, expected } => {
                if let Some(expected) = expected {
                    write!(
                        f,
                        "Expected token {:?} at char {}, got {:?}",
                        expected, got.span.start, got.kind
                    )
                } else {
                    write!(
                        f,
                        "Unexpected token {:?} at char {}",
                        got.kind, got.span.start
                    )
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum JsonValue<'a> {
    Array(JsonArray<'a>),
    Object(JsonObject<'a>),
    Unit(JsonUnit<'a>),
    Null,
}

impl<'a> Display for JsonValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::Unit(unit) => {
                write!(f, "{unit}")
            }
            JsonValue::Object(object) => write!(f, "{object}"),
            JsonValue::Array(array) => write!(f, "{array}"),
            JsonValue::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug)]
pub struct JsonArray<'a> {
    pub members: Vec<JsonValue<'a>>,
}

impl<'a> Display for JsonArray<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.members
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug)]
pub struct JsonObject<'a> {
    pub values: Vec<(JsonUnit<'a>, JsonValue<'a>)>,
}
impl<'a> Display for JsonObject<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.values
                .iter()
                .map(|(key, value)| format!("{}: {}", key, value))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug)]
pub enum JsonUnit<'a> {
    Null,
    Number(&'a str),
    String(&'a str),
    True,
    False,
}

impl<'a> Display for JsonUnit<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Null => write!(f, "null"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "\"{s}\""),
        }
    }
}

struct JsonTokenizer<'a> {
    source: &'a str,
    char_indices: CharIndices<'a>,
}

impl<'a> JsonTokenizer<'a> {
    fn new(source: &'a str) -> Self {
        let char_indices = source.char_indices();
        Self {
            source,
            char_indices,
        }
    }

    fn span_source(&self, token: &JsonToken) -> &'a str {
        &self.source[token.span.start..token.span.end]
    }

    fn skip_whitespace_and_next(&mut self) -> Option<(usize, char)> {
        let mut it_clone = self.char_indices.clone();
        let mut v = it_clone.next();
        while let Some((_i, c)) = v {
            if !c.is_whitespace() {
                break;
            }
            v = it_clone.next();
        }
        self.char_indices = it_clone;
        v
    }

    fn consume_number_or_null(&mut self, first_index: usize) -> Option<JsonToken> {
        let mut it_clone = self.char_indices.clone();
        let mut last_index = first_index;
        loop {
            if let Some((i, c)) = it_clone.next() {
                last_index = i;
                if !c.is_alphanumeric() {
                    break;
                }
                self.char_indices.next();
            } else {
                last_index += 1;
                self.char_indices.next();
                break;
            }
        }
        // todo: consider failure case?
        Some(JsonToken {
            kind: JsonTokenKind::Number,
            span: Span {
                start: first_index,
                end: last_index,
            },
        })
    }

    fn is_next_closing_brace(&self) -> bool {
        let mut it_clone = self.char_indices.clone();
        it_clone.next().is_some_and(|(_i, c)| c == '}')
    }

    fn is_next_closing_square_bracket(&self) -> bool {
        let mut it_clone = self.char_indices.clone();
        it_clone.next().is_some_and(|(_i, c)| c == ']')
    }

    fn is_on_last(&self) -> bool {
        let mut it_clone = self.char_indices.clone();
        it_clone.next().is_some() && it_clone.next().is_none()
    }

    fn next(&mut self) -> Option<JsonToken> {
        let (i, c) = self.skip_whitespace_and_next()?;

        let t = match c {
            '{' => Some(JsonTokenKind::OpeningBrace),
            '}' => Some(JsonTokenKind::ClosingBrace),
            '[' => Some(JsonTokenKind::OpeningSquareBracket),
            ']' => Some(JsonTokenKind::ClosingSquareBracket),
            ',' => Some(JsonTokenKind::Comma),
            ':' => Some(JsonTokenKind::Colon),
            _ => None,
        };
        if t.is_some() {
            return t.map(|t| JsonToken {
                kind: t,
                span: Span {
                    start: i,
                    end: i + 1,
                },
            });
        }

        if c == '"' {
            // i need to consume the whole string
            let mut previous_char = None;
            let mut string_end_index = i + c.len_utf8();
            for (i, str_char) in self.char_indices.by_ref() {
                string_end_index = i + str_char.len_utf8();
                if str_char == '"' {
                    if let Some('\\') = previous_char {
                    } else {
                        break;
                    }
                }
                previous_char = Some(str_char);
            }
            return Some(JsonToken {
                kind: JsonTokenKind::String,
                span: Span {
                    start: i,
                    end: string_end_index,
                },
            });
        };
        // let's just assume it's a number if nothing else
        self.consume_number_or_null(i)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct JsonToken {
    kind: JsonTokenKind,
    span: Span,
}

#[derive(Clone, Copy, Debug)]
pub enum JsonTokenKind {
    OpeningBrace,
    ClosingBrace,
    OpeningSquareBracket,
    ClosingSquareBracket,
    Comma,
    Colon,
    String,
    Number,
    Null,
}
