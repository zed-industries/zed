/* Copyright 2016-2022 Torbjørn Birch Moltu
 * Copyright 2018 Aljoscha Meyer
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */

use crate::utf8_char::Utf8Char;
use crate::utf16_char::Utf16Char;
use crate::utf8_iterators::*;
use crate::utf16_iterators::*;
use crate::decoding_iterators::*;
use crate::error::*;
use crate::error::Utf8ErrorKind::*;
extern crate core;
use core::{char, u32};
use core::ops::{Not, Index, RangeFull};
use core::borrow::Borrow;
#[cfg(feature="ascii")]
extern crate ascii;
#[cfg(feature="ascii")]
use ascii::AsciiStr;

// TODO better docs and tests

/// Methods for working with `u8`s as UTF-8 bytes.
pub trait U8UtfExt {
    /// How many more bytes will you need to complete this codepoint?
    ///
    /// # Errors
    ///
    /// An error is returned if the byte is not a valid start of an UTF-8
    /// codepoint:
    ///
    /// * `128..192`: [`UnexpectedContinuationByte`](error/enum.Utf8ErrorKind.html#variant.UnexpectedContinuationByte)
    /// * `245..`, `192` and `193`: [`NonUtf8Byte`](error/enum.Utf8ErrorKind.html#variant.NonUtf8Byte)  
    fn extra_utf8_bytes(self) -> Result<usize,Utf8Error>;

    /// How many more bytes will you need to complete this codepoint?
    ///
    /// This function assumes that the byte is a valid UTF-8 start, and might
    /// return any value otherwise. (but the function is safe to call with any
    /// value and will return a consistent result).
    fn extra_utf8_bytes_unchecked(self) -> usize;
}

impl U8UtfExt for u8 {
    #[inline]
    fn extra_utf8_bytes(self) -> Result<usize,Utf8Error> {
        match self {
            0x00..=0x7f => Ok(0),
            0xc2..=0xdf => Ok(1),
            0xe0..=0xef => Ok(2),
            0xf0..=0xf4 => Ok(3),
            0xc0..=0xc1 | 0xf5..=0xff => Err(Utf8Error{ kind: NonUtf8Byte }),// too big or overlong
            0x80..=0xbf => Err(Utf8Error{ kind: UnexpectedContinuationByte }),// following byte
        }
    }
    #[inline]
    fn extra_utf8_bytes_unchecked(self) -> usize {
        // For fun I've optimized this function (for x86 instruction count):
        // The most straightforward implementation, that lets the compiler do
        // the optimizing:
        //match self {
        //    0b0000_0000...0b0111_1111 => 0,
        //    0b1100_0010...0b1101_1111 => 1,
        //    0b1110_0000...0b1110_1111 => 2,
        //    0b1111_0000...0b1111_0100 => 3,
        //                _             => whatever()
        //}
        // Using `unsafe{core::hint::unreachable_unchecked()}` for the
        // "don't care" case is a terrible idea: while having the function
        // non-deterministically return whatever happens to be in a register
        // MIGHT be acceptable, it permits the function to not `ret`urn at all,
        // but let execution fall through to whatever comes after it in the
        // binary! (in other words completely UB).
        // Currently unreachable_unchecked() might trap too,
        // which is certainly not what we want.
        // I also think `unsafe{mem::unitialized()}` is much more likely to
        // explicitly produce whatever happens to be in a register than tell
        // the compiler it can ignore this branch but needs to produce a value.
        //
        // From the bit patterns we see that for non-ASCII values the result is
        // (number of leading set bits) - 1
        // The standard library doesn't have a method for counting leading ones,
        // but it has leading_zeros(), which can be used after inverting.
        // This function can therefore be reduced to the one-liner
        //`self.not().leading_zeros().saturating_sub(1) as usize`, which would
        // be branchless for architectures with instructions for
        // leading_zeros() and saturating_sub().

        // Shortest version as long as ASCII-ness can be predicted: (especially
        // if the BSR instruction which leading_zeros() uses is microcoded or
        // doesn't exist)
        // u8.leading_zeros() would cast to a bigger type internally, so that's
        // free. compensating by shifting left by 24 before inverting lets the
        // compiler know that the value passed to leading_zeros() is not zero,
        // for which BSR's output is undefined and leading_zeros() normally has
        // special case with a branch.
        // Shifting one bit too many left acts as a saturating_sub(1).
        if self<128 {0} else {((self as u32)<<25).not().leading_zeros() as usize}

        // Branchless but longer version: (9 instructions)
        // It's tempting to try (self|0x80).not().leading_zeros().wrapping_sub(1),
        // but that produces high lengths for ASCII values 0b01xx_xxxx.
        // If we could somehow (branchlessy) clear that bit for ASCII values...
        // We can by masking with the value shifted right with sign extension!
        // (any nonzero number of bits in range works)
        //let extended = self as i8 as i32;
        //let ascii_cleared = (extended<<25) & (extended>>25);
        //ascii_cleared.not().leading_zeros() as usize

        // cmov version: (7 instructions)
        //(((self as u32)<<24).not().leading_zeros() as usize).saturating_sub(1)
    }
}


/// Methods for working with `u16`s as UTF-16 units.
pub trait U16UtfExt {
    /// Will you need an extra unit to complete this codepoint?
    ///
    /// Returns `Err` for trailing surrogates, `Ok(true)` for leading surrogates,
    /// and `Ok(false)` for others.
    fn utf16_needs_extra_unit(self) -> Result<bool,Utf16FirstUnitError>;

