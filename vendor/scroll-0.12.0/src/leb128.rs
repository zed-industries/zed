use core::convert::{AsRef, From};
use core::{result, u8};

use crate::ctx::TryFromCtx;
use crate::{error, Pread};

#[derive(Debug, PartialEq, Copy, Clone)]
/// An unsigned leb128 integer
pub struct Uleb128 {
    value: u64,
    count: usize,
}

impl Uleb128 {
    #[inline]
    /// Return how many bytes this Uleb128 takes up in memory
    pub fn size(&self) -> usize {
        self.count
    }
    #[inline]
    /// Read a variable length u64 from `bytes` at `offset`
    pub fn read(bytes: &[u8], offset: &mut usize) -> error::Result<u64> {
        let tmp = bytes.pread::<Uleb128>(*offset)?;
        *offset += tmp.size();
        Ok(tmp.into())
    }
}

impl AsRef<u64> for Uleb128 {
    fn as_ref(&self) -> &u64 {
        &self.value
    }
}

impl From<Uleb128> for u64 {
    #[inline]
    fn from(uleb128: Uleb128) -> u64 {
        uleb128.value
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// An signed leb128 integer
pub struct Sleb128 {
    value: i64,
    count: usize,
}

impl Sleb128 {
    #[inline]
    /// Return how many bytes this Sleb128 takes up in memory
    pub fn size(&self) -> usize {
        self.count
    }
    #[inline]
    /// Read a variable length i64 from `bytes` at `offset`
    pub fn read(bytes: &[u8], offset: &mut usize) -> error::Result<i64> {
        let tmp = bytes.pread::<Sleb128>(*offset)?;
        *offset += tmp.size();
        Ok(tmp.into())
    }
}

impl AsRef<i64> for Sleb128 {
    fn as_ref(&self) -> &i64 {
        &self.value
    }
}

impl From<Sleb128> for i64 {
    #[inline]
    fn from(sleb128: Sleb128) -> i64 {
        sleb128.value
    }
}

// Below implementation heavily adapted from: https://github.com/fitzgen/leb128
const CONTINUATION_BIT: u8 = 1 << 7;
const SIGN_BIT: u8 = 1 << 6;

#[inline]
fn mask_continuation(byte: u8) -> u8 {
    byte & !CONTINUATION_BIT
}

// #[inline]
// fn mask_continuation_u64(val: u64) -> u8 {
//     let byte = val & (u8::MAX as u64);
//     mask_continuation(byte as u8)
// }

impl<'a> TryFromCtx<'a> for Uleb128 {
    type Error = error::Error;
    #[inline]
    fn try_from_ctx(src: &'a [u8], _ctx: ()) -> result::Result<(Self, usize), Self::Error> {
        let mut result = 0;
        let mut shift = 0;
        let mut count = 0;
        loop {
            let byte: u8 = src.pread(count)?;

            if shift == 63 && byte != 0x00 && byte != 0x01 {
                return Err(error::Error::BadInput {
                    size: src.len(),
                    msg: "failed to parse",
                });
            }

            let low_bits = u64::from(mask_continuation(byte));
            result |= low_bits << shift;

            count += 1;
            shift += 7;

            if byte & CONTINUATION_BIT == 0 {
                return Ok((
                    Uleb128 {
                        value: result,
                        count,
                    },
                    count,
                ));
            }
        }
    }
}

impl<'a> TryFromCtx<'a> for Sleb128 {
    type Error = error::Error;
    #[inline]
    fn try_from_ctx(src: &'a [u8], _ctx: ()) -> result::Result<(Self, usize), Self::Error> {
        let o = 0;
        let offset = &mut 0;
        let mut result = 0;
        let mut shift = 0;
        let size = 64;
        let mut byte: u8;
        loop {
            byte = src.gread(offset)?;

            if shift == 63 && byte != 0x00 && byte != 0x7f {
                return Err(error::Error::BadInput {
                    size: src.len(),
                    msg: "failed to parse",
                });
            }

            let low_bits = i64::from(mask_continuation(byte));
            result |= low_bits << shift;
            shift += 7;

            if byte & CONTINUATION_BIT == 0 {
                break;
            }
        }

        if shift < size && (SIGN_BIT & byte) == SIGN_BIT {
            // Sign extend the result.
            result |= !0 << shift;
        }
        let count = *offset - o;
        Ok((
            Sleb128 {
                value: result,
                count,
            },
            count,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::LE;
    use super::{Sleb128, Uleb128};

    const CONTINUATION_BIT: u8 = 1 << 7;
    //const SIGN_BIT: u8 = 1 << 6;

    #[test]
    fn uleb_size() {
        use super::super::Pread;
        let buf = [2u8 | CONTINUATION_BIT, 1];
        let bytes = &buf[..];
        let num = bytes.pread::<Uleb128>(0).unwrap();
        #[cfg(feature = "std")]
        println!("num: {num:?}");
        assert_eq!(130u64, num.into());
        assert_eq!(num.size(), 2);

        let buf = [0x00, 0x01];
        let bytes = &buf[..];
        let num = bytes.pread::<Uleb128>(0).unwrap();
        #[cfg(feature = "std")]
        println!("num: {num:?}");
        assert_eq!(0u64, num.into());
        assert_eq!(num.size(), 1);

        let buf = [0x21];
        let bytes = &buf[..];
        let num = bytes.pread::<Uleb128>(0).unwrap();
        #[cfg(feature = "std")]
        println!("num: {num:?}");
        assert_eq!(0x21u64, num.into());
        assert_eq!(num.size(), 1);
    }

    #[test]
    fn uleb128() {
        use super::super::Pread;
        let buf = [2u8 | CONTINUATION_BIT, 1];
        let bytes = &buf[..];
        let num = bytes.pread::<Uleb128>(0).expect("Should read Uleb128");
        assert_eq!(130u64, num.into());
        assert_eq!(
            386,
            bytes.pread_with::<u16>(0, LE).expect("Should read number")
        );
    }

    #[test]
    fn uleb128_overflow() {
        use super::super::Pread;
        let buf = [
            2u8 | CONTINUATION_BIT,
            2 | CONTINUATION_BIT,
            2 | CONTINUATION_BIT,
            2 | CONTINUATION_BIT,
            2 | CONTINUATION_BIT,
            2 | CONTINUATION_BIT,
            2 | CONTINUATION_BIT,
            2 | CONTINUATION_BIT,
            2 | CONTINUATION_BIT,
            2 | CONTINUATION_BIT,
            1,
        ];
        let bytes = &buf[..];
        assert!(bytes.pread::<Uleb128>(0).is_err());
    }

    #[test]
    fn sleb128() {
        use super::super::Pread;
        let bytes = [0x7fu8 | CONTINUATION_BIT, 0x7e];
        let num: i64 = bytes
            .pread::<Sleb128>(0)
            .expect("Should read Sleb128")
            .into();
        assert_eq!(-129, num);
    }
}
