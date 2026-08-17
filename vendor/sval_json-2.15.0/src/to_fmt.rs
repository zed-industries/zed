use core::fmt::{self, Write};

use sval::Stream as _;

use crate::{tags, Error};

macro_rules! _try {
    ($e:expr) => {
        match ($e) {
            Ok(_o) => _o,
            Err(_) => return Err(sval::Error::new()),
        }
    };
}

macro_rules! _try_no_conv {
    ($e:expr) => {
        match ($e) {
            Ok(()) => (),
            Err(e) => return Err(e),
        }
    };
}

/**
Stream a value as JSON to an underlying formatter.
*/
pub fn stream_to_fmt_write(fmt: impl Write, v: impl sval::Value) -> Result<(), Error> {
    let mut stream = Formatter::new(fmt);

    match v.stream(&mut stream) {
        Ok(()) => Ok(()),
        Err(_) => Err(stream.err.unwrap_or_else(Error::generic)),
    }
}

pub(crate) struct Formatter<W> {
    is_internally_tagged: bool,
    is_current_depth_empty: bool,
    is_text_quoted: bool,
    text_handler: Option<TextHandler>,
    err: Option<Error>,
    out: W,
}

impl<W> Formatter<W> {
    pub fn new(out: W) -> Self {
        Formatter {
            is_internally_tagged: false,
            is_current_depth_empty: true,
            is_text_quoted: true,
            text_handler: None,
            err: None,
            out,
        }
    }

    fn err(&mut self, e: Error) -> sval::Error {
        self.err = Some(e);
        sval::Error::new()
    }
}

impl<W> fmt::Debug for Formatter<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Formatter")
            .field("is_internally_tagged", &self.is_internally_tagged)
            .field("is_current_depth_empty", &self.is_current_depth_empty)
            .field("is_text_quoted", &self.is_text_quoted)
            .field("err", &self.err)
            .field("text_handler", &self.text_handler.as_ref().map(|_| ()))
            .finish()
    }
}

