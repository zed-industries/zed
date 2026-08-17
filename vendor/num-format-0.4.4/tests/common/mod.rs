use core::str;

use lazy_static::lazy_static;
use num_format::{CustomFormat, Grouping};

lazy_static! {
    pub(crate) static ref POLICIES: [CustomFormat; 5] = {
        let four_byte_char = "𠜱";
        let longest_minus_sign =
            unsafe { str::from_utf8_unchecked(&[226u8, 128, 142, 45, 226, 128, 142]) };
        [
            CustomFormat::builder()
                .grouping(Grouping::Standard)
                .minus_sign("-")
                .separator(",")
                .build()
                .unwrap(),
            CustomFormat::builder()
                .grouping(Grouping::Standard)
                .minus_sign(longest_minus_sign)
                .separator(four_byte_char)
                .build()
                .unwrap(),
            CustomFormat::builder()
                .grouping(Grouping::Indian)
                .minus_sign(longest_minus_sign)
                .separator(four_byte_char)
                .build()
                .unwrap(),
            CustomFormat::builder()
                .grouping(Grouping::Posix)
                .minus_sign(longest_minus_sign)
                .separator(four_byte_char)
                .build()
                .unwrap(),
            CustomFormat::builder()
                .grouping(Grouping::Standard)
                .minus_sign(longest_minus_sign)
                .separator("")
                .build()
                .unwrap(),
        ]
    };
}
