use crate::{
    formatting::{FormattingFlags, NumberFormatting as NF},
    pargument::PConvWrapper,
    wrapper_types::PWrapper,
};

use arrayvec::ArrayString;

use core::fmt::{self, Write};

fn get_digits_display(n: impl fmt::Display) -> ArrayString<64> {
    let mut buff = ArrayString::<64>::new();
    write!(buff, "{}", n).unwrap();
    buff
}
fn get_hex_digits(n: impl fmt::UpperHex) -> ArrayString<64> {
    let mut buff = ArrayString::<64>::new();
    write!(buff, "{:X}", n).unwrap();
    buff
}
fn get_lower_hex_digits(n: impl fmt::LowerHex) -> ArrayString<64> {
    let mut buff = ArrayString::<64>::new();
    write!(buff, "{:x}", n).unwrap();
    buff
}
fn get_binary_digits(n: impl fmt::Binary) -> ArrayString<192> {
    let mut buff = ArrayString::<192>::new();
    write!(buff, "{:b}", n).unwrap();
    buff
}

const DEF_FLAGS: FormattingFlags = FormattingFlags::DEFAULT;

// This doesn't use unsafe code
#[cfg(not(miri))]
macro_rules! check_number_of_digits_ {
    ($ty:ty) => {{
        fn number_of_digits_test_case(val: $ty) {
            let display_digits = get_digits_display(val);
            let hex_digits = get_hex_digits(val);
            let lower_hex_digits = get_lower_hex_digits(val);
            let binary_digits = get_binary_digits(val);
            let wrapper = PWrapper(val);

            {
                assert_eq!(
                    wrapper.compute_display_len(DEF_FLAGS),
                    display_digits.len(),
                    "const_display_len"
                );
                assert_eq!(
                    wrapper.compute_debug_len(DEF_FLAGS),
                    display_digits.len(),
                    "const_debug_len "
                );
                assert_eq!(
                    wrapper.compute_debug_len(DEF_FLAGS.set_num_fmt(NF::Hexadecimal)),
                    hex_digits.len(),
                    "const_debug_len hexadecimal"
                );
                assert_eq!(
                    wrapper.compute_debug_len(DEF_FLAGS.set_lower_hexadecimal()),
                    lower_hex_digits.len(),
                    "const_debug_len lower hexadecimal"
                );
                assert_eq!(
                    wrapper.compute_debug_len(DEF_FLAGS.set_num_fmt(NF::Binary)),
                    binary_digits.len(),
                    "const_debug_len binary"
                );
            }

            {
                let integer = PWrapper(PConvWrapper(val).to_integer());

                let sa = integer.to_start_array_display();
                assert_eq!(
                    &sa.array[sa.start..],
                    display_digits.as_bytes(),
                    "const_display_len"
                );

                let sa = integer.to_start_array_debug();
                assert_eq!(
                    &sa.array[sa.start..],
                    display_digits.as_bytes(),
                    "const_debug_len "
                );

                let sa = integer.to_start_array_hexadecimal(FormattingFlags::NEW);
                assert_eq!(
                    &sa.array[sa.start..],
                    hex_digits.as_bytes(),
                    "const_debug_len hexadecimal"
                );

                let sa = integer
                    .to_start_array_hexadecimal(FormattingFlags::NEW.set_lower_hexadecimal());
                assert_eq!(
                    &sa.array[sa.start..],
                    lower_hex_digits.as_bytes(),
                    "const_debug_len lower hexadecimal"
                );

                let sa = integer.to_start_array_binary(FormattingFlags::NEW);
                assert_eq!(
                    &sa.array[sa.start..],
                    binary_digits.as_bytes(),
                    "const_debug_len binary"
                );
            }
        }

        let zero: $ty = 0;
        let one: $ty = 1;
        let two: $ty = 2;

        number_of_digits_test_case(zero);
        number_of_digits_test_case(one);
        number_of_digits_test_case(two);

        let mut n: $ty = 10;

        loop {
            number_of_digits_test_case(n - 1);
            number_of_digits_test_case(n);
            number_of_digits_test_case(n + 1);

            match n.checked_mul(10) {
                Some(next) => n = next,
                None => break,
            }
        }

        let max_s2: $ty = <$ty>::MAX - 2;
        let max_s1: $ty = <$ty>::MAX - 1;
        let max_s0: $ty = <$ty>::MAX;

        number_of_digits_test_case(max_s2);
        number_of_digits_test_case(max_s1);
        number_of_digits_test_case(max_s0);
    }};
}

// This doesn't use unsafe code
#[cfg(not(miri))]
#[test]
fn pwrapper_methods() {
    check_number_of_digits_!(i8);
    check_number_of_digits_!(u8);
    check_number_of_digits_!(i16);
    check_number_of_digits_!(u16);
    check_number_of_digits_!(i32);
    check_number_of_digits_!(u32);
    check_number_of_digits_!(u64);
    check_number_of_digits_!(i64);
    check_number_of_digits_!(usize);
    check_number_of_digits_!(isize);
    check_number_of_digits_!(u128);
    check_number_of_digits_!(i128);
}

#[cfg(feature = "fmt")]
#[test]
fn wrapped_formatting() {
    use crate::fmt::{Error, StrWriter};

    const fn inner(writer: &mut StrWriter) -> Result<(), Error> {
        try_!(writec!(writer, "{},", PWrapper(3u8)));
        try_!(writec!(writer, "{},", PWrapper(5u16)));
        try_!(writec!(writer, "{},", PWrapper(8u32)));
        try_!(writec!(writer, "{},", PWrapper("hello")));
        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 128]);

    inner(writer).unwrap();

    assert_eq!(writer.as_str(), "3,5,8,hello,");
}
