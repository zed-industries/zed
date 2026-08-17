//! A parser for the compound text encoding used by the X Input Method protocol.
//!
//! Currently only support utf8 mode. This is intended to be used as a building block for
//! higher level libraries. See the [`xim`] crate for an example.
//!
//! [xim]: https://crates.io/crates/xim

#![no_std]
#![allow(clippy::uninlined_format_args)]
#![forbid(unsafe_code, future_incompatible)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[cfg(feature = "std")]
use std::io::{self, Write};

const UTF8_START: &[u8] = &[0x1B, 0x25, 0x47];
const UTF8_END: &[u8] = &[0x1B, 0x25, 0x40];

/// Wrapper for reduce allocation
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CText<'s> {
    utf8: &'s str,
}

impl<'s> fmt::Debug for CText<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.utf8)
    }
}

impl<'s> fmt::Display for CText<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.utf8)
    }
}

impl<'s> CText<'s> {
    pub const fn new(utf8: &'s str) -> Self {
        Self { utf8 }
    }

    pub const fn len(self) -> usize {
        self.utf8.len() + UTF8_START.len() + UTF8_END.len()
    }

    pub const fn is_empty(self) -> bool {
        self.utf8.is_empty()
    }

    #[cfg(feature = "std")]
    pub fn write(self, mut out: impl Write) -> io::Result<usize> {
        let mut writed = 0;
        writed += out.write(UTF8_START)?;
        writed += out.write(self.utf8.as_bytes())?;
        writed += out.write(UTF8_END)?;
        Ok(writed)
    }
}

/// Encoding utf8 to COMPOUND_TEXT with utf8 escape
pub fn utf8_to_compound_text(text: &str) -> Vec<u8> {
    let mut ret = Vec::with_capacity(text.len() + 6);
    ret.extend_from_slice(UTF8_START);
    ret.extend_from_slice(text.as_bytes());
    ret.extend_from_slice(UTF8_END);
    ret
}

#[derive(Debug, Clone)]
pub enum DecodeError {
    InvalidEncoding,
    UnsupportedEncoding,
    Utf8Error(alloc::string::FromUtf8Error),
}

impl From<alloc::string::FromUtf8Error> for DecodeError {
    fn from(err: alloc::string::FromUtf8Error) -> Self {
        DecodeError::Utf8Error(err)
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEncoding => write!(f, "Invalid compound text"),
            Self::UnsupportedEncoding => write!(f, "This encoding is not supported yet"),
            Self::Utf8Error(e) => write!(f, "Not a valid utf8 {}", e),
        }
    }
}

macro_rules! decode {
    ($decoder:expr, $out:expr, $bytes:expr, $last:expr) => {
        let mut _current_bytes: &[u8] = $bytes;
        loop {
            let (ret, nread, _) = $decoder.decode_to_string(_current_bytes, $out, $last);

            match ret {
                encoding_rs::CoderResult::InputEmpty => break,
                encoding_rs::CoderResult::OutputFull => {
                    $out.reserve(
                        $decoder
                            .max_utf8_buffer_length($bytes.len())
                            .unwrap_or_default(),
                    );
                    _current_bytes = &_current_bytes[nread..];
                }
            }
        }
    };
}

