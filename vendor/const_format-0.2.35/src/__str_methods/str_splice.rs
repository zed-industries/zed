use super::str_indexing::{pass_range_types, IndexValidity, StrIndexArgs, StrIndexArgsConv};

pub struct StrSplceArgsConv<T> {
    pub arg: T,
    pub str: &'static str,
    pub insert: &'static str,
}

#[allow(non_snake_case)]
pub const fn StrSplceArgsConv<T>(
    str: &'static str,
    arg: T,
    insert: &'static str,
) -> StrSplceArgsConv<T> {
    StrSplceArgsConv { str, arg, insert }
}

pub struct StrSpliceArgs {
    pub str: &'static str,
    pub insert: &'static str,
    pub index_validity: IndexValidity,
    pub used_rstart: usize,
    pub used_rlen: usize,
    pub insert_len: usize,
    pub suffix_len: usize,
    pub out_len: usize,
}

/// The return value of [`str_splice`](./macro.str_splice.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SplicedStr {
    /// A string that had `removed` replaced with some other string.
    pub output: &'static str,
    /// The part of the string that was removed.
    pub removed: &'static str,
}

#[repr(C, packed)]
pub struct DecomposedString<P, M, S> {
    pub prefix: P,
    pub middle: M,
    pub suffix: S,
}

macro_rules! define_conversions {
    (
        $( fn($self:ident, $ty:ty) $block:block )*
    ) => {
        $(
            impl StrSplceArgsConv<$ty> {
                pub const fn conv(self) -> StrSpliceArgs {
                    let StrIndexArgs{
                        str,
                        index_validity,
                        used_rstart,
                        used_rend,
                        used_rlen,
                    } = StrIndexArgsConv{
                        arg: self.arg,
                        str: self.str,
                    }.conv();

                    StrSpliceArgs{
                        str,
                        index_validity,
                        used_rstart,
                        used_rlen,
                        insert: self.insert,
                        insert_len: self.insert.len(),
                        suffix_len: str.len() - used_rend,
                        out_len: str.len() - used_rlen + self.insert.len(),
                    }
                }
            }
        )*
    };
}

pass_range_types! {define_conversions}
