// Copyright 2012-2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Original authors: alexchrichton, bluss

// UTF-8 ranges and tags for encoding characters
const TAG_CONT: u8    = 0b1000_0000;
const TAG_TWO_B: u8   = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8  = 0b1111_0000;
const MAX_ONE_B: u32   =     0x80;
const MAX_TWO_B: u32   =    0x800;
const MAX_THREE_B: u32 =  0x10000;

/// Placeholder
pub struct EncodeUtf8Error;

/// Encode a char into buf using UTF-8.
///
/// On success, return the byte length of the encoding (1, 2, 3 or 4).<br>
/// On error, return `EncodeUtf8Error` if the buffer was too short for the char.
///
/// Safety: `ptr` must be writable for `len` bytes.
#[inline]
pub unsafe fn encode_utf8(ch: char, ptr: *mut u8, len: usize) -> Result<usize, EncodeUtf8Error>
{
    let code = ch as u32;
    if code < MAX_ONE_B && len >= 1 {
        ptr.add(0).write(code as u8);
        return Ok(1);
    } else if code < MAX_TWO_B && len >= 2 {
        ptr.add(0).write((code >> 6 & 0x1F) as u8 | TAG_TWO_B);
        ptr.add(1).write((code & 0x3F) as u8 | TAG_CONT);
        return Ok(2);
    } else if code < MAX_THREE_B && len >= 3 {
        ptr.add(0).write((code >> 12 & 0x0F) as u8 | TAG_THREE_B);
        ptr.add(1).write((code >>  6 & 0x3F) as u8 | TAG_CONT);
        ptr.add(2).write((code & 0x3F) as u8 | TAG_CONT);
        return Ok(3);
    } else if len >= 4 {
        ptr.add(0).write((code >> 18 & 0x07) as u8 | TAG_FOUR_B);
        ptr.add(1).write((code >> 12 & 0x3F) as u8 | TAG_CONT);
        ptr.add(2).write((code >>  6 & 0x3F) as u8 | TAG_CONT);
        ptr.add(3).write((code & 0x3F) as u8 | TAG_CONT);
        return Ok(4);
    };
    Err(EncodeUtf8Error)
}


#[test]
#[cfg_attr(miri, ignore)] // Miri is too slow
fn test_encode_utf8() {
    // Test that all codepoints are encoded correctly
    let mut data = [0u8; 16];
    for codepoint in 0..=(std::char::MAX as u32) {
        if let Some(ch) = std::char::from_u32(codepoint) {
            for elt in &mut data { *elt = 0; }
            let ptr = data.as_mut_ptr();
            let len = data.len();
            unsafe {
                let res = encode_utf8(ch, ptr, len).ok().unwrap();
                assert_eq!(res, ch.len_utf8());
            }
            let string = std::str::from_utf8(&data).unwrap();
            assert_eq!(string.chars().next(), Some(ch));
        }
    }
}

#[test]
fn test_encode_utf8_oob() {
    // test that we report oob if the buffer is too short
    let mut data = [0u8; 16];
    let chars = ['a', 'α', '�', '𐍈'];
    for (len, &ch) in (1..=4).zip(&chars) {
        assert_eq!(len, ch.len_utf8(), "Len of ch={}", ch);
        let ptr = data.as_mut_ptr();
        unsafe {
            assert!(matches::matches!(encode_utf8(ch, ptr, len - 1), Err(_)));
            assert!(matches::matches!(encode_utf8(ch, ptr, len), Ok(_)));
        }
    }
}

