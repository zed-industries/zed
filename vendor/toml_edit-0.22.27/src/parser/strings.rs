use std::borrow::Cow;
use std::char;
use std::ops::RangeInclusive;

use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::empty;
use winnow::combinator::fail;
use winnow::combinator::opt;
use winnow::combinator::peek;
use winnow::combinator::preceded;
use winnow::combinator::repeat;
use winnow::combinator::terminated;
use winnow::combinator::trace;
use winnow::prelude::*;
use winnow::stream::Stream;
use winnow::token::any;
use winnow::token::none_of;
use winnow::token::one_of;
use winnow::token::take_while;

use crate::parser::error::CustomError;
use crate::parser::numbers::HEXDIG;
use crate::parser::prelude::*;
use crate::parser::trivia::{from_utf8_unchecked, newline, ws, ws_newlines, NON_ASCII, WSCHAR};

// ;; String

// string = ml-basic-string / basic-string / ml-literal-string / literal-string
pub(crate) fn string<'i>(input: &mut Input<'i>) -> ModalResult<Cow<'i, str>> {
    trace(
        "string",
        alt((
            ml_basic_string,
            basic_string,
            ml_literal_string,
            literal_string.map(Cow::Borrowed),
        )),
    )
    .parse_next(input)
}

// ;; Basic String

// basic-string = quotation-mark *basic-char quotation-mark
pub(crate) fn basic_string<'i>(input: &mut Input<'i>) -> ModalResult<Cow<'i, str>> {
    trace("basic-string", |input: &mut Input<'i>| {
        let _ = one_of(QUOTATION_MARK).parse_next(input)?;

        let mut c = Cow::Borrowed("");
        if let Some(ci) = opt(basic_chars).parse_next(input)? {
            c = ci;
        }
        while let Some(ci) = opt(basic_chars).parse_next(input)? {
            c.to_mut().push_str(&ci);
        }

        let _ = cut_err(one_of(QUOTATION_MARK))
            .context(StrContext::Label("basic string"))
            .parse_next(input)?;

        Ok(c)
    })
    .parse_next(input)
}

// quotation-mark = %x22            ; "
pub(crate) const QUOTATION_MARK: u8 = b'"';

// basic-char = basic-unescaped / escaped
fn basic_chars<'i>(input: &mut Input<'i>) -> ModalResult<Cow<'i, str>> {
    alt((
        // Deviate from the official grammar by batching the unescaped chars so we build a string a
        // chunk at a time, rather than a `char` at a time.
        take_while(1.., BASIC_UNESCAPED)
            .try_map(std::str::from_utf8)
            .map(Cow::Borrowed),
        escaped.map(|c| Cow::Owned(String::from(c))),
    ))
    .parse_next(input)
}

// basic-unescaped = wschar / %x21 / %x23-5B / %x5D-7E / non-ascii
const BASIC_UNESCAPED: (
    (u8, u8),
    u8,
    RangeInclusive<u8>,
    RangeInclusive<u8>,
    RangeInclusive<u8>,
) = (WSCHAR, 0x21, 0x23..=0x5B, 0x5D..=0x7E, NON_ASCII);

// escaped = escape escape-seq-char
fn escaped(input: &mut Input<'_>) -> ModalResult<char> {
    preceded(ESCAPE, escape_seq_char).parse_next(input)
}

// escape = %x5C                    ; \
const ESCAPE: u8 = b'\\';

