#![cfg(feature = "std")]

use std::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroUsize};

use num_format::{CustomFormat, Locale, WriteFormatted};

#[test]
fn test_no_bytes_written() {
    macro_rules! test1 {
        ( $( $n:expr ),* ) => {
            {
                $(
                    let mut s = String::new();
                    let c = s.write_formatted(&$n, &Locale::en).unwrap();
                    assert_eq!(c, 5);
                )*
            }
        };
    }

    test1!(
        1_000u16,
        1_000u32,
        1_000usize,
        1_000u64,
        1_000u128,
        1_000i16,
        1_000i32,
        1_000isize,
        1_000i64,
        1_000i128,
        NonZeroU16::new(1_000).unwrap(),
        NonZeroU32::new(1_000).unwrap(),
        NonZeroUsize::new(1_000).unwrap(),
        NonZeroU64::new(1_000).unwrap(),
        NonZeroU128::new(1_000).unwrap()
    );

    macro_rules! test2 {
        ( $( $n:expr ),* ) => {
            {
                $(
                    let mut s = String::new();
                    let format = CustomFormat::builder().separator("𠜱").build().unwrap();
                    let c = s.write_formatted(&$n, &format).unwrap();
                    assert_eq!(c, 8);
                )*
            }
        };
    }

    test2!(
        1_000u16,
        1_000u32,
        1_000usize,
        1_000u64,
        1_000u128,
        1_000i16,
        1_000i32,
        1_000isize,
        1_000i64,
        1_000i128,
        NonZeroU16::new(1_000).unwrap(),
        NonZeroU32::new(1_000).unwrap(),
        NonZeroUsize::new(1_000).unwrap(),
        NonZeroU64::new(1_000).unwrap(),
        NonZeroU128::new(1_000).unwrap()
    );
}
