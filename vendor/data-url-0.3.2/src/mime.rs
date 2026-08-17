use alloc::{borrow::ToOwned, string::String, vec::Vec};
use core::fmt::{self, Write};
use core::str::FromStr;

/// <https://mimesniff.spec.whatwg.org/#mime-type-representation>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mime {
    pub type_: String,
    pub subtype: String,
    /// (name, value)
    pub parameters: Vec<(String, String)>,
}

impl Mime {
    /// Construct a new [`Mime`] with the given `type_` and `subtype` and an
    /// empty parameter list.
    pub fn new(type_: &str, subtype: &str) -> Self {
        Self {
            type_: type_.into(),
            subtype: subtype.into(),
            parameters: vec![],
        }
    }

    /// Return true if this [`Mime`] matches a given type and subtype, regardless
    /// of what parameters it has.
    pub fn matches(&self, type_: &str, subtype: &str) -> bool {
        self.type_ == type_ && self.subtype == subtype
    }

    pub fn get_parameter<P>(&self, name: &P) -> Option<&str>
    where
        P: ?Sized + PartialEq<str>,
    {
        self.parameters
            .iter()
            .find(|&(n, _)| name == &**n)
            .map(|(_, v)| &**v)
    }
}

#[derive(Debug)]
pub struct MimeParsingError(());

impl fmt::Display for MimeParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid mime type")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MimeParsingError {}

/// <https://mimesniff.spec.whatwg.org/#parsing-a-mime-type>
impl FromStr for Mime {
    type Err = MimeParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s).ok_or(MimeParsingError(()))
    }
}

fn parse(s: &str) -> Option<Mime> {
    let trimmed = s.trim_matches(http_whitespace);

    let (type_, rest) = split2(trimmed, '/');
    require!(only_http_token_code_points(type_) && !type_.is_empty());

    let (subtype, rest) = split2(rest?, ';');
    let subtype = subtype.trim_end_matches(http_whitespace);
    require!(only_http_token_code_points(subtype) && !subtype.is_empty());

    let mut parameters = Vec::new();
    if let Some(rest) = rest {
        parse_parameters(rest, &mut parameters)
    }

    Some(Mime {
        type_: type_.to_ascii_lowercase(),
        subtype: subtype.to_ascii_lowercase(),
        parameters,
    })
}

fn split2(s: &str, separator: char) -> (&str, Option<&str>) {
    let mut iter = s.splitn(2, separator);
    let first = iter.next().unwrap();
    (first, iter.next())
}

fn parse_parameters(s: &str, parameters: &mut Vec<(String, String)>) {
    let mut semicolon_separated = s.split(';');

    while let Some(piece) = semicolon_separated.next() {
        let piece = piece.trim_start_matches(http_whitespace);
        let (name, value) = split2(piece, '=');
        // We can not early return on an invalid name here, because the value
        // parsing later may consume more semicolon seperated pieces.
        let name_valid =
            !name.is_empty() && only_http_token_code_points(name) && !contains(parameters, name);
        if let Some(value) = value {
            let value = if let Some(stripped) = value.strip_prefix('"') {
                let max_len = stripped.len().saturating_sub(1); // without end quote
                let mut unescaped_value = String::with_capacity(max_len);
                let mut chars = stripped.chars();
                'until_closing_quote: loop {
                    while let Some(c) = chars.next() {
                        match c {
                            '"' => break 'until_closing_quote,
                            '\\' => unescaped_value.push(chars.next().unwrap_or_else(|| {
                                semicolon_separated
                                    .next()
                                    .map(|piece| {
                                        // A semicolon inside a quoted value is not a separator
                                        // for the next parameter, but part of the value.
                                        chars = piece.chars();
                                        ';'
                                    })
                                    .unwrap_or('\\')
                            })),
                            _ => unescaped_value.push(c),
                        }
                    }
                    if let Some(piece) = semicolon_separated.next() {
                        // A semicolon inside a quoted value is not a separator
                        // for the next parameter, but part of the value.
                        unescaped_value.push(';');
                        chars = piece.chars()
                    } else {
                        break;
                    }
                }
                if !name_valid || !valid_value(value) {
                    continue;
                }
                unescaped_value
            } else {
                let value = value.trim_end_matches(http_whitespace);
                if value.is_empty() {
                    continue;
                }
                if !name_valid || !valid_value(value) {
                    continue;
                }
                value.to_owned()
            };
            parameters.push((name.to_ascii_lowercase(), value))
        }
    }
}

fn contains(parameters: &[(String, String)], name: &str) -> bool {
    parameters.iter().any(|(n, _)| n == name)
}

fn valid_value(s: &str) -> bool {
    s.chars().all(|c| {
        // <https://mimesniff.spec.whatwg.org/#http-quoted-string-token-code-point>
        matches!(c, '\t' | ' '..='~' | '\u{80}'..='\u{FF}')
    })
}

/// <https://mimesniff.spec.whatwg.org/#serializing-a-mime-type>
impl fmt::Display for Mime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.type_)?;
        f.write_str("/")?;
        f.write_str(&self.subtype)?;
        for (name, value) in &self.parameters {
            f.write_str(";")?;
            f.write_str(name)?;
            f.write_str("=")?;
            if only_http_token_code_points(value) && !value.is_empty() {
                f.write_str(value)?
            } else {
                f.write_str("\"")?;
                for c in value.chars() {
                    if c == '"' || c == '\\' {
                        f.write_str("\\")?
                    }
                    f.write_char(c)?
                }
                f.write_str("\"")?
            }
        }
        Ok(())
    }
}

fn http_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

fn only_http_token_code_points(s: &str) -> bool {
    s.bytes().all(|byte| IS_HTTP_TOKEN[byte as usize])
}

macro_rules! byte_map {
    ($($flag:expr,)*) => ([
        $($flag != 0,)*
    ])
}

// Copied from https://github.com/hyperium/mime/blob/v0.3.5/src/parse.rs#L293
#[rustfmt::skip]
static IS_HTTP_TOKEN: [bool; 256] = byte_map![
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[test]
fn test_basic_mime() {
    let mime = Mime::new("text", "plain");
    assert!(mime.matches("text", "plain"));

    let cloned = mime.clone();
    assert!(cloned.matches("text", "plain"));

    let mime = Mime {
        type_: "text".into(),
        subtype: "html".into(),
        parameters: vec![("one".into(), "two".into())],
    };
    assert!(mime.matches("text", "html"));
}
