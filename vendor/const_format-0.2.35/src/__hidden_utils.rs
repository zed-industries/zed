pub(crate) const fn max_usize(l: usize, r: usize) -> usize {
    if l > r {
        l
    } else {
        r
    }
}
pub(crate) const fn saturating_add(l: usize, r: usize) -> usize {
    let (sum, overflowed) = l.overflowing_add(r);
    if overflowed {
        usize::MAX
    } else {
        sum
    }
}

pub(crate) const fn is_char_boundary_no_len_check(str: &[u8], index: usize) -> bool {
    index == str.len() || (str[index] as i8) >= -0x40
}

#[repr(C)]
pub union PtrToRef<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}
