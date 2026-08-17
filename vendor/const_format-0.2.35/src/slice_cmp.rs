/// A const equivalent of `&str` equality comparison.
///
/// # Example
///
#[cfg_attr(feature = "fmt", doc = "```rust")]
#[cfg_attr(not(feature = "fmt"), doc = "```ignore")]
/// use const_format::utils::str_eq;
///
/// const STRS: &[&str] = &[
///     "foo",
///     "fooooo",
///     "bar",
///     "baz",
/// ];
///
/// const ARE_0_0_EQ: bool = str_eq(STRS[0], STRS[0]);
/// const ARE_0_1_EQ: bool = str_eq(STRS[0], STRS[1]);
///
/// const ARE_1_1_EQ: bool = str_eq(STRS[1], STRS[1]);
/// const ARE_1_2_EQ: bool = str_eq(STRS[1], STRS[2]);
///
/// const ARE_2_2_EQ: bool = str_eq(STRS[2], STRS[2]);
/// const ARE_2_3_EQ: bool = str_eq(STRS[2], STRS[3]);
///
/// assert!(  ARE_0_0_EQ );
/// assert!( !ARE_0_1_EQ );
///
/// assert!(  ARE_1_1_EQ );
/// assert!( !ARE_1_2_EQ );
///
/// assert!(  ARE_2_2_EQ );
/// assert!( !ARE_2_3_EQ );
///
/// ```
///
pub const fn str_eq(left: &str, right: &str) -> bool {
    u8_slice_eq(left.as_bytes(), right.as_bytes())
}

/// A const equivalent of `&[u8]` equality comparison.
///
/// # Example
///
#[cfg_attr(feature = "fmt", doc = "```rust")]
#[cfg_attr(not(feature = "fmt"), doc = "```ignore")]
/// use const_format::utils::u8_slice_eq;
///
/// const SLICES: &[&[u8]] = &[
///     &[10, 20],
///     &[10, 20, 30, 40],
///     &[3, 5, 8, 13],
///     &[4, 9, 16, 25],
/// ];
///
/// const ARE_0_0_EQ: bool = u8_slice_eq(SLICES[0], SLICES[0]);
/// const ARE_0_1_EQ: bool = u8_slice_eq(SLICES[0], SLICES[1]);
///
/// const ARE_1_1_EQ: bool = u8_slice_eq(SLICES[1], SLICES[1]);
/// const ARE_1_2_EQ: bool = u8_slice_eq(SLICES[1], SLICES[2]);
///
/// const ARE_2_2_EQ: bool = u8_slice_eq(SLICES[2], SLICES[2]);
/// const ARE_2_3_EQ: bool = u8_slice_eq(SLICES[2], SLICES[3]);
///
/// assert!(  ARE_0_0_EQ );
/// assert!( !ARE_0_1_EQ );
///
/// assert!(  ARE_1_1_EQ );
/// assert!( !ARE_1_2_EQ );
///
/// assert!(  ARE_2_2_EQ );
/// assert!( !ARE_2_3_EQ );
///
/// ```
///
pub const fn u8_slice_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut i = 0;
    while i != left.len() {
        if left[i] != right[i] {
            return false;
        }
        i += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_eq_test() {
        assert!(u8_slice_eq(&[], &[]));
        assert!(!u8_slice_eq(&[], &[0]));
        assert!(!u8_slice_eq(&[0], &[]));
        assert!(u8_slice_eq(&[0], &[0]));
        assert!(!u8_slice_eq(&[0], &[1]));
        assert!(!u8_slice_eq(&[1], &[0]));
        assert!(!u8_slice_eq(&[0], &[0, 1]));
        assert!(!u8_slice_eq(&[0, 1], &[0]));
        assert!(u8_slice_eq(&[0, 1], &[0, 1]));
        assert!(!u8_slice_eq(&[0, 1], &[0, 2]));
    }

    #[test]
    fn str_eq_test() {
        assert!(str_eq("", ""));
        assert!(!str_eq("", "0"));
        assert!(!str_eq("0", ""));
        assert!(str_eq("0", "0"));
        assert!(!str_eq("0", "1"));
        assert!(!str_eq("1", "0"));
        assert!(!str_eq("0", "0, 1"));
        assert!(!str_eq("0, 1", "0"));
        assert!(!str_eq("0, 1", "1"));
        assert!(str_eq("0, 1", "0, 1"));
        assert!(!str_eq("0, 1", "0, 2"));
    }
}
