extern crate utf8;

use std::borrow::Cow;
use std::collections::VecDeque;
use std::io;
use utf8::*;

/// A re-implementation of std::str::from_utf8
pub fn str_from_utf8(input: &[u8]) -> Result<&str, usize> {
    match decode(input) {
        Ok(s) => return Ok(s),
        Err(DecodeError::Invalid { valid_prefix, .. }) |
        Err(DecodeError::Incomplete { valid_prefix, .. }) => Err(valid_prefix.len()),
    }
}

#[test]
fn test_str_from_utf8() {
    let xs = b"hello";
    assert_eq!(str_from_utf8(xs), Ok("hello"));

    let xs = "ศไทย中华Việt Nam".as_bytes();
    assert_eq!(str_from_utf8(xs), Ok("ศไทย中华Việt Nam"));

    let xs = b"hello\xFF";
    assert!(str_from_utf8(xs).is_err());
}

#[test]
fn test_is_utf8() {
    // Chars of 1, 2, 3, and 4 bytes
    assert!(str_from_utf8("eé€\u{10000}".as_bytes()).is_ok());
    // invalid prefix
    assert!(str_from_utf8(&[0x80]).is_err());
    // invalid 2 byte prefix
    assert!(str_from_utf8(&[0xc0]).is_err());
    assert!(str_from_utf8(&[0xc0, 0x10]).is_err());
    // invalid 3 byte prefix
    assert!(str_from_utf8(&[0xe0]).is_err());
    assert!(str_from_utf8(&[0xe0, 0x10]).is_err());
    assert!(str_from_utf8(&[0xe0, 0xff, 0x10]).is_err());
    // invalid 4 byte prefix
    assert!(str_from_utf8(&[0xf0]).is_err());
    assert!(str_from_utf8(&[0xf0, 0x10]).is_err());
    assert!(str_from_utf8(&[0xf0, 0xff, 0x10]).is_err());
    assert!(str_from_utf8(&[0xf0, 0xff, 0xff, 0x10]).is_err());

    // deny overlong encodings
    assert!(str_from_utf8(&[0xc0, 0x80]).is_err());
    assert!(str_from_utf8(&[0xc0, 0xae]).is_err());
    assert!(str_from_utf8(&[0xe0, 0x80, 0x80]).is_err());
    assert!(str_from_utf8(&[0xe0, 0x80, 0xaf]).is_err());
    assert!(str_from_utf8(&[0xe0, 0x81, 0x81]).is_err());
    assert!(str_from_utf8(&[0xf0, 0x82, 0x82, 0xac]).is_err());
    assert!(str_from_utf8(&[0xf4, 0x90, 0x80, 0x80]).is_err());

    // deny surrogates
    assert!(str_from_utf8(&[0xED, 0xA0, 0x80]).is_err());
    assert!(str_from_utf8(&[0xED, 0xBF, 0xBF]).is_err());

    assert!(str_from_utf8(&[0xC2, 0x80]).is_ok());
    assert!(str_from_utf8(&[0xDF, 0xBF]).is_ok());
    assert!(str_from_utf8(&[0xE0, 0xA0, 0x80]).is_ok());
    assert!(str_from_utf8(&[0xED, 0x9F, 0xBF]).is_ok());
    assert!(str_from_utf8(&[0xEE, 0x80, 0x80]).is_ok());
    assert!(str_from_utf8(&[0xEF, 0xBF, 0xBF]).is_ok());
    assert!(str_from_utf8(&[0xF0, 0x90, 0x80, 0x80]).is_ok());
    assert!(str_from_utf8(&[0xF4, 0x8F, 0xBF, 0xBF]).is_ok());
}

/// A re-implementation of String::from_utf8_lossy
pub fn string_from_utf8_lossy(input: &[u8]) -> Cow<str> {
    let mut result = decode(input);
    if let Ok(s) = result {
        return s.into()
    }
    let mut string = String::with_capacity(input.len() + REPLACEMENT_CHARACTER.len());
    loop {
        match result {
            Ok(s) => {
                string.push_str(s);
                return string.into()
            }
            Err(DecodeError::Incomplete { valid_prefix, .. }) => {
                string.push_str(valid_prefix);
                string.push_str(REPLACEMENT_CHARACTER);
                return string.into()
            }
            Err(DecodeError::Invalid { valid_prefix, remaining_input, .. }) => {
                string.push_str(valid_prefix);
                string.push_str(REPLACEMENT_CHARACTER);
                result = decode(remaining_input);
            }
        }
    }
}