    /// Does this `u16` need another `u16` to complete a codepoint?
    /// Returns `(self & 0xfc00) == 0xd800`
    ///
    /// Is basically an unchecked variant of `utf16_needs_extra_unit()`.
    fn is_utf16_leading_surrogate(self) -> bool;
}
impl U16UtfExt for u16 {
    #[inline]
    fn utf16_needs_extra_unit(self) -> Result<bool,Utf16FirstUnitError> {
        match self {
            // https://en.wikipedia.org/wiki/UTF-16#U.2B10000_to_U.2B10FFFF
            0x00_00..=0xd7_ff | 0xe0_00..=0xff_ff => Ok(false),
            0xd8_00..=0xdb_ff => Ok(true),
                    _         => Err(Utf16FirstUnitError)
        }
    }
    #[inline]
    fn is_utf16_leading_surrogate(self) -> bool {
        (self & 0xfc00) == 0xd800// Clear the ten content bytes of a surrogate,
                                 // and see if it's a leading surrogate.
    }
}




/// Extension trait for `char` that adds methods for converting to and from UTF-8 or UTF-16.
pub trait CharExt: Sized {
    /// Get the UTF-8 representation of this codepoint.
    ///
    /// `Utf8Char` is to `[u8;4]` what `char` is to `u32`:
    /// a restricted type that cannot be mutated internally.
    fn to_utf8(self) -> Utf8Char;

    /// Get the UTF-16 representation of this codepoint.
    ///
    /// `Utf16Char` is to `[u16;2]` what `char` is to `u32`:
    /// a restricted type that cannot be mutated internally.
    fn to_utf16(self) -> Utf16Char;

    /// Iterate over or [read](https://doc.rust-lang.org/std/io/trait.Read.html)
    /// the one to four bytes in the UTF-8 representation of this codepoint.
    ///
    /// An identical alternative to the unstable `char.encode_utf8()`.
    /// That method somehow still exist on stable, so I have to use a different name.
    fn iter_utf8_bytes(self) -> Utf8Iterator;

    /// Iterate over the one or two units in the UTF-16 representation of this codepoint.
    ///
    /// An identical alternative to the unstable `char.encode_utf16()`.
    /// That method somehow still exist on stable, so I have to use a different name.
    fn iter_utf16_units(self) -> Utf16Iterator;


    /// Convert this char to an UTF-8 array, and also return how many bytes of
    /// the array are used,
    ///
    /// The returned array is left-aligned with unused bytes set to zero.
    fn to_utf8_array(self) -> ([u8; 4], usize);

    /// Convert this `char` to UTF-16.
    ///
    /// The second element is non-zero when a surrogate pair is required.
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_unicode::CharExt;
    ///
    /// assert_eq!('@'.to_utf16_array(), ['@' as u16, 0]);
    /// assert_eq!('睷'.to_utf16_array(), ['睷' as u16, 0]);
    /// assert_eq!('\u{abcde}'.to_utf16_array(), [0xda6f, 0xdcde]);
    /// ```
    fn to_utf16_array(self) -> [u16; 2];

    /// Convert this `char` to UTF-16.
    /// The second item is `Some` if a surrogate pair is required.
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_unicode::CharExt;
    ///
    /// assert_eq!('@'.to_utf16_tuple(), ('@' as u16, None));
    /// assert_eq!('睷'.to_utf16_tuple(), ('睷' as u16, None));
    /// assert_eq!('\u{abcde}'.to_utf16_tuple(), (0xda6f, Some(0xdcde)));
    /// ```
    fn to_utf16_tuple(self) -> (u16, Option<u16>);



    /// Create a `char` from the start of an UTF-8 slice,
    /// and also return how many bytes were used.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the slice is empty, doesn't start with a valid
    /// UTF-8 sequence or is too short for the sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_unicode::CharExt;
    /// use encode_unicode::error::Utf8ErrorKind::*;
    ///
    /// assert_eq!(char::from_utf8_slice_start(&[b'A', b'B', b'C']), Ok(('A',1)));
    /// assert_eq!(char::from_utf8_slice_start(&[0xdd, 0xbb]), Ok(('\u{77b}',2)));
    ///
    /// assert_eq!(char::from_utf8_slice_start(&[]).unwrap_err(), TooFewBytes);
    /// assert_eq!(char::from_utf8_slice_start(&[0xf0, 0x99]).unwrap_err(), TooFewBytes);
    /// assert_eq!(char::from_utf8_slice_start(&[0xee, b'F', 0x80]).unwrap_err(), InterruptedSequence);
    /// assert_eq!(char::from_utf8_slice_start(&[0xee, 0x99, 0x0f]).unwrap_err(), InterruptedSequence);
    /// ```
    fn from_utf8_slice_start(src: &[u8]) -> Result<(Self,usize),Utf8Error>;

    /// Create a `char` from the start of an UTF-16 slice,
    /// and also return how many units were used.
    ///
    /// If you want to continue after an error, continue with the next `u16` unit.
    fn from_utf16_slice_start(src: &[u16]) -> Result<(Self,usize), Utf16SliceError>;


