use core::cmp::Ordering::{self, Equal, Greater, Less};

use crate::big_digit::BigDigit;

pub fn cmp_slice(a: &[BigDigit], b: &[BigDigit]) -> Ordering {
    debug_assert!(a.last() != Some(&0));
    debug_assert!(b.last() != Some(&0));

    let (a_len, b_len) = (a.len(), b.len());
    if a_len < b_len {
        return Less;
    }
    if a_len > b_len {
        return Greater;
    }

    for (&ai, &bi) in a.iter().rev().zip(b.iter().rev()) {
        if ai < bi {
            return Less;
        }
        if ai > bi {
            return Greater;
        }
    }
    Equal
}

#[cfg(test)]
mod tests {
    use crate::BigUint;

    use num_traits::Num;

    #[test]
    fn test_eq() {
        let a = BigUint::from_str_radix("265252859812191058636308480000000", 10).unwrap();
        let b = BigUint::from_str_radix("26525285981219105863630848000000", 10).unwrap();
        assert!(a != b);
        assert_ne!(a, b);

        let a = BigUint::from_str_radix("138995801145388806366366393471481216294", 10).unwrap();
        let b = BigUint::from_str_radix("168653801169012228514850424976871974699", 10).unwrap();

        assert!(a != b);
        assert_ne!(a, b);
        assert!(&a != &b);
        assert_ne!(&a, &b);
    }
}
