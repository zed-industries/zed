use crate::formatting::{hex_as_ascii, HexFormatting};

#[cfg(any(test, feature = "fmt"))]
pub(crate) const fn char_display_len(c: char) -> usize {
    match c as u32 {
        0..=127 => 1,
        0x80..=0x7FF => 2,
        0x800..=0xFFFF => 3,
        0x10000..=u32::MAX => 4,
    }
}

#[cfg(any(test, feature = "fmt"))]
pub(crate) const fn char_debug_len(c: char) -> usize {
    let inner = match c {
        '\t' | '\r' | '\n' | '\\' | '\'' | '\"' => 2,
        '\x00'..='\x1F' => 4,
        _ => char_display_len(c),
    };
    inner + 2
}

const fn char_to_utf8(char: char) -> ([u8; 4], usize) {
    let u32 = char as u32;
    match u32 {
        0..=127 => ([u32 as u8, 0, 0, 0], 1),
        0x80..=0x7FF => {
            let b0 = 0b1100_0000 | (u32 >> 6) as u8;
            let b1 = 0b1000_0000 | (u32 & 0b0011_1111) as u8;
            ([b0, b1, 0, 0], 2)
        }
        0x800..=0xFFFF => {
            let b0 = 0b1110_0000 | (u32 >> 12) as u8;
            let b1 = 0b1000_0000 | ((u32 >> 6) & 0b0011_1111) as u8;
            let b2 = 0b1000_0000 | (u32 & 0b0011_1111) as u8;
            ([b0, b1, b2, 0], 3)
        }
        0x10000..=u32::MAX => {
            let b0 = 0b1111_0000 | (u32 >> 18) as u8;
            let b1 = 0b1000_0000 | ((u32 >> 12) & 0b0011_1111) as u8;
            let b2 = 0b1000_0000 | ((u32 >> 6) & 0b0011_1111) as u8;
            let b3 = 0b1000_0000 | (u32 & 0b0011_1111) as u8;
            ([b0, b1, b2, b3], 4)
        }
    }
}

pub(crate) const fn char_to_display(char: char) -> FmtChar {
    let ([b0, b1, b2, b3], len) = char_to_utf8(char);
    FmtChar {
        encoded: [b0, b1, b2, b3, 0, 0],
        len: len as u8,
    }
}

pub(crate) const fn char_to_debug(c: char) -> FmtChar {
    let ([b0, b1, b2, b3], len) = match c {
        '\t' => (*br#"\t  "#, 2),
        '\r' => (*br#"\r  "#, 2),
        '\n' => (*br#"\n  "#, 2),
        '\\' => (*br#"\\  "#, 2),
        '\'' => (*br#"\'  "#, 2),
        '\"' => (*br#"\"  "#, 2),
        '\x00'..='\x1F' => {
            let n = c as u8;
            (
                [
                    b'\\',
                    b'x',
                    hex_as_ascii(n >> 4, HexFormatting::Upper),
                    hex_as_ascii(n & 0b1111, HexFormatting::Upper),
                ],
                4,
            )
        }
        _ => char_to_utf8(c),
    };

    let mut encoded = [b'\'', b0, b1, b2, b3, 0];
    encoded[len + 1] = b'\'';

    FmtChar {
        encoded,
        len: (len as u8) + 2,
    }
}

#[derive(Copy, Clone)]
pub struct FmtChar {
    encoded: [u8; 6],
    len: u8,
}

impl FmtChar {
    /// Array which contains the pre-len display/debug-formatted  `char`,
    /// only `&self.encoded[][..self.len()]` should be copied.
    pub const fn encoded(&self) -> &[u8; 6] {
        &self.encoded
    }

    pub const fn len(&self) -> usize {
        self.len as usize
    }

    pub(crate) const fn as_bytes(&self) -> &[u8] {
        #[cfg(not(feature = "rust_1_64"))]
        {
            match self.len() {
                1 => {
                    let [ret @ .., _, _, _, _, _] = &self.encoded;
                    ret
                }
                2 => {
                    let [ret @ .., _, _, _, _] = &self.encoded;
                    ret
                }
                3 => {
                    let [ret @ .., _, _, _] = &self.encoded;
                    ret
                }
                4 => {
                    let [ret @ .., _, _] = &self.encoded;
                    ret
                }
                5 => {
                    let [ret @ .., _] = &self.encoded;
                    ret
                }
                6 => &self.encoded,
                x => [/*bug WTF*/][x],
            }
        }

        #[cfg(feature = "rust_1_64")]
        {
            ::konst::slice::slice_up_to(&self.encoded, self.len())
        }
    }
}

#[cfg(all(test, not(miri)))]
mod tests;