    /// Convert an UTF-8 sequence as returned from `.to_utf8_array()` into a `char`
    ///
    /// The codepoint must start at the first byte, and leftover bytes are ignored.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the array doesn't start with a valid UTF-8 sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_unicode::CharExt;
    /// use encode_unicode::error::Utf8ErrorKind::*;
    ///
    /// assert_eq!(char::from_utf8_array([b'A', 0, 0, 0]), Ok('A'));
    /// assert_eq!(char::from_utf8_array([0xf4, 0x8b, 0xbb, 0xbb]), Ok('\u{10befb}'));
    /// assert_eq!(char::from_utf8_array([b'A', b'B', b'C', b'D']), Ok('A'));
    /// assert_eq!(char::from_utf8_array([0, 0, 0xcc, 0xbb]), Ok('\0'));
    ///
    /// assert_eq!(char::from_utf8_array([0xef, b'F', 0x80, 0x80]).unwrap_err(), InterruptedSequence);
    /// assert_eq!(char::from_utf8_array([0xc1, 0x80, 0, 0]).unwrap_err().kind(), NonUtf8Byte);
    /// assert_eq!(char::from_utf8_array([0xe0, 0x9a, 0xbf, 0]).unwrap_err().kind(), OverlongEncoding);
    /// assert_eq!(char::from_utf8_array([0xf4, 0xaa, 0x99, 0x88]).unwrap_err(), TooHighCodepoint);
    /// ```
    fn from_utf8_array(utf8: [u8; 4]) -> Result<Self,Utf8Error>;

    /// Convert a UTF-16 pair as returned from `.to_utf16_array()` into a `char`.
    ///
    /// The second element is ignored when not required.
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_unicode::CharExt;
    /// use encode_unicode::error::Utf16ArrayError;
    ///
    /// assert_eq!(char::from_utf16_array(['x' as u16, 'y' as u16]), Ok('x'));
    /// assert_eq!(char::from_utf16_array(['睷' as u16, 0]), Ok('睷'));
    /// assert_eq!(char::from_utf16_array([0xda6f, 0xdcde]), Ok('\u{abcde}'));
    /// assert_eq!(char::from_utf16_array([0xf111, 0xdbad]), Ok('\u{f111}'));
    /// assert_eq!(char::from_utf16_array([0xdaaf, 0xdaaf]), Err(Utf16ArrayError::SecondIsNotTrailingSurrogate));
    /// assert_eq!(char::from_utf16_array([0xdcac, 0x9000]), Err(Utf16ArrayError::FirstIsTrailingSurrogate));
    /// ```
    fn from_utf16_array(utf16: [u16; 2]) -> Result<Self, Utf16ArrayError>;

    /// Convert a UTF-16 pair as returned from `.to_utf16_tuple()` into a `char`.
    fn from_utf16_tuple(utf16: (u16, Option<u16>)) -> Result<Self, Utf16TupleError>;


    /// Convert an UTF-8 sequence into a char.
    ///
    /// The length of the slice is taken as length of the sequence;
    /// it should be 1,2,3 or 4.
    ///
    /// # Safety
    ///
    /// The slice must contain exactly one, valid, UTF-8 sequence.
    ///
    /// Passing a slice that produces an invalid codepoint is always undefined
    /// behavior; Later checks that the codepoint is valid can be removed
    /// by the compiler.
    ///
    /// # Panics
    ///
    /// If the slice is empty
    unsafe fn from_utf8_exact_slice_unchecked(src: &[u8]) -> Self;

    /// Convert a UTF-16 array as returned from `.to_utf16_array()` into a
    /// `char`.
    ///
    /// This function is safe because it avoids creating invalid codepoints,
    /// but the returned value might not be what one expectedd.
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_unicode::CharExt;
    ///
    /// // starts with a trailing surrogate - converted as if it was a valid
    /// // surrogate pair anyway.
    /// assert_eq!(char::from_utf16_array_unchecked([0xdbad, 0xf19e]), '\u{fb59e}');
    /// // missing trailing surrogate - ditto
    /// assert_eq!(char::from_utf16_array_unchecked([0xd802, 0]), '\u{10800}');
    /// ```
    fn from_utf16_array_unchecked(utf16: [u16;2]) -> Self;

    /// Convert a UTF-16 tuple as returned from `.to_utf16_tuple()` into a `char`.
    ///
    /// # Safety
    ///
    /// If the second element is `None`, the first element must be a codepoint
    /// in the basic multilingual pane.
    /// (In other words, outside the range`0xd8_00..0xe0_00`.)  
    /// Violating this results in an invalid `char` in that reserved range
    /// being created, which is (or can easily lead to) undefined behavior.
    unsafe fn from_utf16_tuple_unchecked(utf16: (u16, Option<u16>)) -> Self;


    /// Produces more detailed errors than `char::from_u32()`
    ///
    /// # Errors
    ///
    /// This function will return an error if
    ///
    /// * the value is greater than 0x10ffff
    /// * the value is between 0xd800 and 0xdfff (inclusive)
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_unicode::CharExt;
    /// use encode_unicode::error::CodepointError;
    ///
    /// assert_eq!(char::from_u32_detailed(0x41), Ok('A'));
    /// assert_eq!(char::from_u32_detailed(0x40_00_00), Err(CodepointError::TooHigh));
    /// assert_eq!(char::from_u32_detailed(0xd951), Err(CodepointError::Utf16Reserved));
    /// assert_eq!(char::from_u32_detailed(0xdddd), Err(CodepointError::Utf16Reserved));
    /// assert_eq!(char::from_u32_detailed(0xdd), Ok('Ý'));
    /// assert_eq!(char::from_u32_detailed(0x1f331), Ok('🌱'));
    /// ```
    fn from_u32_detailed(c: u32) -> Result<Self,CodepointError>;
}



impl CharExt for char {
      /////////
     //UTF-8//
    /////////

    fn to_utf8(self) -> Utf8Char {
        self.into()
    }
    fn iter_utf8_bytes(self) -> Utf8Iterator {
        self.to_utf8().into_iter()
    }

