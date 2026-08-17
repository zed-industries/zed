use super::{FmtArg, FmtStrComponent, FormatStr, ParseError, ParseErrorKind, WhichArg};

use crate::{
    formatting::{FormattingFlags, IsAlternate, NumberFormatting},
    parse_utils::StrRawness,
};

#[cfg(test)]
impl FmtStrComponent {
    pub(super) fn str(s: &str) -> Self {
        Self::Str(s.to_string(), StrRawness::dummy())
    }
    pub(super) fn arg(which_arg: WhichArg, formatting: FormattingFlags) -> Self {
        Self::Arg(FmtArg {
            which_arg,
            formatting,
            rawness: StrRawness::dummy(),
        })
    }
}

impl FmtArg {
    fn new(which_arg: WhichArg, formatting: FormattingFlags, rawness: StrRawness) -> Self {
        Self {
            which_arg,
            formatting,
            rawness,
        }
    }
}

#[allow(dead_code)]
impl WhichArg {
    pub(super) fn ident(s: &str) -> Self {
        Self::Ident(s.to_string())
    }
}

/////////////////////////////////////

#[cfg(test)]
impl std::str::FromStr for FormatStr {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<FormatStr, ParseError> {
        parse_format_str(input, StrRawness::dummy())
    }
}

impl FormatStr {
    pub fn parse(input: &str, rawness: StrRawness) -> Result<FormatStr, ParseError> {
        parse_format_str(input, rawness)
    }
}

fn parse_format_str(input: &str, rawness: StrRawness) -> Result<FormatStr, ParseError> {
    let mut components = Vec::<FmtStrComponent>::new();

    let mut arg_start = 0;

    loop {
        let open_pos = input.find_from('{', arg_start);

        let str = &input[arg_start..open_pos.unwrap_or(input.len())];
        components.push_arg_str(parse_mid_str(str, arg_start)?, rawness);

        if let Some(open_pos) = open_pos {
            let after_open = open_pos + 1;
            if input[after_open..].starts_with('{') {
                components.push_arg_str("{".to_string(), rawness);

                arg_start = open_pos + 2;
            } else if let Some(close_pos) = input.find_from('}', after_open) {
                let after_close = close_pos + 1;

                let arg = parse_fmt_arg(&input[after_open..close_pos], after_open, rawness)?;
                components.push(FmtStrComponent::Arg(arg));

                arg_start = after_close;
            } else {
                return Err(ParseError {
                    pos: open_pos,
                    kind: ParseErrorKind::UnclosedArg,
                });
            }
        } else {
            break;
        }
    }

    Ok(FormatStr { list: components })
}

/// Parses the text between arguments, to unescape `}}` into `}`
fn parse_mid_str(str: &str, starts_at: usize) -> Result<String, ParseError> {
    let mut buffer = String::with_capacity(str.len());

    let mut starts_pos = 0;
    let bytes = str.as_bytes();

    while let Some(close_pos) = str.find_from('}', starts_pos) {
        let after_close = close_pos + 1;
        if bytes.get(after_close) == Some(&b'}') {
            buffer.push_str(&str[starts_pos..after_close]);
            starts_pos = after_close + 1;
        } else {
            return Err(ParseError {
                pos: starts_at + close_pos,
                kind: ParseErrorKind::InvalidClosedArg,
            });
        }
    }
    buffer.push_str(&str[starts_pos..]);

    Ok(buffer)
}

/// Parses the format arguments (`{:?}`, `{foo:}`, `{0}`, etc).
///
/// `starts_at` is the offset of `input` in the formatting string.
fn parse_fmt_arg(input: &str, starts_at: usize, rawness: StrRawness) -> Result<FmtArg, ParseError> {
    let colon = input.find(':');

    let which_arg_str = &input[..colon.unwrap_or(input.len())];
    let formatting_str = colon.map_or("", |x| &input[x + 1..]);
    let formatting_starts_at = colon.map_or(input.len(), |x| starts_at + x + 1);

    Ok(FmtArg::new(
        parse_which_arg(which_arg_str, starts_at)?,
        parse_formatting(formatting_str, formatting_starts_at)?,
        rawness,
    ))
}

/// Parses the name of the argument in `{foo}`, `{}`, `{bar:?}`
///
/// `starts_at` is the offset of `input` in the formatting string.
fn parse_which_arg(input: &str, starts_at: usize) -> Result<WhichArg, ParseError> {
    if input.is_empty() {
        Ok(WhichArg::Positional(None))
    } else if input.as_bytes()[0].is_ascii_digit() {
        match input.parse::<usize>() {
            Ok(number) => Ok(WhichArg::Positional(Some(number))),
            Err(_) => Err(ParseError {
                pos: starts_at,
                kind: ParseErrorKind::NotANumber {
                    what: input.to_string(),
                },
            }),
        }
    } else {
        parse_ident(input, starts_at)
    }
}

/// Parses the `?` and other formatters inside formatting arguments (`{}`).
///
/// `starts_at` is the offset of `input` in the formatting string.
fn parse_formatting(input: &str, starts_at: usize) -> Result<FormattingFlags, ParseError> {
    match input {
        "#" => return Ok(FormattingFlags::display(IsAlternate::Yes)),
        "" => return Ok(FormattingFlags::display(IsAlternate::No)),
        _ => {}
    }

    let mut bytes = input.as_bytes();

    let make_error = || ParseError {
        pos: starts_at,
        kind: ParseErrorKind::UnknownFormatting {
            what: input.to_string(),
        },
    };

    if let [before @ .., b'?'] = bytes {
        bytes = before;
    }

    let mut num_fmt = NumberFormatting::Decimal;
    let mut is_alternate = IsAlternate::No;

    for byte in bytes {
        match byte {
            b'b' if num_fmt.is_regular() => num_fmt = NumberFormatting::Binary,
            b'x' if num_fmt.is_regular() => num_fmt = NumberFormatting::LowerHexadecimal,
            b'X' if num_fmt.is_regular() => num_fmt = NumberFormatting::Hexadecimal,
            b'#' => is_alternate = IsAlternate::Yes,
            _ => return Err(make_error()),
        }
    }
    Ok(FormattingFlags::debug(num_fmt, is_alternate))
}

/// Parses an identifier in a formatting argument.
///
/// `starts_at` is the offset of `input` in the formatting string.
fn parse_ident(ident_str: &str, starts_at: usize) -> Result<WhichArg, ParseError> {
    if is_ident(ident_str) {
        Ok(WhichArg::Ident(ident_str.to_string()))
    } else {
        Err(ParseError {
            pos: starts_at,
            kind: ParseErrorKind::NotAnIdent {
                what: ident_str.to_string(),
            },
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

fn is_ident(s: &str) -> bool {
    use unicode_xid::UnicodeXID;

    if s.is_empty() || s == "_" {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    // For some reason '_' is not considered a valid character for the stard of an ident
    (first.is_xid_start() || first == '_') && chars.all(|c| c.is_xid_continue())
}

////////////////////////////////////////////////////////////////////////////////

trait VecExt {
    fn push_arg_str(&mut self, str: String, rawness: StrRawness);
}

impl VecExt for Vec<FmtStrComponent> {
    fn push_arg_str(&mut self, str: String, rawness: StrRawness) {
        if !str.is_empty() {
            self.push(FmtStrComponent::Str(str, rawness));
        }
    }
}

trait StrExt {
    fn find_from(&self, c: char, from: usize) -> Option<usize>;
}

impl StrExt for str {
    fn find_from(&self, c: char, from: usize) -> Option<usize> {
        self[from..].find(c).map(|p| p + from)
    }
}
