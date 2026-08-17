mod args;

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use proc_macro::{Group, Ident, Literal, Span, TokenStream, TokenTree};

use self::args::{
    parse_arguments, pop_argument, pop_optional_argument, Argument, OutputType, PatternString,
    TranslateString,
};

pub(crate) fn combine(item: TokenStream) -> TokenStream {
    let mut args = parse_arguments(item, "combine!");
    let output = pop_argument::<OutputType>(&mut args, "combine!", "output");
    let input = pop_argument::<TokenStream>(&mut args, "combine!", "input");
    let prefix = pop_optional_argument::<TokenStream>(&mut args, "combine!", "prefix");
    let suffix = pop_optional_argument::<TokenStream>(&mut args, "combine!", "suffix");
    let paren = pop_optional_argument::<TokenTree>(&mut args, "combine!", "paren");
    let paren_prefix = pop_optional_argument::<TokenStream>(&mut args, "combine!", "paren_prefix");
    let paren_suffix = pop_optional_argument::<TokenStream>(&mut args, "combine!", "paren_suffix");
    let span = pop_optional_argument::<Span>(&mut args, "combine!", "span");

    let mut s = String::with_capacity(32);

    parse_stream(&mut s, "input", output, input);

    let span = span.unwrap_or_else(Span::call_site);

    let output_token = match output {
        OutputType::Ident => TokenTree::Ident(Ident::new(to_ident(&s).as_ref(), span)),
        OutputType::String => TokenTree::Literal(Literal::string(s.as_str())),
        OutputType::ISize => {
            let s = s.replace(|c: char| c != '+' && c != '-' && !c.is_ascii_digit(), "");
            TokenTree::Literal(Literal::isize_suffixed(
                s.parse::<isize>().expect("input: Expected a valid isize"),
            ))
        }
        OutputType::RawString => unreachable!(),
    };

    let mut stream = TokenStream::new();
    if let Some(prefix) = prefix {
        stream.extend(prefix);
    }
    if let Some(paren) = paren {
        let TokenTree::Group(group) = paren else {
            panic!("combine!: Expected an empty group in paren");
        };
        let mut paren_stream = TokenStream::new();
        if let Some(paren_prefix) = paren_prefix {
            paren_stream.extend(paren_prefix);
        }
        paren_stream.extend([output_token]);
        if let Some(paren_suffix) = paren_suffix {
            paren_stream.extend(paren_suffix);
        }
        let group = Group::new(group.delimiter(), paren_stream);
        stream.extend([TokenTree::Group(group)]);
    } else {
        stream.extend([output_token]);
    }
    if let Some(suffix) = suffix {
        stream.extend(suffix);
    }
    stream
}

#[allow(unknown_lints, tail_expr_drop_order)]
fn parse_stream(s: &mut String, arg: &str, output: OutputType, input: TokenStream) {
    let mut input = input.into_iter();
    while let Some(token) = input.next() {
        match token {
            TokenTree::Literal(literal) => {
                let literal = literal.to_string();
                if literal.starts_with('"') && literal.ends_with('"') {
                    decode_literal_string(
                        s,
                        arg,
                        literal
                            .strip_prefix('"')
                            .unwrap()
                            .strip_suffix('"')
                            .unwrap(),
                    );
                } else if literal.starts_with('\'') && literal.ends_with('\'') {
                    decode_literal_string(
                        s,
                        arg,
                        literal
                            .strip_prefix('\'')
                            .unwrap()
                            .strip_suffix('\'')
                            .unwrap(),
                    );
                } else if literal.starts_with(|c: char| c.is_ascii_digit()) {
                    s.push_str(&literal);
                } else {
                    panic!(
                        "{arg}: Expected a literal string or numeric literal, got `{literal:?}`"
                    );
                }
            }
            TokenTree::Ident(ident) => {
                let ident = ident.to_string();
                if output != OutputType::RawString
                    && ident.len() > 2
                    && ident.starts_with("__")
                    && ident.ends_with("__")
                {
                    parse_function(s, arg, &mut input, &ident);
                } else {
                    s.push_str(&ident);
                }
            }
            TokenTree::Group(group) => parse_stream(s, arg, output, group.stream()),
            TokenTree::Punct(punct) => s.push(punct.as_char()),
        }
    }
}

