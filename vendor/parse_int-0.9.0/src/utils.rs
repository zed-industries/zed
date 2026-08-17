use std::any::{Any, TypeId};

#[inline(always)]
pub(crate) fn process_integer(integer: &str, separator: usize) -> String {
    let (sign, digits) = if let Some(digits) = integer.strip_prefix('-') {
        ("-", digits)
    } else {
        ("", integer)
    };

    let len = digits.len();
    if len <= separator {
        return integer.to_string();
    }

    let split_pos = len % separator;

    // invert
    let split_pos = separator - split_pos;

    // start with the sign
    let mut result = sign.to_string();
    let mut count = split_pos % separator;

    for c in digits.chars() {
        if count == separator {
            result.push('_');
            count = 1;
        } else {
            count += 1;
        }
        result.push(c);
    }

    result
}

#[cfg(test)]
mod test_integer {
    use super::*;

    #[test]
    fn short() {
        assert_eq!("ab", process_integer("ab", 2));
        assert_eq!("-ab", process_integer("-ab", 2));

        assert_eq!("abc", process_integer("abc", 3));
        assert_eq!("-abc", process_integer("-abc", 3));
    }

    #[test]
    fn twos() {
        assert_eq!("a_bc_de_f1_23", process_integer("abcdef123", 2));
        assert_eq!("-a_bc_de_f1_23", process_integer("-abcdef123", 2));
        assert_eq!("c_de_f1_23", process_integer("cdef123", 2));
    }
    #[test]
    fn threes() {
        assert_eq!("abc_def_123", process_integer("abcdef123", 3));
        assert_eq!("-abc_def_123", process_integer("-abcdef123", 3));

        assert_eq!("bc_def_123", process_integer("bcdef123", 3));
        assert_eq!("-bc_def_123", process_integer("-bcdef123", 3));

        assert_eq!("c_def_123", process_integer("cdef123", 3));
        assert_eq!("-c_def_123", process_integer("-cdef123", 3));
    }
}

#[inline(always)]
pub(crate) fn process_fractional(fractional: &str) -> String {
    let mut result = String::new();
    let mut count = 0;

    for c in fractional.chars() {
        if count == 3 {
            result.push('_');
            count = 1;
        } else {
            count += 1;
        }
        result.push(c);
    }

    result
}

#[cfg(test)]
mod test_fraction {
    use super::*;

    #[test]
    fn threes() {
        assert_eq!("abc_def_123", process_fractional("abcdef123"));
        assert_eq!("abc_def_1", process_fractional("abcdef1"));
    }
}

#[inline(always)]
pub(crate) fn is_floaty<N: ?Sized + Any>(_num: &N) -> bool {
    let nt = TypeId::of::<N>();
    TypeId::of::<f32>() == nt || TypeId::of::<f64>() == nt
    // unstable types for now
    // || TypeId::of::<f16>() == nt
    //|| TypeId::of::<f128>() == nt
}

#[cfg(test)]
mod test_floaty {
    use super::*;

    #[test]
    fn bool() {
        assert_eq!(is_floaty(&0.0_f32), true, "f32");
        assert_eq!(is_floaty(&0.0_f64), true, "f64");

        assert_eq!(is_floaty(&0_i32), false);
        assert_eq!(is_floaty(&0_isize), false);
        assert_eq!(is_floaty(&0_usize), false);
        assert_eq!(is_floaty(&0_u64), false);

        assert_eq!(is_floaty(&"ferris".to_string()), false);
    }
}
