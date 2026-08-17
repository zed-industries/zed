use core::fmt::{self, Debug};

macro_rules! for_range_inc {
    ($current:ident in $start:expr, $end:expr => $($code:tt)*) => {
        let mut $current = $start;
        let end = $end;

        while $current <= end {
            $($code)*

            $current+=1;
        }
    };
}

use core::ops::Range;

#[derive(Copy, Clone)]
struct ByteKind(u8);

impl Debug for ByteKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match () {
            _ if self.0 == Self::Other.0 => "Other",
            _ if self.0 == Self::Number.0 => "Number",
            _ if self.0 == Self::LowerCase.0 => "LowerCase",
            _ if self.0 == Self::UpperCase.0 => "UpperCase",
            _ if self.0 == Self::NonAscii.0 => "NonAscii",
            _ => unreachable!(),
        })
    }
}

#[allow(non_upper_case_globals)]
impl ByteKind {
    const Other: Self = Self(0b0001);
    const Number: Self = Self(0b0010);
    const LowerCase: Self = Self(0b0100);
    const UpperCase: Self = Self(0b1000);
    const Alphabetic: Self = Self(Self::LowerCase.0 | Self::UpperCase.0);
    // Assumes that non-ascii chars are mostly alphabetic,
    // this should work out fine most of the time.
    const NonAscii: Self = Self(0b1100);
}

impl ByteKind {
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn eq(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    #[inline(always)]
    pub const fn ne(self, other: Self) -> bool {
        (self.0 & other.0) == 0
    }

    #[inline(always)]
    pub const fn is_alphabetic(self) -> bool {
        self.0 == Self::LowerCase.0 || self.0 == Self::UpperCase.0
    }

    pub const fn is_end_of_word(mut self, prev: Self, other: Self) -> bool {
        if self.0 == Self::NonAscii.0 {
            self = prev;
        }

        if self.0 == Self::UpperCase.0 {
            other.ne(Self::Alphabetic)
        } else {
            self.ne(other)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct WordIterator<'a> {
    bytes: &'a [u8],
    start: usize,
}

const BYTE_KIND: &[ByteKind; 256] = &{
    let mut out = [ByteKind::NonAscii; 256];

    // Make sure that this goes first
    for_range_inc! {i in 0, 127 => out[i as usize] = ByteKind::Other; }
    for_range_inc! {i in b'A', b'Z' => out[i as usize] = ByteKind::UpperCase; }
    for_range_inc! {i in b'a', b'z' => out[i as usize] = ByteKind::LowerCase; }
    for_range_inc! {i in b'0', b'9' => out[i as usize] = ByteKind::Number; }

    out
};

impl<'a> WordIterator<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, start: 0 }
    }

    const fn skip_same_kind(mut self, mut kind: ByteKind) -> (Self, ByteKind) {
        let orig_bytes_len = self.bytes.len();

        let mut prev_kind = kind;
        while let [b, rem @ ..] = self.bytes {
            let next_kind = BYTE_KIND[*b as usize];
            let cmp = kind.is_end_of_word(prev_kind, next_kind);
            if kind.is_alphabetic() {
                prev_kind = kind;
            }
            kind = next_kind;
            if cmp {
                break;
            }
            self.bytes = rem;
        }

        // Advance until a char boundary is found
        while let [b, rem @ ..] = self.bytes {
            if (*b as i8) >= -0x40 {
                break;
            }
            self.bytes = rem;
        }

        // Remember not to add return statements to the function
        self.start += orig_bytes_len - self.bytes.len();

        (self, kind)
    }

    pub(crate) const fn next(self) -> Option<(Self, Range<usize>)> {
        let (this, fkind) = self.skip_same_kind(ByteKind::Other);
        if let [] = this.bytes {
            None
        } else {
            let (next, _) = this.skip_same_kind(fkind);
            let range = this.start..next.start;
            Some((next, range))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use arrayvec::ArrayVec;

    fn get_words(text: &str) -> ArrayVec<&str, 20> {
        let mut list = <ArrayVec<&str, 20>>::new();
        let mut word_iter = WordIterator::new(text.as_bytes());

        while let Some((niter, word_range)) = word_iter.next() {
            word_iter = niter;
            list.push(&text[word_range]);
        }

        list
    }

    #[test]
    fn test_word_iter() {
        assert_eq!(
            get_words("01934324ñmaniÑNnFooBar")[..],
            ["01934324", "ñmaniÑ", "Nn", "Foo", "Bar"],
        );

        assert_eq!(
            get_words("01934 324  ñmani-嶲Nn____FOOOBar")[..],
            ["01934", "324", "ñmani", "嶲Nn", "FOOOBar"],
        );

        assert_eq!(get_words("    01934 1111 ")[..], ["01934", "1111"],);

        assert_eq!(get_words("    嶲01934 ")[..], ["嶲", "01934"],);

        assert_eq!(get_words("    嶲A01934 ")[..], ["嶲A", "01934"],);

        assert_eq!(get_words("    嶲a01934 ")[..], ["嶲a", "01934"],);

        assert_eq!(get_words("    ñA01934 ")[..], ["ñA", "01934"],);

        assert_eq!(get_words("    ña01934 ")[..], ["ña", "01934"],);
    }
}