/// Resolve `__LOCATIONHASH__`'s optional `ignore_base` into a crate-root
/// directory.
///
/// The marker token's span locates the crate and its string value is a path
/// suffix stripped to reach that crate's root. Returns `None` (ignore nothing)
/// when `local_file` is unavailable, the suffix does not match, or the base
/// would be empty/filesystem-root (would match every token).
fn pop_ignore_base(args: &mut HashMap<String, TokenTree>, ident: &str) -> Option<PathBuf> {
    let token = args.remove("ignore_base")?;
    let span = <Span as Argument>::from_token_tree(token.clone())
        .unwrap_or_else(|e| panic!("{ident}: Invalid argument 'ignore_base': {e}"));
    let suffix = <String as Argument>::from_token_tree(token)
        .unwrap_or_else(|e| panic!("{ident}: Invalid argument 'ignore_base': {e}"));

    let file = crate::fallback::local_file(&span)?;
    if !file.ends_with(&suffix) {
        return None;
    }
    let components = Path::new(&suffix).components().count();
    let base = file.ancestors().nth(components)?;
    if base.as_os_str().is_empty() || base.parent().is_none() {
        return None;
    }
    Some(base.to_path_buf())
}

fn parse_function(
    s: &mut String,
    arg: &str,
    input: &mut proc_macro::token_stream::IntoIter,
    ident: &String,
) {
    let func = &ident[2..ident.len() - 2];
    let Some(TokenTree::Group(group)) = input.next() else {
        panic!("Expected arguments after {ident:?}");
    };
    let mut args = parse_arguments(group.stream(), ident);
    match func {
        "FILE" => {
            let of = pop_optional_argument::<Span>(&mut args, ident, "of")
                .unwrap_or_else(Span::call_site);
            let file = crate::fallback::file(&of);
            s.push_str(&file);
        }
        "LINE" => {
            let of = pop_optional_argument::<Span>(&mut args, ident, "of")
                .unwrap_or_else(Span::call_site);
            let line = crate::fallback::line(&of);
            s.push_str(&line.to_string());
        }
        "COLUMN" => {
            let of = pop_optional_argument::<Span>(&mut args, ident, "of")
                .unwrap_or_else(Span::call_site);
            let column = crate::fallback::column(&of);
            s.push_str(&column.to_string());
        }
        "SOURCE" => {
            let of = pop_argument::<TokenStream>(&mut args, ident, "of");
            for (i, token) in of.into_iter().enumerate() {
                if i > 0 {
                    s.push(' ');
                }
                s.push_str(
                    crate::fallback::source_text(&token.span())
                        .unwrap_or_default()
                        .as_ref(),
                );
            }
        }
        "DEBUG" => {
            let of = pop_argument::<TokenTree>(&mut args, ident, "of");
            s.push_str(&format!("{:?}", of));
        }
        "HASH" => {
            let input = pop_argument::<String>(&mut args, ident, "string");
            let alphabet = pop_optional_argument::<TranslateString>(&mut args, ident, "alphabet");
            let mut hash = crate::hash::xx3::xx3hash(&input);
            if let Some(alphabet) = alphabet {
                if hash == 0 {
                    s.push(alphabet.char(0));
                } else {
                    let mut hash_str = String::with_capacity(32);
                    while hash > 0 {
                        hash_str.push(alphabet.char(hash as usize % alphabet.len()));
                        hash /= alphabet.len() as u64;
                    }
                    s.extend(hash_str.chars().rev());
                }
            } else {
                s.push_str(&format!("{:016x}", hash));
            }
        }
        "LOCATIONHASH" => {
            let input = pop_argument::<TokenStream>(&mut args, ident, "of");
            let alphabet = pop_optional_argument::<TranslateString>(&mut args, ident, "alphabet");
            let ignore_base = pop_ignore_base(&mut args, ident);
            let mut hash = crate::hash::location_hash(input, ignore_base.as_deref());
            if let Some(alphabet) = alphabet {
                if hash == 0 {
                    s.push(alphabet.char(0));
                } else {
                    let mut hash_str = String::with_capacity(32);
                    while hash > 0 {
                        hash_str.push(alphabet.char(hash as usize % alphabet.len()));
                        hash /= alphabet.len() as u64;
                    }
                    s.extend(hash_str.chars().rev());
                }
            } else {
                s.push_str(&format!("{:016x}", hash));
            }
        }
        "LENGTH" => {
            let input = pop_argument::<String>(&mut args, ident, "string");
            s.push_str(&input.len().to_string());
        }
        "REPLACE" => {
            let input = pop_argument::<String>(&mut args, ident, "input");
            let pattern = pop_argument::<PatternString>(&mut args, ident, "pattern");
            let replacement = pop_argument::<String>(&mut args, ident, "replacement");

            match pattern {
                PatternString::Range(range) => {
                    s.push_str(&input.replace(|c: char| range.is_match(c), &replacement));
                }
                PatternString::String(string) => {
                    s.push_str(&input.replace(&string, &replacement));
                }
            }
        }
        "TRANSLATE" => {
            let input = pop_argument::<String>(&mut args, ident, "input");
            let pattern = pop_argument::<TranslateString>(&mut args, ident, "pattern");
            let replacement =
                pop_optional_argument::<TranslateString>(&mut args, ident, "replacement")
                    .unwrap_or_default();
            for c in input.chars() {
                if let Some(index) = pattern.find(c) {
                    if !replacement.is_empty() {
                        s.push(replacement.char(index));
                    }
                } else {
                    s.push(c);
                }
            }
        }
        "TRIM" => {
            let input = pop_argument::<String>(&mut args, ident, "input");
            let left = pop_argument::<PatternString>(&mut args, ident, "left");
            let right = pop_argument::<PatternString>(&mut args, ident, "right");
            let input = match left {
                PatternString::Range(range) => {
                    input.trim_start_matches(|c: char| range.is_match(c))
                }
                PatternString::String(string) => input.trim_start_matches(&string),
            };
            let input = match right {
                PatternString::Range(range) => input.trim_end_matches(|c: char| range.is_match(c)),
                PatternString::String(string) => input.trim_end_matches(&string),
            };
            s.push_str(input);
        }
        "CONTAINS" => {
            let input = pop_argument::<String>(&mut args, ident, "input");
            let pattern = pop_argument::<PatternString>(&mut args, ident, "pattern");
            let sucesss = match pattern {
                PatternString::Range(range) => input.contains(|c: char| range.is_match(c)),
                PatternString::String(string) => input.contains(&string),
            };
            s.push_str(if sucesss { "1" } else { "0" });
        }
        "STREQ" => {
            let a = pop_argument::<String>(&mut args, ident, "a");
            let b = pop_argument::<String>(&mut args, ident, "b");
            s.push_str(if a == b { "1" } else { "0" });
        }
        "SUBSTRING" => {
            let input = pop_argument::<String>(&mut args, ident, "input");
            let start = pop_optional_argument::<usize>(&mut args, ident, "start").unwrap_or(0);
            let end = pop_optional_argument::<usize>(&mut args, ident, "end");
            let length = pop_optional_argument::<usize>(&mut args, ident, "length");

            let end = end.or(length.map(|l| start + l)).unwrap_or(usize::MAX);
            let end = end.min(input.len());

            s.push_str(&input[start..end]);
        }
        "PAD" => {
            let input = pop_argument::<String>(&mut args, ident, "input");
            let length = pop_argument::<usize>(&mut args, ident, "length");
            let left =
                pop_optional_argument::<String>(&mut args, ident, "left").unwrap_or_default();
            let right =
                pop_optional_argument::<String>(&mut args, ident, "right").unwrap_or_default();
            if !left.is_empty() && !right.is_empty() {
                panic!("{arg}: Expected a left xor right padding, got both");
            }

            let mut len = input.len();
            if !left.is_empty() {
                while len < length {
                    s.push_str(&left);
                    len += left.len();
                }
            }
            s.push_str(&input);
            if !right.is_empty() {
                while len < length {
                    s.push_str(&right);
                    len += right.len();
                }
            }
        }
        "IF" => {
            let a = pop_argument::<isize>(&mut args, ident, "test");
            let then_tree = pop_argument::<String>(&mut args, ident, "then");
            let else_tree =
                pop_optional_argument::<String>(&mut args, ident, "else").unwrap_or_default();

            if a != 0 {
                s.push_str(&then_tree);
            } else {
                s.push_str(&else_tree);
            }
        }
        "ADD" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&(a + b).to_string());
        }
        "SUB" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&(a - b).to_string());
        }
        "MUL" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&(a * b).to_string());
        }
        "DIV" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&(a / b).to_string());
        }
        "AND" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&(a & b).to_string());
        }
        "OR" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&(a | b).to_string());
        }
        "NE" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&((a != b) as u8).to_string());
        }
        "EQ" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&((a == b) as u8).to_string());
        }
        "GE" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&((a >= b) as u8).to_string());
        }
        "GT" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&((a > b) as u8).to_string());
        }
        "LE" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&((a <= b) as u8).to_string());
        }
        "LT" => {
            let a = pop_argument::<isize>(&mut args, ident, "a");
            let b = pop_argument::<isize>(&mut args, ident, "b");
            s.push_str(&((a < b) as u8).to_string());
        }
        "RAW" => {
            let input = pop_argument::<TokenStream>(&mut args, ident, "input");
            let mut raw = String::with_capacity(32);
            parse_stream(&mut raw, ident, OutputType::RawString, input);
            s.push_str(&raw);
        }
        "TOSTRING" => {
            let input = pop_argument::<String>(&mut args, ident, "input");
            s.push_str(&input);
        }
        "TOIDENT" => {
            let input = pop_argument::<String>(&mut args, ident, "input");
            s.push_str(to_ident(&input).as_ref());
        }
        "TONUMBER" => {
            let input = pop_argument::<isize>(&mut args, ident, "input");
            s.push_str(&input.to_string());
        }
        _ => {
            panic!("Unknown function: {func}");
        }
    }
}

