/// Numeric-like type can be represented by a `u64`.
///
/// Used by `Atomic` to store values.
pub(crate) trait Numeric: Sized + Copy + PartialEq {
    /// Convert a value into `u64` representation
    fn into_u64(self) -> u64;

    /// Convert a `u64` representation into the value
    fn from_u64(src: u64) -> Self;
}

macro_rules! impl_num {
    ( $($t:ty),* ) => {
        $(
            impl Numeric for $t {
                fn into_u64(self) -> u64 {
                    self as u64
                }

                fn from_u64(src: u64) -> $t {
                    src as $t
                }
            }
        )*
    };
}

impl_num!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl<T> Numeric for *mut T {
    fn into_u64(self) -> u64 {
        self as u64
    }

    fn from_u64(src: u64) -> *mut T {
        src as *mut T
    }
}

impl Numeric for bool {
    fn into_u64(self) -> u64 {
        if self {
            1
        } else {
            0
        }
    }

    fn from_u64(src: u64) -> bool {
        src != 0
    }
}
