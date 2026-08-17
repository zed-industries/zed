#![allow(missing_docs)]

#[doc(hidden)]
#[macro_export]
macro_rules! __identity {
    ($($tt:tt)*) => {$($tt)*};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_rng_ext {
    () => {
        pub trait RngExt {
            fn as_rng(&self) -> &fastrand::Rng;

            fn pick<'a, T>(&self, slice: &'a [T]) -> &'a T {
                &slice[self.as_rng().usize(0..slice.len())]
            }

            fn char_(&self, bounds: core::ops::RangeInclusive<char>) -> char {
                let this = self.as_rng();

                if let None = bounds.clone().next() {
                    panic!("There are no chars in the {:?} bounds", bounds);
                }

                let u32_bounds = u32::from(*bounds.start())..=u32::from(*bounds.end());

                loop {
                    if let Some(x) = core::char::from_u32(this.u32(u32_bounds.clone())) {
                        break x;
                    }
                }
            }

            fn unicode_char(&self) -> char {
                let this = self.as_rng();
                let range = this
                    .pick(&[
                        '\u{0000}'..='\u{007F}',
                        '\u{0080}'..='\u{07FF}',
                        '\u{0800}'..='\u{FFFF}',
                        '\u{10000}'..='\u{10FFFF}',
                    ])
                    .clone();
                this.char_(range)
            }
        }

        impl RngExt for fastrand::Rng {
            fn as_rng(&self) -> &fastrand::Rng {
                self
            }
        }
    };
}

pub const ALL_ASCII: &str = "\
 \x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\
 \x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f \
 !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]\
 ^_`abcdefghijklmnopqrstuvwxyz{|}~\x7f\u{80}\u{81}\u{90}\u{91}\
";

pub const ALL_ASCII_ESCAPED: &str = "\
 \\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\t\\n\\x0B\\x0C\\r\\x0E\\x0F\
 \\x10\\x11\\x12\\x13\\x14\\x15\\x16\\x17\\x18\\x19\\x1A\\x1B\\x1C\\x1D\\x1E\\x1F \
 !\\\"#$%&\\\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\\\]\
 ^_`abcdefghijklmnopqrstuvwxyz{|}~\x7f\u{80}\u{81}\u{90}\u{91}\
";
