// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use crate::enums::*;
use crate::reg_key::RegKey;
use crate::reg_value::RegValue;
use crate::types::FromRegValue;
use std::error::Error;
use std::fmt;
use std::io;

macro_rules! parse_string {
    ($s:ident) => {{
        let s: String = $s.read_value()?;
        s.parse()
            .map_err(|e| DecoderError::ParseError(format!("{:?}", e)))
    }};
}

macro_rules! no_impl {
    ($e:expr) => {
        Err(DecoderError::DecodeNotImplemented($e.to_owned()))
    };
}

#[cfg(feature = "serialization-serde")]
mod serialization_serde;

#[derive(Debug)]
pub enum DecoderError {
    DecodeNotImplemented(String),
    DeserializerError(String),
    IoError(io::Error),
    ParseError(String),
    NoFieldName,
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DecoderError {}

impl From<io::Error> for DecoderError {
    fn from(err: io::Error) -> DecoderError {
        DecoderError::IoError(err)
    }
}

pub type DecodeResult<T> = Result<T, DecoderError>;

#[derive(Debug, Clone)]
enum DecoderCursor {
    Start,
    Key(u32),
    KeyName(u32, String),
    KeyVal(u32, String),
    Field(u32),
    FieldName(u32, String),
    FieldVal(u32, String),
}

#[derive(Debug)]
pub struct Decoder {
    key: RegKey,
    cursor: DecoderCursor,
}

const DECODER_SAM: u32 = KEY_QUERY_VALUE | KEY_ENUMERATE_SUB_KEYS;

impl Decoder {
    pub fn from_key(key: &RegKey) -> DecodeResult<Decoder> {
        key.open_subkey_with_flags("", DECODER_SAM)
            .map(Decoder::new)
            .map_err(DecoderError::IoError)
    }

    fn new(key: RegKey) -> Decoder {
        Decoder {
            key,
            cursor: DecoderCursor::Start,
        }
    }

    fn read_value<T: FromRegValue>(&mut self) -> Result<T, DecoderError> {
        use self::DecoderCursor::*;
        let cursor = self.cursor.clone();
        match cursor {
            FieldVal(index, name) => {
                self.cursor = DecoderCursor::Field(index + 1);
                self.key.get_value(name).map_err(DecoderError::IoError)
            }
            _ => Err(DecoderError::DeserializerError("Not a value".to_owned())),
        }
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>, DecoderError> {
        use self::DecoderCursor::*;
        let cursor = self.cursor.clone();
        match cursor {
            FieldVal(index, name) => {
                self.cursor = DecoderCursor::Field(index + 1);
                let RegValue { bytes, .. } = self
                    .key
                    .get_raw_value(name)
                    .map_err(DecoderError::IoError)?;
                Ok(bytes)
            }
            _ => Err(DecoderError::DeserializerError("Not a value".to_owned())),
        }
    }
}
