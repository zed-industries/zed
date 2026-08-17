mod str_replace;

pub use self::str_replace::{ReplaceInput, ReplaceInputConv};

mod str_repeat;
pub use str_repeat::StrRepeatArgs;

mod str_splice;
pub use str_splice::{DecomposedString, SplicedStr, StrSplceArgsConv, StrSpliceArgs};

mod str_indexing;
pub use str_indexing::{IndexValidity, StrIndexArgs, StrIndexArgsConv};

#[cfg(feature = "rust_1_64")]
mod str_split;

#[cfg(feature = "rust_1_64")]
pub use str_split::{SplitInput, SplitInputConv};

mod pattern;

use pattern::{Pattern, PatternCtor, PatternNorm};

mod ascii_byte {
    #[derive(Copy, Clone)]
    pub struct AsciiByte(u8);

    impl AsciiByte {
        #[inline(always)]
        pub const fn new(byte: u8) -> Self {
            if byte > 127 {
                let byte = byte as usize;
                let _: () = [/* byte isn't valid ascii */][byte];
                loop {}
            }
            Self(byte)
        }
        #[inline(always)]
        pub const fn get(self) -> u8 {
            self.0
        }
    }
}
pub use ascii_byte::AsciiByte;

// copied from the konst crate, if that implementation is wrong, this needs to be fixed
const fn bytes_find(left: &[u8], right: &[u8], from: usize) -> Option<usize> {
    let mut matching = right;

    __for_range! {i in from..left.len() =>
        match matching {
            [mb, m_rem @ ..] => {
                let b = left[i];

                matching = if b == *mb {
                    m_rem
                } else {
                    match right {
                        // For when the string is "lawlawn" and we are trying to find "lawn"
                        [mb2, m_rem2 @ ..] if b == *mb2 => m_rem2,
                        _ => right,
                    }
                };
            }
            [] => {
                return Some(i - right.len())
            }
        }
    }

    if matching.is_empty() {
        Some(left.len() - right.len())
    } else {
        None
    }
}
