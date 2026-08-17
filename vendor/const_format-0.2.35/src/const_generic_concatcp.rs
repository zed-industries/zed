//! Reimplements some stuff from concatcp to be const generic instead of macro generated

use crate::pmr::{LenAndArray, PArgument, PVariant};

#[doc(hidden)]
pub const fn __priv_concatenate<const LEN: usize>(input: &[PArgument]) -> LenAndArray<[u8; LEN]> {
    let mut out = LenAndArray {
        len: 0,
        array: [0u8; LEN],
    };

    crate::__for_range! { outer_i in 0..input.len() =>
        let current = &input[outer_i];

        match current.elem {
            PVariant::Str(s) => crate::__write_pvariant!(str, current, s => out),
            PVariant::Int(int) => crate::__write_pvariant!(int, current, int => out),
            PVariant::Char(c) => crate::__write_pvariant!(char, current, c => out),
        }
    }

    out
}