pub fn compound_text_to_utf8(bytes: &[u8]) -> Result<String, DecodeError> {
    let split: Vec<&[u8]> = bytes.split(|&b| b == 0x1b).collect();

    let mut result = String::new();

    for chunk in split {
        let mut iter = chunk.iter();
        match (iter.next(), iter.next()) {
            // UTF-8
            (Some(0x25), Some(0x47)) => {
                let left = iter.as_slice().to_vec();
                match String::from_utf8(left) {
                    Ok(out) => result.push_str(&out),
                    Err(e) => return Err(DecodeError::from(e)),
                };
            }
            // UTF-8 End
            (Some(0x25), Some(0x40)) => {}
            // 94N
            (Some(0x24), Some(0x28)) => match iter.next() {
                // JP
                Some(0x42) => {
                    let left = iter.as_slice();
                    let mut decoder = encoding_rs::ISO_2022_JP.new_decoder_without_bom_handling();
                    let mut out = String::new();
                    decode!(decoder, &mut out, &[0x1B, 0x24, 0x42], false);
                    decode!(decoder, &mut out, &left, true);

                    result.push_str(&out);
                }

                // CN (GB2312)
                Some(0x41) => {
                    let left: Vec<u8> = iter.map(|&b| b + 0x80).collect();
                    let (out, _) = encoding_rs::GBK.decode_without_bom_handling(&left);
                    result.push_str(&out);
                }

                // KR (KS C 5601)
                Some(0x43) => {
                    let left: Vec<u8> = iter.map(|&b| b + 0x80).collect();
                    let (out, _) = encoding_rs::EUC_KR.decode_with_bom_removal(&left);
                    result.push_str(&out);
                }
                // Invalid encode
                _ => return Err(DecodeError::InvalidEncoding),
            },
            // ISO-8859-1
            (Some(0x2d), Some(0x41)) => {
                let left = iter.as_slice();
                let out = encoding_rs::mem::decode_latin1(left);
                result.push_str(&out);
            }
            // ISO-8859-2
            (Some(0x2d), Some(0x42)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_2.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-3
            (Some(0x2d), Some(0x43)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_3.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-4
            (Some(0x2d), Some(0x44)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_4.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-7
            (Some(0x2d), Some(0x46)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_7.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-6
            (Some(0x2d), Some(0x47)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_6.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-8
            (Some(0x2d), Some(0x48)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_8.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-5
            (Some(0x2d), Some(0x4c)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_5.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-9
            (Some(0x2d), Some(0x4d)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::WINDOWS_1254.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-10
            (Some(0x2d), Some(0x56)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_10.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-13
            (Some(0x2d), Some(0x59)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_13.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-14
            (Some(0x2d), Some(0x5f)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_14.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-15
            (Some(0x2d), Some(0x62)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_15.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // ISO-8859-16
            (Some(0x2d), Some(0x66)) => {
                let left = iter.as_slice();
                let (out, _) = encoding_rs::ISO_8859_16.decode_without_bom_handling(left);
                result.push_str(&out);
            }
            // defaults to ISO-8859-1
            _ => {
                let out = encoding_rs::mem::decode_latin1(chunk);
                result.push_str(&out);
            }
        };
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn korean() {
        const UTF8: &str = "가나다";
        const COMP: &[u8] = &[
            27, 37, 71, 234, 176, 128, 235, 130, 152, 235, 139, 164, 27, 37, 64,
        ];
        assert_eq!(crate::utf8_to_compound_text(UTF8), COMP);
        assert_eq!(crate::compound_text_to_utf8(COMP).unwrap(), UTF8);
    }

    #[test]
    fn iso_2022_jp() {
        const UTF8: &str = "東京";
        const COMP: &[u8] = &[27, 36, 40, 66, 69, 108, 53, 126];
        assert_eq!(crate::compound_text_to_utf8(COMP).unwrap(), UTF8);
    }

    #[test]
    fn iso_2022_jp_long() {
        const UTF8: &str = "知ってるつもり";
        const COMP: &[u8] = &[
            27, 36, 40, 66, 67, 78, 36, 67, 36, 70, 36, 107, 36, 68, 36, 98, 36, 106, 27, 40, 66,
        ];
        assert_eq!(crate::compound_text_to_utf8(COMP).unwrap(), UTF8);
    }

    #[test]
    fn gb2312_cn() {
        const UTF8: &str = "很高兴认识你";
        const COMP: &[u8] = &[
            0x1b, 0x24, 0x28, 0x41, 0x3a, 0x5c, 0x38, 0x5f, 0x50, 0x4b, 0x48, 0x4f, 0x4a, 0x36,
            0x44, 0x63,
        ];
        assert_eq!(crate::compound_text_to_utf8(COMP).unwrap(), UTF8);
    }

    #[test]
    fn gb2312_cn_mixed() {
        const UTF8: &str = "炸哦你";
        const COMP: &[u8] = &[
            0x1b, 0x24, 0x28, 0x42, 0x5f, 0x5a, 0x53, 0x28, 0x1b, 0x24, 0x28, 0x41, 0x44, 0x63,
        ];
        assert_eq!(crate::compound_text_to_utf8(COMP).unwrap(), UTF8);
    }

    #[test]
    fn ks_c_5601() {
        const UTF8: &str = "넌최고야";
        const COMP: &[u8] = &[
            0x1b, 0x24, 0x28, 0x43, 0x33, 0x4d, 0x43, 0x56, 0x30, 0x6d, 0x3e, 0x5f,
        ];
        assert_eq!(crate::compound_text_to_utf8(COMP).unwrap(), UTF8);
    }

    #[test]
    fn iso_8859_1() {
        const UTF8: &str = "¡¸ÀÑâó";
        const COMP: &[u8] = &[0x1b, 0x2d, 0x41, 0xa1, 0xb8, 0xc0, 0xd1, 0xe2, 0xf3];
        assert_eq!(crate::compound_text_to_utf8(COMP).unwrap(), UTF8);
    }

    #[test]
    fn iso_8859_2() {
        const UTF8: &str = "ĄŁĽŚŠŤ";
        const COMP: &[u8] = &[0x1b, 0x2d, 0x42, 0xa1, 0xa3, 0xa5, 0xa6, 0xa9, 0xab];
        assert_eq!(crate::compound_text_to_utf8(COMP).unwrap(), UTF8);
    }
}