    fn to_utf8_array(self) -> ([u8; 4], usize) {
        let len = self.len_utf8();
        let mut c = self as u32;
        if len == 1 {// ASCII, the common case
            ([c as u8, 0, 0, 0],  1)
        } else {
            let mut parts = 0;// convert to 6-bit bytes
                        parts |= c & 0x3f;  c>>=6;
            parts<<=8;  parts |= c & 0x3f;  c>>=6;
            parts<<=8;  parts |= c & 0x3f;  c>>=6;
            parts<<=8;  parts |= c & 0x3f;
            parts |= 0x80_80_80_80;// set the most significant bit
            parts >>= 8*(4-len);// right-align bytes
            // Now, unused bytes are zero, (which matters for Utf8Char.eq())
            // and the rest are 0b10xx_xxxx

            // set header on first byte
            parts |= (0xff_00u32 >> len)  &  0xff;// store length
            parts &= Not::not(1u32 << (7-len));// clear the next bit after it

            (parts.to_le_bytes(), len)
        }
    }


    fn from_utf8_slice_start(src: &[u8]) -> Result<(Self,usize),Utf8Error> {
        let first = match src.first() {
            Some(first) => *first,
            None => return Err(Utf8Error{ kind: TooFewBytes }),
        };
        let bytes = match first.extra_utf8_bytes() {
            Err(e)    => return Err(e),
            Ok(0)     => return Ok((first as char, 1)),
            Ok(extra) if extra >= src.len()
                      => return Err(Utf8Error{ kind: TooFewBytes }),
            Ok(extra) => &src[..=extra],
        };
        if bytes.iter().skip(1).any(|&b| (b >> 6) != 0b10 ) {
            Err(Utf8Error{ kind: InterruptedSequence })
        } else if overlong(bytes[0], bytes[1]) {
            Err(Utf8Error{ kind: OverlongEncoding })
        } else {
            match char::from_u32_detailed(merge_nonascii_unchecked_utf8(bytes)) {
                Ok(c) => Ok((c, bytes.len())),
                Err(CodepointError::Utf16Reserved) => {
                    Err(Utf8Error{ kind: Utf16ReservedCodepoint })
                },
                Err(CodepointError::TooHigh) => Err(Utf8Error{ kind: TooHighCodepoint }),
            }
        }
    }

    fn from_utf8_array(utf8: [u8; 4]) -> Result<Self,Utf8Error> {
        let src = match utf8[0].extra_utf8_bytes() {
            Err(error) => return Err(error),
            Ok(0)      => return Ok(utf8[0] as char),
            Ok(extra)  => &utf8[..=extra],
        };
        if src[1..].iter().any(|&b| (b >> 6) != 0b10 ) {
            Err(Utf8Error{ kind: InterruptedSequence })
        } else if overlong(utf8[0], utf8[1]) {
            Err(Utf8Error{ kind: OverlongEncoding })
        } else {
            match char::from_u32_detailed(merge_nonascii_unchecked_utf8(src)) {
                Ok(c) => Ok(c),
                Err(CodepointError::Utf16Reserved) => {
                    Err(Utf8Error{ kind: Utf16ReservedCodepoint })
                },
                Err(CodepointError::TooHigh) => Err(Utf8Error{ kind: TooHighCodepoint }),
            }
        }
    }

    unsafe fn from_utf8_exact_slice_unchecked(src: &[u8]) -> Self {
        unsafe {
            if src.len() == 1 {
                src[0] as char
            } else {
                char::from_u32_unchecked(merge_nonascii_unchecked_utf8(src))
            }
        }
    }



      //////////
     //UTF-16//
    //////////

    fn to_utf16(self) -> Utf16Char {
        Utf16Char::from(self)
    }
    fn iter_utf16_units(self) -> Utf16Iterator {
        self.to_utf16().into_iter()
    }

    fn to_utf16_array(self) -> [u16;2] {
        let (first, second) = self.to_utf16_tuple();
        [first, second.unwrap_or(0)]
    }
    fn to_utf16_tuple(self) -> (u16, Option<u16>) {
        if self <= '\u{ffff}' {// single
            (self as u16, None)
        } else {// double
            let c = self as u32 - 0x_01_00_00;
            let high = 0x_d8_00 + (c >> 10);
            let low = 0x_dc_00 + (c & 0x_03_ff);
            (high as u16,  Some(low as u16))
        }
    }


    fn from_utf16_slice_start(src: &[u16]) -> Result<(Self,usize), Utf16SliceError> {
        use crate::errors::Utf16SliceError::*;
        unsafe {match (src.get(0), src.get(1)) {
            (Some(&u @ 0x00_00..=0xd7_ff), _) |
            (Some(&u @ 0xe0_00..=0xff_ff), _)
                => Ok((char::from_u32_unchecked(u as u32), 1)),
            (Some(0xdc_00..=0xdf_ff), _) => Err(FirstIsTrailingSurrogate),
            (None, _) => Err(EmptySlice),
            (Some(&f @ 0xd8_00..=0xdb_ff), Some(&s @ 0xdc_00..=0xdf_ff))
                => Ok((char::from_utf16_tuple_unchecked((f, Some(s))), 2)),
            (Some(0xd8_00..=0xdb_ff), Some(_)) => Err(SecondIsNotTrailingSurrogate),
            (Some(0xd8_00..=0xdb_ff), None) => Err(MissingSecond),
        }}
    }

    fn from_utf16_array(utf16: [u16;2]) -> Result<Self, Utf16ArrayError> {
        use crate::errors::Utf16ArrayError::*;
        if let Some(c) = char::from_u32(utf16[0] as u32) {
            Ok(c) // single
        } else if utf16[0] < 0xdc_00  &&  utf16[1] & 0xfc_00 == 0xdc_00 {
            // correct surrogate pair
            Ok(combine_surrogates(utf16[0], utf16[1]))
        } else if utf16[0] < 0xdc_00 {
            Err(SecondIsNotTrailingSurrogate)
        } else {
            Err(FirstIsTrailingSurrogate)
        }
    }
    fn from_utf16_tuple(utf16: (u16, Option<u16>)) -> Result<Self, Utf16TupleError> {
        unsafe {
            match Utf16Char::validate_tuple(utf16) {
                Ok(()) => Ok(Self::from_utf16_tuple_unchecked(utf16)),
                Err(e) => Err(e),
            }
        }
    }