impl<'sval, W> sval::Stream<'sval> for Formatter<W>
where
    W: Write,
{
    fn null(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(_try!(self.out.write_str("null")))
    }

    fn bool(&mut self, v: bool) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(_try!(self.out.write_str(if v { "true" } else { "false" })))
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.is_current_depth_empty = false;

        if self.is_text_quoted {
            _try!(self.out.write_char('"'));
        }

        Ok(())
    }

    fn text_fragment_computed(&mut self, v: &str) -> sval::Result {
        match self.text_handler {
            None => _try!(escape_str(v, &mut self.out)),
            Some(ref mut handler) => _try!(handler.text_fragment(v, &mut self.out)),
        }

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        if self.is_text_quoted {
            _try!(self.out.write_char('"'));
        }

        Ok(())
    }

    fn u8(&mut self, v: u8) -> sval::Result {
        self.is_current_depth_empty = false;

        _try!(self.out.write_str(itoa::Buffer::new().format(v)));

        Ok(())
    }

    fn u16(&mut self, v: u16) -> sval::Result {
        self.is_current_depth_empty = false;

        _try!(self.out.write_str(itoa::Buffer::new().format(v)));

        Ok(())
    }

    fn u32(&mut self, v: u32) -> sval::Result {
        self.is_current_depth_empty = false;

        _try!(self.out.write_str(itoa::Buffer::new().format(v)));

        Ok(())
    }

    fn u64(&mut self, v: u64) -> sval::Result {
        self.is_current_depth_empty = false;

        _try!(self.out.write_str(itoa::Buffer::new().format(v)));

        Ok(())
    }

    fn u128(&mut self, v: u128) -> sval::Result {
        self.is_current_depth_empty = false;

        _try!(self.out.write_str(itoa::Buffer::new().format(v)));

        Ok(())
    }

    fn i8(&mut self, v: i8) -> sval::Result {
        self.is_current_depth_empty = false;

        _try!(self.out.write_str(itoa::Buffer::new().format(v)));

        Ok(())
    }

    fn i16(&mut self, v: i16) -> sval::Result {
        self.is_current_depth_empty = false;

        _try!(self.out.write_str(itoa::Buffer::new().format(v)));

        Ok(())
    }

    fn i32(&mut self, v: i32) -> sval::Result {
        self.is_current_depth_empty = false;

        _try!(self.out.write_str(itoa::Buffer::new().format(v)));

        Ok(())
    }

    fn i64(&mut self, v: i64) -> sval::Result {
        self.is_current_depth_empty = false;

        _try!(self.out.write_str(itoa::Buffer::new().format(v)));

        Ok(())
    }

    fn i128(&mut self, v: i128) -> sval::Result {
        self.is_current_depth_empty = false;

        _try!(self.out.write_str(itoa::Buffer::new().format(v)));

        Ok(())
    }

    fn f32(&mut self, v: f32) -> sval::Result {
        self.is_current_depth_empty = false;

        if v.is_nan() || v.is_infinite() {
            self.null()
        } else {
            _try!(self.out.write_str(ryu::Buffer::new().format_finite(v)));

            Ok(())
        }
    }

    fn f64(&mut self, v: f64) -> sval::Result {
        self.is_current_depth_empty = false;

        if v.is_nan() || v.is_infinite() {
            self.null()
        } else {
            _try!(self.out.write_str(ryu::Buffer::new().format_finite(v)));

            Ok(())
        }
    }

    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        if !self.is_text_quoted {
            return Err(self.err(Error::invalid_key()));
        }

        self.is_current_depth_empty = true;

        _try!(self.out.write_char('{'));

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.is_text_quoted = false;
        self.is_internally_tagged = false;

        if !self.is_current_depth_empty {
            _try!(self.out.write_str(",\""));
        } else {
            _try!(self.out.write_char('"'));
        }

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        _try!(self.out.write_str("\":"));

        self.is_text_quoted = true;

        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;
        _try!(self.out.write_char('}'));

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        if !self.is_text_quoted {
            return Err(self.err(Error::invalid_key()));
        }

        self.is_current_depth_empty = true;

        _try!(self.out.write_char('['));

        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.is_internally_tagged = false;

        if !self.is_current_depth_empty {
            _try!(self.out.write_char(','));
        }

        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;
        _try!(self.out.write_char(']'));

        Ok(())
    }

    fn enum_begin(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        _try_no_conv!(self.internally_tagged_begin(label, index));

        self.is_internally_tagged = true;
        self.is_current_depth_empty = true;

        Ok(())
    }

    fn enum_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        if self.is_current_depth_empty {
            _try_no_conv!(self.tag(tag, label, index));
        }

        if self.is_internally_tagged {
            self.internally_tagged_map_end()
        } else {
            self.internally_tagged_end(label, index)
        }
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        match tag {
            Some(&tags::JSON_TEXT) => {
                self.text_handler = Some(TextHandler::native());
            }
            Some(&tags::JSON_VALUE) => {
                self.is_text_quoted = false;
                self.text_handler = Some(TextHandler::native());
            }
            Some(&sval::tags::NUMBER) => {
                self.is_text_quoted = false;

                if self.text_handler.is_none() {
                    self.text_handler = Some(TextHandler::number());
                }
            }
            Some(&tags::JSON_NUMBER) => {
                self.is_text_quoted = false;
                self.text_handler = Some(TextHandler::native());
            }
            _ => (),
        }

        _try_no_conv!(self.internally_tagged_begin(label, index));
        self.is_current_depth_empty = true;

        Ok(())
    }

    fn tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        match tag {
            Some(&tags::JSON_TEXT) => {
                self.text_handler = None;
            }
            Some(&tags::JSON_VALUE) => {
                self.is_text_quoted = true;
                self.text_handler = None;
            }
            Some(&sval::tags::NUMBER) => {
                self.is_text_quoted = true;

                if let Some(TextHandler::Number(mut number)) = self.text_handler.take() {
                    _try!(number.end(&mut self.out));
                }
            }
            Some(&tags::JSON_NUMBER) => {
                self.is_text_quoted = true;
                self.text_handler = None;
            }
            _ => (),
        }

        if self.is_current_depth_empty {
            _try_no_conv!(self.tag(tag, label, index));
        }

        self.internally_tagged_end(label, index)
    }

    fn tag(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.is_internally_tagged = false;
        self.is_current_depth_empty = false;

        match tag {
            Some(&sval::tags::RUST_OPTION_NONE) => self.null(),
            _ => {
                if let Some(label) = label {
                    self.value(label.as_str())
                } else if let Some(index) = index.and_then(|ix| ix.to_i64()) {
                    self.value(&index)
                } else {
                    self.null()
                }
            }
        }
    }

    fn record_begin(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        _try_no_conv!(self.internally_tagged_begin(label, index));
        self.map_begin(num_entries_hint)
    }

    fn record_value_begin(&mut self, _: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
        self.is_internally_tagged = false;

        if !self.is_current_depth_empty {
            _try!(self.out.write_str(",\""));
        } else {
            _try!(self.out.write_char('"'));
        }

        // If the label is a Rust identifier then it doesn't need escaping as JSON
        if let Some(&sval::tags::VALUE_IDENT) = label.tag() {
            _try!(self.out.write_str(label.as_str()));
        } else {
            _try!(escape_str(label.as_str(), &mut self.out));
        }

        _try!(self.out.write_str("\":"));

        self.map_value_begin()
    }

    fn record_end(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        _try_no_conv!(self.map_end());
        self.internally_tagged_end(label, index)
    }

    fn tuple_begin(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        _try_no_conv!(self.internally_tagged_begin(label, index));
        self.seq_begin(num_entries_hint)
    }

    fn tuple_end(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        _try_no_conv!(self.seq_end());
        self.internally_tagged_end(label, index)
    }
}

