use crate::helper::{Helper, NestingLevel};

use super::Encoding;

pub(crate) const fn static_int_str_len(mut n: u64) -> usize {
    let mut i = 0;
    if n == 0 {
        return 1;
    }
    while n > 0 {
        n /= 10;
        i += 1;
    }
    i
}

pub(crate) const fn static_int_str_array<const RES: usize>(mut n: u64) -> [u8; RES] {
    let mut res: [u8; RES] = [0; RES];
    let mut i = 0;
    if n == 0 {
        res[0] = b'0';
        return res;
    }
    while n > 0 {
        res[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }

    let mut rev: [u8; RES] = [0; RES];
    let mut rev_i = 0;
    while 0 < i {
        i -= 1;
        rev[rev_i] = res[i];
        n /= 10;
        rev_i += 1;
    }
    rev
}

pub(crate) const fn static_encoding_str_len(encoding: &Encoding, level: NestingLevel) -> usize {
    use Helper::*;

    match Helper::new(encoding) {
        Primitive(primitive) => primitive.to_str().len(),
        BitField(size, None) => 1 + static_int_str_len(size as u64),
        BitField(size, Some((offset, t))) => {
            1 + static_int_str_len(*offset)
                + static_encoding_str_len(t, level.bitfield())
                + static_int_str_len(size as u64)
        }
        Indirection(kind, t) => 1 + static_encoding_str_len(t, level.indirection(kind)),
        Array(len, item) => {
            1 + static_int_str_len(len) + static_encoding_str_len(item, level.array()) + 1
        }
        Container(_, name, items) => {
            let mut res = 1 + name.len();
            if let Some(level) = level.container_include_fields() {
                res += 1;
                let mut i = 0;
                while i < items.len() {
                    res += static_encoding_str_len(&items[i], level);
                    i += 1;
                }
            }
            res + 1
        }
        NoneInvalid => 0,
    }
}

pub(crate) const fn static_encoding_str_array<const LEN: usize>(
    encoding: &Encoding,
    level: NestingLevel,
) -> [u8; LEN] {
    use Helper::*;

    let mut res: [u8; LEN] = [0; LEN];
    let mut res_i = 0;

    match Helper::new(encoding) {
        Primitive(primitive) => {
            let s = primitive.to_str().as_bytes();
            let mut i = 0;
            while i < s.len() {
                res[i] = s[i];
                i += 1;
            }
        }
        BitField(size, None) => {
            res[res_i] = b'b';
            res_i += 1;

            let mut i = 0;
            // We use 3 even though it creates an oversized array
            let arr = static_int_str_array::<3>(size as u64);
            while i < static_int_str_len(size as u64) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }
        }
        BitField(size, Some((offset, t))) => {
            let level = level.bitfield();
            res[res_i] = b'b';
            res_i += 1;

            let mut i = 0;
            // We use 20 even though it creates an oversized array
            let arr = static_int_str_array::<20>(*offset);
            while i < static_int_str_len(*offset) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }

            let mut i = 0;
            // We use LEN even though it creates an oversized array
            // This could probably be reduced to 1
            let arr = static_encoding_str_array::<LEN>(t, level);
            while i < static_encoding_str_len(t, level) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }

            let mut i = 0;
            // We use 3 even though it creates an oversized array
            let arr = static_int_str_array::<3>(size as u64);
            while i < static_int_str_len(size as u64) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }
        }
        Indirection(kind, t) => {
            let level = level.indirection(kind);
            res[res_i] = kind.prefix_byte();
            res_i += 1;

            let mut i = 0;
            // We use LEN even though it creates an oversized array
            let arr = static_encoding_str_array::<LEN>(t, level);
            while i < static_encoding_str_len(t, level) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }
        }
        Array(len, item) => {
            let level = level.array();
            let mut res_i = 0;

            res[res_i] = b'[';
            res_i += 1;

            let mut i = 0;
            // We use 20 even though it creates an oversized array
            let arr = static_int_str_array::<20>(len);
            while i < static_int_str_len(len) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }

            let mut i = 0;
            // We use LEN even though it creates an oversized array
            let arr = static_encoding_str_array::<LEN>(item, level);
            while i < static_encoding_str_len(item, level) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }

            res[res_i] = b']';
        }
        Container(kind, name, items) => {
            let mut res_i = 0;

            res[res_i] = kind.start_byte();
            res_i += 1;

            let mut name_i = 0;
            let name = name.as_bytes();
            while name_i < name.len() {
                res[res_i] = name[name_i];
                res_i += 1;
                name_i += 1;
            }

            if let Some(level) = level.container_include_fields() {
                res[res_i] = b'=';
                res_i += 1;

                let mut items_i = 0;
                while items_i < items.len() {
                    // We use LEN even though it creates an oversized array
                    let field_res = static_encoding_str_array::<LEN>(&items[items_i], level);

                    let mut item_res_i = 0;
                    while item_res_i < static_encoding_str_len(&items[items_i], level) {
                        res[res_i] = field_res[item_res_i];
                        res_i += 1;
                        item_res_i += 1;
                    }
                    items_i += 1;
                }
            }

            res[res_i] = kind.end_byte();
        }
        NoneInvalid => {}
    };
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! const_int_str {
        ($n:expr) => {{
            const X: [u8; static_int_str_len($n)] = static_int_str_array($n);
            unsafe { core::str::from_utf8_unchecked(&X) }
        }};
    }

    #[test]
    fn test_const_int_str() {
        const STR_0: &str = const_int_str!(0);
        const STR_4: &str = const_int_str!(4);
        const STR_42: &str = const_int_str!(42);
        const STR_100: &str = const_int_str!(100);
        const STR_999: &str = const_int_str!(999);
        const STR_1236018655: &str = const_int_str!(1236018655);

        assert_eq!(STR_0, "0");
        assert_eq!(STR_4, "4");
        assert_eq!(STR_42, "42");
        assert_eq!(STR_100, "100");
        assert_eq!(STR_999, "999");
        assert_eq!(STR_1236018655, "1236018655");
    }

    // static encoding tests are in `encoding.rs`
}