    fn from_utf16_array_unchecked(utf16: [u16;2]) -> Self {
        // treat any array with a surrogate value in [0] as a surrogate because
        // combine_surrogates() is safe.
        // `(utf16[0] & 0xf800) == 0xd80` might not be quite as fast as
        // `utf16[1] != 0`, but avoiding the potential for UB is worth it
        // since the conversion isn't zero-cost in either case.
        char::from_u32(utf16[0] as u32)
            .unwrap_or_else(|| combine_surrogates(utf16[0], utf16[1]) )
    }
    unsafe fn from_utf16_tuple_unchecked(utf16: (u16, Option<u16>)) -> Self {
        unsafe {
            match utf16.1 {
                Some(second) => combine_surrogates(utf16.0, second),
                None         => char::from_u32_unchecked(utf16.0 as u32)
            }
        }
    }


    fn from_u32_detailed(c: u32) -> Result<Self,CodepointError> {
        match char::from_u32(c) {
            Some(c) => Ok(c),
            None if c > 0x10_ff_ff => Err(CodepointError::TooHigh),
            None => Err(CodepointError::Utf16Reserved),
        }
    }
}

// Adapted from https://www.cl.cam.ac.uk/~mgk25/ucs/utf8_check.c
fn overlong(first: u8, second: u8) -> bool {
    if first < 0x80 {
        false
    } else if (first & 0xe0) == 0xc0 {
        (first & 0xfe) == 0xc0
    } else if (first & 0xf0) == 0xe0 {
        first == 0xe0 && (second & 0xe0) == 0x80
    } else {
        first == 0xf0 && (second & 0xf0) == 0x80
    }
}

/// Decodes the codepoint represented by a multi-byte UTF-8 sequence.
///
/// Does not check that the codepoint is valid,
/// and returns `u32` because casting invalid codepoints to `char` is insta UB.
fn merge_nonascii_unchecked_utf8(src: &[u8]) -> u32 {
    let mut c = src[0] as u32 & (0x7f >> src.len());
    for b in &src[1..] {
        c = (c << 6)  |  (b & 0b0011_1111) as u32;
    }
    c
}

/// Create a `char` from a leading and a trailing surrogate.
///
/// This function is safe because it ignores the six most significant bits of
/// each argument and always produces a codepoint in `0x01_00_00..=0x10_ff_ff`.
fn combine_surrogates(first: u16,  second: u16) -> char {
    unsafe {
        let high = (first & 0x_03_ff) as u32;
        let low = (second & 0x_03_ff) as u32;
        let c = ((high << 10) | low) + 0x_01_00_00; // no, the constant can't be or'd in
        char::from_u32_unchecked(c)
    }
}



/// Adds `.utf8chars()` and `.utf16chars()` iterator constructors to `&str`.
pub trait StrExt: AsRef<str> {
    /// Equivalent to `.chars()` but produces `Utf8Char`s.
    fn utf8chars(&self) -> Utf8Chars;
    /// Equivalent to `.chars()` but produces `Utf16Char`s.
    fn utf16chars(&self) -> Utf16Chars;
    /// Equivalent to `.char_indices()` but produces `Utf8Char`s.
    fn utf8char_indices(&self) -> Utf8CharIndices;
    /// Equivalent to `.char_indices()` but produces `Utf16Char`s.
    fn utf16char_indices(&self) -> Utf16CharIndices;
}

impl StrExt for str {
    fn utf8chars(&self) -> Utf8Chars {
        Utf8Chars::from(self)
    }
    fn utf16chars(&self) -> Utf16Chars {
        Utf16Chars::from(self)
    }
    fn utf8char_indices(&self) -> Utf8CharIndices {
        Utf8CharIndices::from(self)
    }
    fn utf16char_indices(&self) -> Utf16CharIndices {
        Utf16CharIndices::from(self)
    }
}

#[cfg(feature="ascii")]
impl StrExt for AsciiStr {
    fn utf8chars(&self) -> Utf8Chars {
        Utf8Chars::from(self.as_str())
    }
    fn utf16chars(&self) -> Utf16Chars {
        Utf16Chars::from(self.as_str())
    }
    fn utf8char_indices(&self) -> Utf8CharIndices {
        Utf8CharIndices::from(self.as_str())
    }
    fn utf16char_indices(&self) -> Utf16CharIndices {
        Utf16CharIndices::from(self.as_str())
    }
}