// escape-seq-char =  %x22         ; "    quotation mark  U+0022
// escape-seq-char =/ %x5C         ; \    reverse solidus U+005C
// escape-seq-char =/ %x62         ; b    backspace       U+0008
// escape-seq-char =/ %x66         ; f    form feed       U+000C
// escape-seq-char =/ %x6E         ; n    line feed       U+000A
// escape-seq-char =/ %x72         ; r    carriage return U+000D
// escape-seq-char =/ %x74         ; t    tab             U+0009
// escape-seq-char =/ %x75 4HEXDIG ; uXXXX                U+XXXX
// escape-seq-char =/ %x55 8HEXDIG ; UXXXXXXXX            U+XXXXXXXX
fn escape_seq_char(input: &mut Input<'_>) -> ModalResult<char> {
    dispatch! {any;
        b'b' => empty.value('\u{8}'),
        b'f' => empty.value('\u{c}'),
        b'n' => empty.value('\n'),
        b'r' => empty.value('\r'),
        b't' => empty.value('\t'),
        b'u' => cut_err(hexescape::<4>).context(StrContext::Label("unicode 4-digit hex code")),
        b'U' => cut_err(hexescape::<8>).context(StrContext::Label("unicode 8-digit hex code")),
        b'\\' => empty.value('\\'),
        b'"' => empty.value('"'),
        _ => {
            cut_err(fail::<_, char, _>)
            .context(StrContext::Label("escape sequence"))
            .context(StrContext::Expected(StrContextValue::CharLiteral('b')))
            .context(StrContext::Expected(StrContextValue::CharLiteral('f')))
            .context(StrContext::Expected(StrContextValue::CharLiteral('n')))
            .context(StrContext::Expected(StrContextValue::CharLiteral('r')))
            .context(StrContext::Expected(StrContextValue::CharLiteral('t')))
            .context(StrContext::Expected(StrContextValue::CharLiteral('u')))
            .context(StrContext::Expected(StrContextValue::CharLiteral('U')))
            .context(StrContext::Expected(StrContextValue::CharLiteral('\\')))
            .context(StrContext::Expected(StrContextValue::CharLiteral('"')))
        }
    }
    .parse_next(input)
}

fn hexescape<const N: usize>(input: &mut Input<'_>) -> ModalResult<char> {
    take_while(0..=N, HEXDIG)
        .verify(|b: &[u8]| b.len() == N)
        .map(|b: &[u8]| unsafe { from_utf8_unchecked(b, "`is_ascii_digit` filters out on-ASCII") })
        .verify_map(|s| u32::from_str_radix(s, 16).ok())
        .try_map(|h| char::from_u32(h).ok_or(CustomError::OutOfRange))
        .parse_next(input)
}

// ;; Multiline Basic String

// ml-basic-string = ml-basic-string-delim [ newline ] ml-basic-body
//                   ml-basic-string-delim
fn ml_basic_string<'i>(input: &mut Input<'i>) -> ModalResult<Cow<'i, str>> {
    trace(
        "ml-basic-string",
        delimited(
            ML_BASIC_STRING_DELIM,
            preceded(opt(newline), cut_err(ml_basic_body))
                .context(StrContext::Label("multiline basic string")),
            cut_err(ML_BASIC_STRING_DELIM).context(StrContext::Label("multiline basic string")),
        ),
    )
    .parse_next(input)
}

// ml-basic-string-delim = 3quotation-mark
const ML_BASIC_STRING_DELIM: &[u8] = b"\"\"\"";

// ml-basic-body = *mlb-content *( mlb-quotes 1*mlb-content ) [ mlb-quotes ]
fn ml_basic_body<'i>(input: &mut Input<'i>) -> ModalResult<Cow<'i, str>> {
    let mut c = Cow::Borrowed("");
    if let Some(ci) = opt(mlb_content).parse_next(input)? {
        c = ci;
    }
    while let Some(ci) = opt(mlb_content).parse_next(input)? {
        c.to_mut().push_str(&ci);
    }

    while let Some(qi) = opt(mlb_quotes(none_of(b'\"').value(()))).parse_next(input)? {
        if let Some(ci) = opt(mlb_content).parse_next(input)? {
            c.to_mut().push_str(qi);
            c.to_mut().push_str(&ci);
            while let Some(ci) = opt(mlb_content).parse_next(input)? {
                c.to_mut().push_str(&ci);
            }
        } else {
            break;
        }
    }

    if let Some(qi) = opt(mlb_quotes(ML_BASIC_STRING_DELIM.void())).parse_next(input)? {
        c.to_mut().push_str(qi);
    }

    Ok(c)
}

// mlb-content = mlb-char / newline / mlb-escaped-nl
// mlb-char = mlb-unescaped / escaped
fn mlb_content<'i>(input: &mut Input<'i>) -> ModalResult<Cow<'i, str>> {
    alt((
        // Deviate from the official grammar by batching the unescaped chars so we build a string a
        // chunk at a time, rather than a `char` at a time.
        take_while(1.., MLB_UNESCAPED)
            .try_map(std::str::from_utf8)
            .map(Cow::Borrowed),
        // Order changed fromg grammar so `escaped` can more easily `cut_err` on bad escape sequences
        mlb_escaped_nl.map(|_| Cow::Borrowed("")),
        escaped.map(|c| Cow::Owned(String::from(c))),
        newline.map(|_| Cow::Borrowed("\n")),
    ))
    .parse_next(input)
}

