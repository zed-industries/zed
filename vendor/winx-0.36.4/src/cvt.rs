// This file is derived from code in library/std/src/sys/windows/mod.rs
// in Rust at revision e708cbd91c9cae4426d69270248362b423324556.

pub(crate) trait IsZero {
    fn is_zero(&self) -> bool;
}

macro_rules! impl_is_zero {
    ($($t:ident)*) => ($(impl IsZero for $t {
        fn is_zero(&self) -> bool {
            *self == 0
        }
    })*)
}

impl_is_zero! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }

pub(crate) fn cvt<I: IsZero>(i: I) -> std::io::Result<I> {
    if i.is_zero() {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(i)
    }
}