/// Iterator methods that convert between `u8`s and `Utf8Char` or `u16`s and `Utf16Char`
///
/// All the iterator adapters also accept iterators that produce references of
/// the type they convert from.
pub trait IterExt: Iterator+Sized {
    /// Converts an iterator of `Utf8Char`s or `&Utf8Char`s to an iterator of
    /// `u8`s.
    ///
    /// Has the same effect as `.flat_map()` or `.flatten()`, but the returned
    /// iterator is ~40% faster.
    ///
    /// The iterator also implements `Read`
    /// (when the `std` feature isn't disabled).  
    /// Reading will never produce an error, and calls to `.read()` and `.next()`
    /// can be mixed.
    ///
    /// The exact number of bytes cannot be known in advance, but `size_hint()`
    /// gives the possible range.
    /// (min: all remaining characters are ASCII, max: all require four bytes)
    ///
    /// # Examples
    ///
    /// From iterator of values:
    ///
    /// ```
    /// use encode_unicode::{IterExt, StrExt};
    ///
    /// let iterator = "foo".utf8chars();
    /// let mut bytes = [0; 4];
    /// iterator.to_bytes().zip(&mut bytes).for_each(|(b,dst)| *dst = b );
    /// assert_eq!(&bytes, b"foo\0");
    /// ```
    ///
    /// From iterator of references:
    ///
    #[cfg_attr(feature="std", doc=" ```")]
    #[cfg_attr(not(feature="std"), doc=" ```no_compile")]
    /// use encode_unicode::{IterExt, StrExt, Utf8Char};
    ///
    /// let chars: Vec<Utf8Char> = "💣 bomb 💣".utf8chars().collect();
    /// let bytes: Vec<u8> = chars.iter().to_bytes().collect();
    /// let flat_map: Vec<u8> = chars.iter().cloned().flatten().collect();
    /// assert_eq!(bytes, flat_map);
    /// ```
    ///
    /// `Read`ing from it:
    ///
    #[cfg_attr(feature="std", doc=" ```")]
    #[cfg_attr(not(feature="std"), doc=" ```no_compile")]
    /// use encode_unicode::{IterExt, StrExt};
    /// use std::io::Read;
    ///
    /// let s = "Ååh‽";
    /// assert_eq!(s.len(), 8);
    /// let mut buf = [b'E'; 9];
    /// let mut reader = s.utf8chars().to_bytes();
    /// assert_eq!(reader.read(&mut buf[..]).unwrap(), 8);
    /// assert_eq!(reader.read(&mut buf[..]).unwrap(), 0);
    /// assert_eq!(&buf[..8], s.as_bytes());
    /// assert_eq!(buf[8], b'E');
    /// ```
    fn to_bytes(self) -> Utf8CharSplitter<Self::Item,Self> where Self::Item: Borrow<Utf8Char>;

    /// Converts an iterator of `Utf16Char` (or `&Utf16Char`) to an iterator of
    /// `u16`s.
    ///
    /// Has the same effect as `.flat_map()` or `.flatten()`, but the returned
    /// iterator is about twice as fast.
    ///
    /// The exact number of units cannot be known in advance, but `size_hint()`
    /// gives the possible range.
    ///
    /// # Examples
    ///
    /// From iterator of values:
    ///
    /// ```
    /// use encode_unicode::{IterExt, StrExt};
    ///
    /// let iterator = "foo".utf16chars();
    /// let mut units = [0; 4];
    /// iterator.to_units().zip(&mut units).for_each(|(u,dst)| *dst = u );
    ///
    /// assert_eq!(units, ['f' as u16, 'o' as u16, 'o' as u16, 0]);
    /// ```
    ///
    /// From iterator of references:
    ///
    #[cfg_attr(feature="std", doc=" ```")]
    #[cfg_attr(not(feature="std"), doc=" ```no_compile")]
    /// use encode_unicode::{IterExt, StrExt, Utf16Char};
    ///
    /// // (💣 takes two units)
    /// let chars: Vec<Utf16Char> = "💣 bomb 💣".utf16chars().collect();
    /// let units: Vec<u16> = chars.iter().to_units().collect();
    /// let flat_map: Vec<u16> = chars.iter().flat_map(|u16c| *u16c ).collect();
    ///
    /// assert_eq!(units, flat_map);
    /// ```
    fn to_units(self) -> Utf16CharSplitter<Self::Item,Self> where Self::Item: Borrow<Utf16Char>;

    /// Decodes bytes as UTF-8 and groups them into `Utf8Char`s
    ///
    /// When errors (invalid values or sequences) are encountered,
    /// it continues with the byte right after the start of the error sequence.  
    /// This is neither the most intelligent choiche (sometimes it is guaranteed to
    ///  produce another error), nor the easiest to implement, but I believe it to
    /// be the most predictable.
    /// It also means that ASCII characters are never hidden by errors.
    ///
    /// # Examples
    ///
    /// Replace all errors with u+FFFD REPLACEMENT_CHARACTER:
    /// ```
    /// use encode_unicode::{Utf8Char, IterExt};
    ///
    /// let mut buf = [b'\0'; 255];
    /// let len = b"foo\xCFbar".iter()
    ///     .to_utf8chars()
    ///     .flat_map(|r| r.unwrap_or(Utf8Char::from('\u{FFFD}')) )
    ///     .zip(&mut buf[..])
    ///     .map(|(byte, dst)| *dst = byte )
    ///     .count();
    ///
    /// assert_eq!(&buf[..len], "foo\u{FFFD}bar".as_bytes());
    /// ```
    ///
    /// Collect everything up until the first error into a string:
    #[cfg_attr(feature="std", doc=" ```")]
    #[cfg_attr(not(feature="std"), doc=" ```no_compile")]
    /// use encode_unicode::iterator::Utf8CharMerger;
    /// let mut good = String::new();
    /// for r in Utf8CharMerger::from(b"foo\xcc\xbbbar\xcc\xddbaz") {
    ///     if let Ok(uc) = r {
    ///         good.push_str(uc.as_str());
    ///     } else {
    ///         break;
    ///     }
    /// }
    /// assert_eq!(good, "foo̻bar");
    /// ```
    ///
    /// Abort decoding on error:
    #[cfg_attr(feature="std", doc=" ```")]
    #[cfg_attr(not(feature="std"), doc=" ```no_compile")]
    /// use encode_unicode::{IterExt, Utf8Char};
    /// use encode_unicode::error::{Utf8Error, Utf8ErrorKind};
    ///
    /// let result = b"ab\0\xe0\xbc\xa9 \xf3\x80\x77".iter()
    ///     .to_utf8chars()
    ///     .collect::<Result<String,Utf8Error>>();
    ///
    /// assert_eq!(result.unwrap_err().kind(), Utf8ErrorKind::InterruptedSequence);
    /// ```
    fn to_utf8chars(self) -> Utf8CharMerger<Self::Item,Self> where Self::Item: Borrow<u8>;