pub const DECODED_LOSSY: &'static [(&'static [u8], &'static str)] = &[
    (b"hello", "hello"),
    (b"\xe0\xb8\xa8\xe0\xb9\x84\xe0\xb8\x97\xe0\xb8\xa2\xe4\xb8\xad\xe5\x8d\x8e", "ศไทย中华"),
    (b"Vi\xe1\xbb\x87t Nam", "Việt Nam"),
    (b"Hello\xC2 There\xFF ", "Hello\u{FFFD} There\u{FFFD} "),
    (b"Hello\xC0\x80 There", "Hello\u{FFFD}\u{FFFD} There"),
    (b"\xE6\x83 Goodbye", "\u{FFFD} Goodbye"),
    (b"\xF5foo\xF5\x80bar", "\u{FFFD}foo\u{FFFD}\u{FFFD}bar"),
    (b"\xF5foo\xF5\xC2", "\u{FFFD}foo\u{FFFD}\u{FFFD}"),
    (b"\xF1foo\xF1\x80bar\xF1\x80\x80baz", "\u{FFFD}foo\u{FFFD}bar\u{FFFD}baz"),
    (b"\xF4foo\xF4\x80bar\xF4\xBFbaz", "\u{FFFD}foo\u{FFFD}bar\u{FFFD}\u{FFFD}baz"),
    (b"\xF0\x80\x80\x80foo\xF0\x90\x80\x80bar", "\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}foo\u{10000}bar"),
    (b"\xF0\x90\x80foo", "\u{FFFD}foo"),
    // surrogates
    (b"\xED\xA0\x80foo\xED\xBF\xBFbar", "\u{FFFD}\u{FFFD}\u{FFFD}foo\u{FFFD}\u{FFFD}\u{FFFD}bar"),
];

#[test]
fn test_string_from_utf8_lossy() {
    for &(input, expected) in DECODED_LOSSY {
        assert_eq!(string_from_utf8_lossy(input), expected);
    }
}

pub fn all_partitions<'a, F>(input: &'a [u8], f: F)
    where F: Fn(&[&[u8]])
{

    fn all_partitions_inner<'a, F>(chunks: &mut Vec<&'a [u8]>, input: &'a [u8], f: &F)
        where F: Fn(&[&[u8]])
    {
        if input.is_empty() {
            f(chunks)
        }
        for i in 1..(input.len() + 1) {
            chunks.push(&input[..i]);
            all_partitions_inner(chunks, &input[i..], f);
            chunks.pop();
        }
    }

    let mut chunks = Vec::new();
    all_partitions_inner(&mut chunks, input, &f);
    assert_eq!(chunks.len(), 0);
}

#[test]
fn test_incremental_decoder() {
    for &(input, expected) in DECODED_LOSSY {
        all_partitions(input, |chunks| {
            let mut string = String::new();
            {
                let mut decoder = LossyDecoder::new(|s| string.push_str(s));
                for &chunk in &*chunks {
                    decoder.feed(chunk);
                }
            }
            assert_eq!(string, expected);
        });
    }
}

#[test]
fn test_bufread_decoder() {
    for &(input, expected) in DECODED_LOSSY {
        all_partitions(input, |chunks| {
            let chunks = Chunks(chunks.to_vec().into());
            let string = BufReadDecoder::read_to_string_lossy(chunks).unwrap();
            assert_eq!(string, expected)
        });
    }
}

struct Chunks<'a>(VecDeque<&'a [u8]>);

impl<'a> io::Read for Chunks<'a> {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }
}

impl<'a> io::BufRead for Chunks<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Ok(*self.0.front().unwrap())
    }

    fn consume(&mut self, bytes: usize) {
        {
            let front = self.0.front_mut().unwrap();
            *front = &front[bytes..];
            if !front.is_empty() {
                return
            }
        }
        if self.0.len() > 1 {
            self.0.pop_front();
        }
    }

}