impl<'sval, W> Formatter<W>
where
    W: Write,
{
    fn internally_tagged_begin(
        &mut self,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        // If there's a label then begin a map, using the label as the key
        if self.is_internally_tagged {
            self.is_internally_tagged = false;

            if let Some(label) = label {
                return self.internally_tagged_map_begin_label(label.as_str());
            } else if let Some(index) = index.and_then(|index| index.to_i64()) {
                return self.internally_tagged_map_begin_index(index);
            }
        }

        Ok(())
    }

    fn internally_tagged_end(
        &mut self,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.is_internally_tagged =
            label.is_some() || index.and_then(|index| index.to_i64()).is_some();

        Ok(())
    }

    fn internally_tagged_map_begin_label(&mut self, label: &str) -> sval::Result {
        _try_no_conv!(self.map_begin(Some(1)));

        _try_no_conv!(self.map_key_begin());
        _try!(escape_str(label, &mut self.out));
        _try_no_conv!(self.map_key_end());

        self.map_value_begin()
    }

    fn internally_tagged_map_begin_index(&mut self, index: i64) -> sval::Result {
        _try_no_conv!(self.map_begin(Some(1)));

        _try_no_conv!(self.map_key_begin());
        _try_no_conv!(self.i64(index));
        _try_no_conv!(self.map_key_end());

        self.map_value_begin()
    }

    fn internally_tagged_map_end(&mut self) -> sval::Result {
        _try_no_conv!(self.map_value_end());
        self.map_end()
    }
}

enum TextHandler {
    Native,
    Number(NumberTextHandler),
}

struct NumberTextHandler {
    at_start: bool,
    sign_negative: bool,
    leading_zeroes: usize,
    is_nan_or_infinity: bool,
}

impl TextHandler {
    const fn native() -> Self {
        TextHandler::Native
    }

    const fn number() -> Self {
        TextHandler::Number(NumberTextHandler {
            sign_negative: false,
            leading_zeroes: 0,
            at_start: true,
            is_nan_or_infinity: false,
        })
    }

    fn text_fragment(&mut self, v: &str, mut out: impl Write) -> fmt::Result {
        match self {
            TextHandler::Native => out.write_str(v),
            TextHandler::Number(number) => number.text_fragment(v, out),
        }
    }
}

