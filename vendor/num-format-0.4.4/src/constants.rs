use crate::strings::{MAX_MIN_LEN, MAX_SEP_LEN};

// Want this to be as large as the largest possible string representation of any type
// that implements ToFormattedStr, which is currently i128's Grouping::Indian representation.
// The max len of an i128 formatted string is ...
// 39 digits + 18 separators (each potentially 8 bytes) + 1 minus sign (potentially 8 bytes)
pub(crate) const MAX_BUF_LEN: usize = 39 + 18 * MAX_SEP_LEN + MAX_MIN_LEN;

pub(crate) const TABLE: &[u8] = b"\
    0001020304050607080910111213141516171819\
    2021222324252627282930313233343536373839\
    4041424344454647484950515253545556575859\
    6061626364656667686970717273747576777879\
    8081828384858687888990919293949596979899";

pub(crate) const U8_MAX_LEN: usize = 3;
pub(crate) const U16_MAX_LEN: usize = 5;
pub(crate) const U32_MAX_LEN: usize = 10;
pub(crate) const USIZE_MAX_LEN: usize = 20;
pub(crate) const U64_MAX_LEN: usize = 20;
pub(crate) const U128_MAX_LEN: usize = 39;

pub(crate) const I8_MAX_LEN: usize = 4;
pub(crate) const I16_MAX_LEN: usize = 6;
pub(crate) const I32_MAX_LEN: usize = 11;
pub(crate) const ISIZE_MAX_LEN: usize = 20;
pub(crate) const I64_MAX_LEN: usize = 20;
pub(crate) const I128_MAX_LEN: usize = 40;