pub(crate) fn decode_literal_string(s: &mut String, name: &str, literal: &str) {
    if !literal.contains('\\') {
        s.push_str(literal);
    } else {
        let mut iter = literal.chars();
        while let Some(c) = iter.next() {
            if c == '\\' {
                match iter.next() {
                    Some('n') => s.push('\n'),
                    Some('r') => s.push('\r'),
                    Some('t') => s.push('\t'),
                    Some('\\') => s.push('\\'),
                    Some('"') => s.push('"'),
                    Some('\'') => s.push('\''),
                    Some('0') => s.push('\0'),
                    Some('x') => {
                        let Some(c) = iter.next() else {
                            panic!("{}: Expected a hexadecimal character", name);
                        };
                        let Some(c2) = iter.next() else {
                            panic!("{}: Expected a hexadecimal character", name);
                        };
                        let Ok(c) = format!("{}{}", c, c2).parse::<u8>() else {
                            panic!("{}: Expected a hexadecimal character", name);
                        };
                        s.push(char::from(c));
                    }
                    Some(_) => panic!("{name}: Expected a valid escape sequence (got {c:?})"),
                    None => break,
                }
            } else {
                s.push(c);
            }
        }
    }
}

fn to_ident(input: &str) -> Cow<'_, str> {
    // Idents must be XID_Start + XID_Continue*, but we just use
    // is_alphabetic and is_alphanumeric as an approximation.
    let mut s = input;
    while !s.starts_with(|c: char| c.is_alphabetic() || c == '_') {
        if s.is_empty() {
            panic!("No valid ident characters found in `{input}`");
        }
        s = &s[1..];
    }

    // If we detect any invalid chars, filter them one-by-one.
    if s[1..].contains(|c: char| !c.is_alphanumeric() && c != '_') {
        let mut output = String::with_capacity(s.len());
        let mut chars = s.chars();
        // We know there's always at least one
        output.push(chars.next().unwrap());
        for c in chars {
            if c.is_alphanumeric() || c == '_' {
                output.push(c);
            }
        }
        Cow::Owned(output)
    } else {
        // Fast path: all valid chars are alphanumeric or underscore.
        Cow::Borrowed(s)
    }
}