    /// Decodes bytes as UTF-16 and groups them into `Utf16Char`s
    ///
    /// When errors (unmatched leading surrogates or unexpected trailing surrogates)
    /// are encountered, an error is produced for every unit.
    ///
    /// # Examples
    ///
    /// Replace errors with '�':
    #[cfg_attr(feature="std", doc=" ```")]
    #[cfg_attr(not(feature="std"), doc=" ```no_compile")]
    /// use encode_unicode::{IterExt, Utf16Char};
    ///
    /// let slice = &['a' as u16, 0xdf00, 0xd83c, 0xdca0][..];
    /// let string = slice.iter()
    ///     .to_utf16chars()
    ///     .map(|r| r.unwrap_or(Utf16Char::from('\u{fffd}')) ) // REPLACEMENT_CHARACTER
    ///     .collect::<String>();
    ///
    /// assert_eq!(string, "a�🂠");
    /// ```
    ///
    /// ```
    /// use encode_unicode::{IterExt, Utf16Char};
    /// use encode_unicode::error::Utf16PairError::*;
    ///
    /// let slice = [0xdcba, 0xdeff, 0xd8be, 0xdeee, 'Y' as u16, 0xdab1, 0xdab1];
    /// let mut iter = slice.iter().to_utf16chars();
    /// assert_eq!(iter.size_hint(), (3, Some(7)));
    /// assert_eq!(iter.next(), Some(Err(UnexpectedTrailingSurrogate)));
    /// assert_eq!(iter.next(), Some(Err(UnexpectedTrailingSurrogate)));
    /// assert_eq!(iter.next(), Some(Ok(Utf16Char::from('\u{3faee}'))));
    /// assert_eq!(iter.next(), Some(Ok(Utf16Char::from('Y'))));
    /// assert_eq!(iter.next(), Some(Err(UnmatchedLeadingSurrogate)));
    /// assert_eq!(iter.next(), Some(Err(Incomplete)));
    /// assert_eq!(iter.into_remaining_units().next(), None);
    /// ```
    ///
    /// Search for a codepoint and return the codepoint index of the first match:
    /// ```
    /// use encode_unicode::{IterExt, Utf16Char};
    ///
    /// let position = [0xd875, 0xdd4f, '≈' as u16, '2' as u16].iter()
    ///     .to_utf16chars()
    ///     .position(|r| r == Ok(Utf16Char::from('≈')) );
    ///
    /// assert_eq!(position, Some(1));
    /// ```
    fn to_utf16chars(self) -> Utf16CharMerger<Self::Item,Self> where Self::Item: Borrow<u16>;
}

impl<I:Iterator> IterExt for I {
    fn to_bytes(self) -> Utf8CharSplitter<Self::Item,Self> where Self::Item: Borrow<Utf8Char> {
        Utf8CharSplitter::from(self)
    }
    fn to_units(self) -> Utf16CharSplitter<Self::Item,Self> where Self::Item: Borrow<Utf16Char> {
        Utf16CharSplitter::from(self)
    }
    fn to_utf8chars(self) -> Utf8CharMerger<Self::Item,Self> where Self::Item: Borrow<u8> {
        Utf8CharMerger::from(self)
    }
    fn to_utf16chars(self) -> Utf16CharMerger<Self::Item,Self> where Self::Item: Borrow<u16> {
        Utf16CharMerger::from(self)
    }
}


