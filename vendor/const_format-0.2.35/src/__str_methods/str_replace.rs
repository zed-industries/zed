use super::{bytes_find, Pattern, PatternCtor, PatternNorm};

pub struct ReplaceInputConv<T>(pub &'static str, pub T, pub &'static str);

macro_rules! ctor {
    ($ty:ty) => {
        impl ReplaceInputConv<$ty> {
            pub const fn conv(self) -> ReplaceInput {
                ReplaceInput {
                    str: self.0,
                    pattern: PatternCtor(self.1).conv(),
                    replaced_with: self.2,
                }
            }
        }
    };
}

ctor! {u8}
ctor! {&'static str}
ctor! {char}

pub struct ReplaceInput {
    str: &'static str,
    pattern: Pattern,
    replaced_with: &'static str,
}

impl ReplaceInput {
    pub const fn replace_length(&self) -> usize {
        str_replace_length(self.str, self.pattern, self.replaced_with)
    }
    pub const fn replace<const L: usize>(&self) -> [u8; L] {
        str_replace(self.str, self.pattern, self.replaced_with)
    }
}

const fn str_replace_length(inp: &str, r: Pattern, replaced_with: &str) -> usize {
    let inp = inp.as_bytes();

    let replaced_len = replaced_with.len();
    let mut out_len = 0;

    match r.normalize() {
        PatternNorm::AsciiByte(byte) => {
            let byte = byte.get();
            iter_copy_slice! {b in inp =>
                out_len += if b == byte { replaced_len } else { 1 };
            }
        }
        PatternNorm::Str(str) => {
            if str.is_empty() {
                return inp.len();
            }
            let str_len = str.len();
            let mut i = 0;
            while let Some(next_match) = bytes_find(inp, str, i) {
                out_len += (next_match - i) + replaced_len;
                i = next_match + str_len;
            }
            out_len += inp.len() - i;
        }
    }

    out_len
}

const fn str_replace<const L: usize>(inp: &str, r: Pattern, replaced_with: &str) -> [u8; L] {
    let inp = inp.as_bytes();

    let replaced_with_bytes = replaced_with.as_bytes();
    let mut out = [0u8; L];
    let mut out_i = 0;

    macro_rules! write_replaced {
        () => {
            iter_copy_slice! {b in replaced_with_bytes =>
                out[out_i] = b;
                out_i += 1;
            }
        };
    }
    macro_rules! write_byte {
        ($byte:expr) => {
            out[out_i] = $byte;
            out_i += 1;
        };
    }

    match r.normalize() {
        PatternNorm::AsciiByte(byte) => {
            let byte = byte.get();
            iter_copy_slice! {b in inp =>
                if b == byte {
                    write_replaced!{}
                } else {
                    write_byte!{b}
                }
            }
        }
        PatternNorm::Str(str) => {
            if str.is_empty() {
                iter_copy_slice! {b in inp =>
                    write_byte!(b);
                }
                return out;
            }
            let str_len = str.len();
            let mut i = 0;
            while let Some(next_match) = bytes_find(inp, str, i) {
                __for_range! {j in i..next_match =>
                    write_byte!(inp[j]);
                }
                write_replaced! {}

                i = next_match + str_len;
            }
            __for_range! {j in i..inp.len() =>
                write_byte!(inp[j]);
            }
        }
    }
    out
}