impl NumberTextHandler {
    fn text_fragment(&mut self, v: &str, mut out: impl Write) -> fmt::Result {
        if !self.is_nan_or_infinity {
            let mut range = 0..0;

            for b in v.as_bytes() {
                match b {
                    // JSON numbers don't support leading zeroes (except for `0.x`)
                    // so we need to shift over them
                    b'0' if self.at_start => {
                        self.leading_zeroes += 1;
                        range.start += 1;
                        range.end += 1;
                    }
                    // If we're not skipping zeroes then shift over it to write later
                    b'0'..=b'9' => {
                        if self.at_start && self.sign_negative {
                            _try_no_conv!(out.write_char('-'));
                        }

                        self.at_start = false;
                        range.end += 1;
                    }
                    // If we encounter a decimal point we might need to write a leading `0`
                    b'.' => {
                        if self.at_start {
                            if self.sign_negative {
                                _try_no_conv!(out.write_char('-'));
                            }

                            _try_no_conv!(out.write_char('0'));
                        }

                        self.at_start = false;
                        range.end += 1;
                    }
                    // If we encounter a sign then stash it until we know the number is finite
                    // A value like `-inf` should still write `null`, not `-null`
                    b'-' if self.at_start => {
                        self.sign_negative = true;
                        range.start += 1;
                        range.end += 1;
                    }
                    // JSON doesn't support a leading `+` sign
                    b'+' if self.at_start => {
                        range.start += 1;
                        range.end += 1;
                    }
                    // `snan`, `nan`, `inf` in any casing should write `null`
                    b's' | b'n' | b'i' | b'S' | b'N' | b'I' => {
                        self.is_nan_or_infinity = true;
                        self.at_start = false;

                        _try_no_conv!(out.write_str("null"));

                        range.start = 0;
                        range.end = 0;

                        break;
                    }
                    _ => range.end += 1,
                }
            }

            _try_no_conv!(out.write_str(&v[range]));
        }

        Ok(())
    }

    fn end(&mut self, mut out: impl Write) -> fmt::Result {
        if self.at_start {
            _try_no_conv!(out.write_char('0'));
        }

        Ok(())
    }
}

/*
This `escape_str` implementation has been shamelessly lifted from dtolnay's `miniserde`:
https://github.com/dtolnay/miniserde
*/

#[inline(always)]
fn escape_str(value: &str, mut out: impl Write) -> Result<(), fmt::Error> {
    let bytes = value.as_bytes();
    let mut start = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        let escape = ESCAPE[byte as usize];
        if escape == 0 {
            continue;
        }

        if start < i {
            _try_no_conv!(out.write_str(&value[start..i]));
        }

        match escape {
            BB => _try_no_conv!(out.write_str("\\b")),
            TT => _try_no_conv!(out.write_str("\\t")),
            NN => _try_no_conv!(out.write_str("\\n")),
            FF => _try_no_conv!(out.write_str("\\f")),
            RR => _try_no_conv!(out.write_str("\\r")),
            QU => _try_no_conv!(out.write_str("\\\"")),
            BS => _try_no_conv!(out.write_str("\\\\")),
            U => {
                static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
                _try_no_conv!(out.write_str("\\u00"));
                _try_no_conv!(out.write_char(HEX_DIGITS[(byte >> 4) as usize] as char));
                _try_no_conv!(out.write_char(HEX_DIGITS[(byte & 0xF) as usize] as char));
            }
            _ => unreachable!(),
        }

        start = i + 1;
    }

    if start != bytes.len() {
        _try_no_conv!(out.write_str(&value[start..]));
    }

    Ok(())
}

const BB: u8 = b'b'; // \x08
const TT: u8 = b't'; // \x09
const NN: u8 = b'n'; // \x0A
const FF: u8 = b'f'; // \x0C
const RR: u8 = b'r'; // \x0D
const QU: u8 = b'"'; // \x22
const BS: u8 = b'\\'; // \x5C
const U: u8 = b'u'; // \x00...\x1F except the ones above

// Lookup table of escape sequences. A value of b'x' at index i means that byte
// i is escaped as "\x" in JSON. A value of 0 means that byte i is not escaped.
#[rustfmt::skip]
static ESCAPE: [u8; 256] = [
    //  1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    U,  U,  U,  U,  U,  U,  U,  U, BB, TT, NN,  U, FF, RR,  U,  U, // 0
    U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U, // 1
    0,  0, QU,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 2
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 3
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 4
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, BS,  0,  0,  0, // 5
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 6
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 7
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 8
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 9
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // A
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // B
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // C
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // D
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // E
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // F
];