/// Methods for iterating over `u8` and `u16` slices as UTF-8 or UTF-16 characters.
///
/// The iterators are slightly faster than the similar methods in [`IterExt`](trait.IterExt.html)
/// because they con "push back" items for free after errors and don't need a
/// separate buffer that must be checked on every call to `.next()`.
pub trait SliceExt: Index<RangeFull> {
    /// Decode `u8` slices as UTF-8 and iterate over the codepoints as `Utf8Char`s,
    ///
    /// # Examples
    ///
    /// Get the index and error type of the first error:
    #[cfg_attr(feature="std", doc=" ```")]
    #[cfg_attr(not(feature="std"), doc=" ```no_compile")]
    /// use encode_unicode::{SliceExt, Utf8Char, error::Utf8ErrorKind};
    ///
    /// let slice = b"ab\0\xe0\xbc\xa9 \xf3\x80\x77";
    /// let result = slice.utf8char_indices()
    ///     .map(|(offset,r,length)| r.map_err(|e| (offset,e.kind(),length) ) )
    ///     .collect::<Result<String,(usize,Utf8ErrorKind,usize)>>();
    ///
    /// assert_eq!(result, Err((7, Utf8ErrorKind::TooFewBytes, 1)));
    /// ```
    ///
    /// ```
    /// use encode_unicode::{SliceExt, Utf8Char};
    /// use std::error::Error;
    ///
    /// let slice = b"\xf0\xbf\xbf\xbfXY\xdd\xbb\xe1\x80\x99quux123";
    /// let mut fixed_size = [Utf8Char::default(); 8];
    /// for (cp_i, (byte_index, r, _)) in slice.utf8char_indices().enumerate().take(8) {
    ///     match r {
    ///         Ok(u8c) => fixed_size[cp_i] = u8c,
    ///         Err(e) => panic!("Invalid codepoint at index {} ({})", cp_i, e),
    ///     }
    /// }
    /// let chars = ['\u{3ffff}', 'X', 'Y', '\u{77b}', '\u{1019}', 'q', 'u', 'u'];
    /// assert_eq!(fixed_size, chars);
    /// ```
    ///
    #[cfg_attr(feature="std", doc=" ```")]
    #[cfg_attr(not(feature="std"), doc=" ```no_compile")]
    /// use encode_unicode::{SliceExt, Utf8Char, error::Utf8ErrorKind};
    ///
    /// let bytes = b"\xfa-\xf4\x8f\xee\xa1\x8f-\xed\xa9\x87\xf0\xcc\xbb";
    /// let mut errors = Vec::new();
    /// let mut lengths = Vec::new();
    /// let mut string = String::new();
    /// for (offset,result,length) in bytes.utf8char_indices() {
    ///     lengths.push((offset,length));
    ///     let c = result.unwrap_or_else(|error| {
    ///         errors.push((offset, error.kind()));
    ///         Utf8Char::from('\u{fffd}') // replacement character
    ///     });
    ///     string.push_str(c.as_str());
    /// }
    ///
    /// assert_eq!(string, "�-��\u{e84f}-����\u{33b}");
    /// assert_eq!(lengths, [(0,1), (1,1), (2,1), (3,1), (4,3), (7,1),
    ///                      (8,1), (9,1), (10,1), (11,1), (12,2)]);
    /// assert_eq!(errors, [
    ///     ( 0, Utf8ErrorKind::NonUtf8Byte),
    ///     ( 2, Utf8ErrorKind::InterruptedSequence),
    ///     ( 3, Utf8ErrorKind::UnexpectedContinuationByte),
    ///     ( 8, Utf8ErrorKind::Utf16ReservedCodepoint),
    ///     ( 9, Utf8ErrorKind::UnexpectedContinuationByte),
    ///     (10, Utf8ErrorKind::UnexpectedContinuationByte),
    ///     (11, Utf8ErrorKind::TooFewBytes), // (but it was not the last element returned!)
    /// ]);
    /// ```
    fn utf8char_indices(&self) -> Utf8CharDecoder where Self::Output: Borrow<[u8]>;


    /// Decode `u16` slices as UTF-16 and iterate over the codepoints as `Utf16Char`s,
    ///
    /// The iterator produces `(usize,Result<Utf16Char,Utf16Error>,usize)`,
    /// and the slice is validated as you go.
    ///
    /// The first `usize` contains the offset from the start of the slice and
    /// the last `usize` contains the length of the codepoint or error.
    /// The length is either 1 or 2, and always 1 for errors.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature="std", doc=" ```")]
    #[cfg_attr(not(feature="std"), doc=" ```no_compile")]
    /// use encode_unicode::{SliceExt, Utf8Char};
    ///
    /// let slice = &['a' as u16, 0xdf00, 0xd83c, 0xdca0][..];
    /// let mut errors = Vec::new();
    /// let string = slice.utf16char_indices().map(|(offset,r,_)| match r {
    ///     Ok(u16c) => Utf8Char::from(u16c),
    ///     Err(_) => {
    ///         errors.push(offset);
    ///         Utf8Char::from('\u{fffd}') // REPLACEMENT_CHARACTER
    ///     }
    /// }).collect::<String>();
    ///
    /// assert_eq!(string, "a�🂠");
    /// assert_eq!(errors, [1]);
    /// ```
    ///
    /// Search for a codepoint and return its unit and codepoint index.
    /// ```
    /// use encode_unicode::{SliceExt, Utf16Char};
    ///
    /// let slice = [0xd875,/*'𝕏'*/ 0xdd4f, '≈' as u16, '2' as u16];
    /// let position = slice.utf16char_indices()
    ///     .enumerate()
    ///     .find(|&(_,(_,r,_))| r == Ok(Utf16Char::from('≈')) )
    ///     .map(|(codepoint, (offset, _, _))| (codepoint, offset) );
    ///
    /// assert_eq!(position, Some((1,2)));
    /// ```
    ///
    /// Error types:
    /// ```
    /// use encode_unicode::{SliceExt, Utf16Char};
    /// use encode_unicode::error::Utf16PairError::*;
    ///
    /// let slice = [0xdcba, 0xdeff, 0xd8be, 0xdeee, 'λ' as u16, 0xdab1, 0xdab1];
    /// let mut iter = slice.utf16char_indices();
    /// assert_eq!(iter.next(), Some((0, Err(UnexpectedTrailingSurrogate), 1)));
    /// assert_eq!(iter.next(), Some((1, Err(UnexpectedTrailingSurrogate), 1)));
    /// assert_eq!(iter.next(), Some((2, Ok(Utf16Char::from('\u{3faee}')), 2)));
    /// assert_eq!(iter.next(), Some((4, Ok(Utf16Char::from('λ')), 1)));
    /// assert_eq!(iter.next(), Some((5, Err(UnmatchedLeadingSurrogate), 1)));
    /// assert_eq!(iter.next(), Some((6, Err(Incomplete), 1)));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.as_slice(), [])
    /// ```
    fn utf16char_indices(&self) -> Utf16CharDecoder where Self::Output: Borrow<[u16]>;
}

impl<S: ?Sized+Index<RangeFull>> SliceExt for S {
    fn utf8char_indices(&self) -> Utf8CharDecoder where Self::Output: Borrow<[u8]> {
        Utf8CharDecoder::from(self[..].borrow())
    }
    fn utf16char_indices(&self) -> Utf16CharDecoder where Self::Output: Borrow<[u16]> {
        Utf16CharDecoder::from(self[..].borrow())
    }
}