// mlb-quotes = 1*2quotation-mark
fn mlb_quotes<'i>(
    mut term: impl ModalParser<Input<'i>, (), ContextError>,
) -> impl ModalParser<Input<'i>, &'i str, ContextError> {
    move |input: &mut Input<'i>| {
        let start = input.checkpoint();
        let res = terminated(b"\"\"", peek(term.by_ref()))
            .map(|b| unsafe { from_utf8_unchecked(b, "`bytes` out non-ASCII") })
            .parse_next(input);

        match res {
            Err(winnow::error::ErrMode::Backtrack(_)) => {
                input.reset(&start);
                terminated(b"\"", peek(term.by_ref()))
                    .map(|b| unsafe { from_utf8_unchecked(b, "`bytes` out non-ASCII") })
                    .parse_next(input)
            }
            res => res,
        }
    }
}

// mlb-unescaped = wschar / %x21 / %x23-5B / %x5D-7E / non-ascii
const MLB_UNESCAPED: (
    (u8, u8),
    u8,
    RangeInclusive<u8>,
    RangeInclusive<u8>,
    RangeInclusive<u8>,
) = (WSCHAR, 0x21, 0x23..=0x5B, 0x5D..=0x7E, NON_ASCII);

// mlb-escaped-nl = escape ws newline *( wschar / newline
// When the last non-whitespace character on a line is a \,
// it will be trimmed along with all whitespace
// (including newlines) up to the next non-whitespace
// character or closing delimiter.
fn mlb_escaped_nl(input: &mut Input<'_>) -> ModalResult<()> {
    repeat(1.., (ESCAPE, ws, ws_newlines))
        .map(|()| ())
        .value(())
        .parse_next(input)
}

// ;; Literal String

// literal-string = apostrophe *literal-char apostrophe
pub(crate) fn literal_string<'i>(input: &mut Input<'i>) -> ModalResult<&'i str> {
    trace(
        "literal-string",
        delimited(
            APOSTROPHE,
            cut_err(take_while(0.., LITERAL_CHAR)),
            cut_err(APOSTROPHE),
        )
        .try_map(std::str::from_utf8)
        .context(StrContext::Label("literal string")),
    )
    .parse_next(input)
}

// apostrophe = %x27 ; ' apostrophe
pub(crate) const APOSTROPHE: u8 = b'\'';

// literal-char = %x09 / %x20-26 / %x28-7E / non-ascii
const LITERAL_CHAR: (
    u8,
    RangeInclusive<u8>,
    RangeInclusive<u8>,
    RangeInclusive<u8>,
) = (0x9, 0x20..=0x26, 0x28..=0x7E, NON_ASCII);

// ;; Multiline Literal String

// ml-literal-string = ml-literal-string-delim [ newline ] ml-literal-body
//                     ml-literal-string-delim
fn ml_literal_string<'i>(input: &mut Input<'i>) -> ModalResult<Cow<'i, str>> {
    trace(
        "ml-literal-string",
        delimited(
            (ML_LITERAL_STRING_DELIM, opt(newline)),
            cut_err(ml_literal_body.map(|t| {
                if t.contains("\r\n") {
                    Cow::Owned(t.replace("\r\n", "\n"))
                } else {
                    Cow::Borrowed(t)
                }
            }))
            .context(StrContext::Label("multiline literal string")),
            cut_err(ML_LITERAL_STRING_DELIM).context(StrContext::Label("multiline literal string")),
        ),
    )
    .parse_next(input)
}

// ml-literal-string-delim = 3apostrophe
const ML_LITERAL_STRING_DELIM: &[u8] = b"'''";

// ml-literal-body = *mll-content *( mll-quotes 1*mll-content ) [ mll-quotes ]
fn ml_literal_body<'i>(input: &mut Input<'i>) -> ModalResult<&'i str> {
    (
        repeat(0.., mll_content).map(|()| ()),
        repeat(
            0..,
            (
                mll_quotes(none_of(APOSTROPHE).value(())),
                repeat(1.., mll_content).map(|()| ()),
            ),
        )
        .map(|()| ()),
        opt(mll_quotes(ML_LITERAL_STRING_DELIM.void())),
    )
        .take()
        .try_map(std::str::from_utf8)
        .parse_next(input)
}

// mll-content = mll-char / newline
fn mll_content(input: &mut Input<'_>) -> ModalResult<u8> {
    alt((one_of(MLL_CHAR), newline.value(b'\n'))).parse_next(input)
}

