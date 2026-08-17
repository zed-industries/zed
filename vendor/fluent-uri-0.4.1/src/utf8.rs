//! UTF-8 utilities taken from `core::str`, Rust 1.81.

#[inline]
const fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}

#[inline]
const fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}

#[inline]
pub const fn next_code_point(bytes: &[u8], i: usize) -> (u32, usize) {
    let x = bytes[i];
    if x < 128 {
        return (x as u32, 1);
    }

    let init = utf8_first_byte(x, 2);
    let y = bytes[i + 1];
    if x < 0xE0 {
        (utf8_acc_cont_byte(init, y), 2)
    } else {
        let z = bytes[i + 2];
        let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
        if x < 0xF0 {
            ((init << 12) | y_z, 3)
        } else {
            let w = bytes[i + 3];
            (((init & 7) << 18) | utf8_acc_cont_byte(y_z, w), 4)
        }
    }
}

#[cfg(feature = "alloc")]
const UTF8_CHAR_WIDTH: &[u8; 256] = &[
    // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 1
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 2
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 3
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 5
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 7
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // C
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // D
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // E
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F
];

#[cfg(feature = "alloc")]
#[inline]
const fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}

const CONT_MASK: u8 = 0b0011_1111;

#[cfg(feature = "alloc")]
pub struct Utf8Chunk<'a> {
    valid: &'a str,
    invalid: &'a [u8],
    incomplete: bool,
}

#[cfg(feature = "alloc")]
impl<'a> Utf8Chunk<'a> {
    pub fn valid(&self) -> &'a str {
        self.valid
    }

    pub fn invalid(&self) -> &'a [u8] {
        self.invalid
    }

    pub fn incomplete(&self) -> bool {
        self.incomplete
    }
}

#[cfg(feature = "alloc")]
pub struct Utf8Chunks<'a> {
    source: &'a [u8],
}

#[cfg(feature = "alloc")]
impl<'a> Utf8Chunks<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { source: bytes }
    }
}

#[cfg(feature = "alloc")]
impl<'a> Iterator for Utf8Chunks<'a> {
    type Item = Utf8Chunk<'a>;

    fn next(&mut self) -> Option<Utf8Chunk<'a>> {
        if self.source.is_empty() {
            return None;
        }

        const TAG_CONT_U8: u8 = 128;

        let mut incomplete = false;
        let mut safe_get = |i| match self.source.get(i) {
            Some(x) => *x,
            None => {
                incomplete = true;
                0
            }
        };

        let mut i = 0;
        let mut valid_up_to = 0;
        while i < self.source.len() {
            let byte = self.source[i];
            i += 1;

            if byte >= 128 {
                let w = utf8_char_width(byte);

                match w {
                    2 => {
                        if safe_get(i) & 192 != TAG_CONT_U8 {
                            break;
                        }
                        i += 1;
                    }
                    3 => {
                        match (byte, safe_get(i)) {
                            (0xE0, 0xA0..=0xBF) => (),
                            (0xE1..=0xEC, 0x80..=0xBF) => (),
                            (0xED, 0x80..=0x9F) => (),
                            (0xEE..=0xEF, 0x80..=0xBF) => (),
                            _ => break,
                        }
                        i += 1;
                        if safe_get(i) & 192 != TAG_CONT_U8 {
                            break;
                        }
                        i += 1;
                    }
                    4 => {
                        match (byte, safe_get(i)) {
                            (0xF0, 0x90..=0xBF) => (),
                            (0xF1..=0xF3, 0x80..=0xBF) => (),
                            (0xF4, 0x80..=0x8F) => (),
                            _ => break,
                        }
                        i += 1;
                        if safe_get(i) & 192 != TAG_CONT_U8 {
                            break;
                        }
                        i += 1;
                        if safe_get(i) & 192 != TAG_CONT_U8 {
                            break;
                        }
                        i += 1;
                    }
                    _ => break,
                }
            }

            valid_up_to = i;
        }

        let (inspected, remaining) = self.source.split_at(i);
        self.source = remaining;

        let (valid, invalid) = inspected.split_at(valid_up_to);

        Some(Utf8Chunk {
            valid: core::str::from_utf8(valid).unwrap(),
            invalid,
            incomplete,
        })
    }
}
