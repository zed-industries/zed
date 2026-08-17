// Copyright 2018 the SVG Types Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// List of all errors.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// An input data ended earlier than expected.
    ///
    /// Should only appear on invalid input data.
    /// Errors in a valid XML should be handled by errors below.
    UnexpectedEndOfStream,

    /// An input text contains unknown data.
    UnexpectedData(usize),

    /// A provided string doesn't have a valid data.
    ///
    /// For example, if we try to parse a color form `zzz`
    /// string - we will get this error.
    /// But if we try to parse a number list like `1.2 zzz`,
    /// then we will get `InvalidNumber`, because at least some data is valid.
    InvalidValue,

    /// An invalid ident.
    ///
    /// CSS idents have certain rules with regard to the characters they may contain.
    /// For example, they may not start with a number. If an invalid ident is encountered,
    /// this error will be returned.
    InvalidIdent,

    /// An invalid/unexpected character.
    ///
    /// The first byte is an actual one, others - expected.
    ///
    /// We are using a single value to reduce the struct size.
    InvalidChar(Vec<u8>, usize),

    /// An unexpected character instead of an XML space.
    ///
    /// The first string is an actual one, others - expected.
    ///
    /// We are using a single value to reduce the struct size.
    InvalidString(Vec<String>, usize),

    /// An invalid number.
    InvalidNumber(usize),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::UnexpectedEndOfStream => {
                write!(f, "unexpected end of stream")
            }
            Self::UnexpectedData(pos) => {
                write!(f, "unexpected data at position {pos}")
            }
            Self::InvalidValue => {
                write!(f, "invalid value")
            }
            Self::InvalidIdent => {
                write!(f, "invalid ident")
            }
            Self::InvalidChar(ref chars, pos) => {
                // Vec<u8> -> Vec<String>
                let list: Vec<String> = chars
                    .iter()
                    .skip(1)
                    .map(|c| String::from_utf8(vec![*c]).unwrap())
                    .collect();

                write!(
                    f,
                    "expected '{}' not '{}' at position {}",
                    list.join("', '"),
                    chars[0] as char,
                    pos
                )
            }
            Self::InvalidString(ref strings, pos) => {
                write!(
                    f,
                    "expected '{}' not '{}' at position {}",
                    strings[1..].join("', '"),
                    strings[0],
                    pos
                )
            }
            Self::InvalidNumber(pos) => {
                write!(f, "invalid number at position {pos}")
            }
        }
    }
}

impl core::error::Error for Error {
    fn description(&self) -> &str {
        "an SVG data parsing error"
    }
}