// mll-char = %x09 / %x20-26 / %x28-7E / non-ascii
const MLL_CHAR: (
    u8,
    RangeInclusive<u8>,
    RangeInclusive<u8>,
    RangeInclusive<u8>,
) = (0x9, 0x20..=0x26, 0x28..=0x7E, NON_ASCII);

// mll-quotes = 1*2apostrophe
fn mll_quotes<'i>(
    mut term: impl ModalParser<Input<'i>, (), ContextError>,
) -> impl ModalParser<Input<'i>, &'i str, ContextError> {
    move |input: &mut Input<'i>| {
        let start = input.checkpoint();
        let res = terminated(b"''", peek(term.by_ref()))
            .map(|b| unsafe { from_utf8_unchecked(b, "`bytes` out non-ASCII") })
            .parse_next(input);

        match res {
            Err(winnow::error::ErrMode::Backtrack(_)) => {
                input.reset(&start);
                terminated(b"'", peek(term.by_ref()))
                    .map(|b| unsafe { from_utf8_unchecked(b, "`bytes` out non-ASCII") })
                    .parse_next(input)
            }
            res => res,
        }
    }
}

#[cfg(test)]
#[cfg(feature = "parse")]
#[cfg(feature = "display")]
mod test {
    use super::*;

    #[test]
    fn basic_string() {
        let input =
            r#""I'm a string. \"You can quote me\". Name\tJos\u00E9\nLocation\tSF. \U0002070E""#;
        let expected = "I\'m a string. \"You can quote me\". Name\tJosé\nLocation\tSF. \u{2070E}";
        let parsed = string.parse(new_input(input));
        assert_eq!(parsed.as_deref(), Ok(expected), "Parsing {input:?}");
    }

    #[test]
    fn ml_basic_string() {
        let cases = [
            (
                r#""""
Roses are red
Violets are blue""""#,
                r#"Roses are red
Violets are blue"#,
            ),
            (r#"""" \""" """"#, " \"\"\" "),
            (r#"""" \\""""#, " \\"),
        ];

        for &(input, expected) in &cases {
            let parsed = string.parse(new_input(input));
            assert_eq!(parsed.as_deref(), Ok(expected), "Parsing {input:?}");
        }

        let invalid_cases = [r#""""  """#, r#""""  \""""#];

        for input in &invalid_cases {
            let parsed = string.parse(new_input(input));
            assert!(parsed.is_err());
        }
    }

    #[test]
    fn ml_basic_string_escape_ws() {
        let inputs = [
            r#""""
The quick brown \


  fox jumps over \
    the lazy dog.""""#,
            r#""""\
       The quick brown \
       fox jumps over \
       the lazy dog.\
       """"#,
        ];
        for input in &inputs {
            let expected = "The quick brown fox jumps over the lazy dog.";
            let parsed = string.parse(new_input(input));
            assert_eq!(parsed.as_deref(), Ok(expected), "Parsing {input:?}");
        }
        let empties = [
            r#""""\
       """"#,
            r#""""
\
  \
""""#,
        ];
        for input in &empties {
            let expected = "";
            let parsed = string.parse(new_input(input));
            assert_eq!(parsed.as_deref(), Ok(expected), "Parsing {input:?}");
        }
    }

    #[test]
    fn literal_string() {
        let inputs = [
            r"'C:\Users\nodejs\templates'",
            r"'\\ServerX\admin$\system32\'",
            r#"'Tom "Dubs" Preston-Werner'"#,
            r"'<\i\c*\s*>'",
        ];

        for input in &inputs {
            let expected = &input[1..input.len() - 1];
            let parsed = string.parse(new_input(input));
            assert_eq!(parsed.as_deref(), Ok(expected), "Parsing {input:?}");
        }
    }

    #[test]
    fn ml_literal_string() {
        let inputs = [
            r"'''I [dw]on't need \d{2} apples'''",
            r#"''''one_quote''''"#,
        ];
        for input in &inputs {
            let expected = &input[3..input.len() - 3];
            let parsed = string.parse(new_input(input));
            assert_eq!(parsed.as_deref(), Ok(expected), "Parsing {input:?}");
        }

        let input = r#"'''
The first newline is
trimmed in raw strings.
   All other whitespace
   is preserved.
'''"#;
        let expected = &input[4..input.len() - 3];
        let parsed = string.parse(new_input(input));
        assert_eq!(parsed.as_deref(), Ok(expected), "Parsing {input:?}");
    }
}
