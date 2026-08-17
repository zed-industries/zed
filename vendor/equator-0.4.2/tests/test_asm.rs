use core::hint::black_box;
use equator::assert;
use std::collections::HashMap;

#[test]
#[should_panic]
pub fn test() {
    black_box(test_asm_1_med as fn(_, _));
    black_box(test_asm_2_smol as fn(_, _, _, _));
    black_box(test_asm_2_med as fn(_, _, _, _));
    black_box(test_asm_2_big as fn(_, _, _, _));

    black_box(test_std_asm_1_med as fn(_, _));
    black_box(test_std_asm_2_smol as fn(_, _, _, _));
    black_box(test_std_asm_2_med as fn(_, _, _, _));
    black_box(test_std_asm_2_big as fn(_, _, _, _));

    black_box(test_assert2_asm_1_med as fn(_, _));
    black_box(test_assert2_asm_2_smol as fn(_, _, _, _));
    black_box(test_assert2_asm_2_med as fn(_, _, _, _));
    black_box(test_assert2_asm_2_big as fn(_, _, _, _));

    let x = 3;
    let y = 2;
    let z = true;

    assert!(true == false);
    assert!(all(true == false, x < y, any(!z, z)));
    assert!(all(true == false, x + 1 < y, any(!z, z)));
}

#[test]
#[should_panic]
pub fn test_different_types() {
    assert!(*[0, 1, 2usize].as_slice() == [0, 1usize]);
}

#[derive(Copy, Clone, Debug)]
struct ApproxEq {
    symbol: &'static str,
    tol: f64,
}

#[derive(Copy, Clone, Debug)]
enum ApproxEqError {
    Absolute { distance: f64 },
}

impl equator::CmpError<ApproxEq, f64, f64> for ApproxEq {
    type Error = ApproxEqError;
}
impl equator::Cmp<f64, f64> for ApproxEq {
    fn test(&self, lhs: &f64, rhs: &f64) -> Result<(), Self::Error> {
        let distance = (lhs - rhs).abs();
        let result = if distance <= self.tol {
            Ok(())
        } else {
            Err(ApproxEqError::Absolute { distance })
        };

        result
    }
}
impl equator::CmpDisplay<ApproxEq, f64, f64> for ApproxEqError {
    fn fmt(
        &self,
        cmp: &ApproxEq,
        lhs: &f64,
        lhs_source: &str,
        _: &dyn core::fmt::Debug,
        rhs: &f64,
        rhs_source: &str,
        _: &dyn core::fmt::Debug,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        let ApproxEq { symbol, tol } = *cmp;
        let ApproxEqError::Absolute { distance } = self;

        writeln!(
                f,
                "Assertion failed: {lhs_source} {symbol} {rhs_source}, with absolute tolerance {tol:.1e}"
            )?;
        writeln!(f, "- {lhs_source} = {lhs:#?}")?;
        writeln!(f, "- {rhs_source} = {rhs:#?}")?;
        write!(f, "- distance = {distance:#?}")
    }
}

#[test]
pub fn test_move() {
    let ref mut m = HashMap::<usize, Vec<()>>::new();
    let x = vec![];
    assert!(*x == []);
    assert!(*x == [], "oops {x:?}");
    assert!(m.insert(0, x).is_none());
}

#[inline(never)]
pub fn test_asm_1_med(a: usize, b: usize) {
    assert!(a == b);
}

#[inline(never)]
pub fn test_asm_2_smol(a: u8, b: u8, c: u8, d: u8) {
    assert!([a, c] == [b, d]);
}

#[inline(never)]
pub fn test_asm_2_med(a: usize, b: usize, c: usize, d: usize) {
    assert!(all(a == b, c == d));
}

#[inline(never)]
pub fn test_asm_2_big(a: usize, b: usize, c: usize, d: usize) {
    assert!([a, c] == [b, d]);
}

#[inline(never)]
pub fn test_std_asm_2_smol(a: u8, b: u8, c: u8, d: u8) {
    std::assert_eq!([a, c], [b, d]);
}

#[inline(never)]
pub fn test_std_asm_2_med(a: usize, b: usize, c: usize, d: usize) {
    std::assert_eq!(a, b);
    std::assert_eq!(c, d);
}

#[inline(never)]
pub fn test_std_asm_2_big(a: usize, b: usize, c: usize, d: usize) {
    std::assert_eq!([a, c], [b, d]);
}

#[inline(never)]
pub fn test_assert2_asm_2_smol(a: u8, b: u8, c: u8, d: u8) {
    assert2::assert!([a, c] == [b, d]);
}

#[inline(never)]
pub fn test_assert2_asm_2_med(a: usize, b: usize, c: usize, d: usize) {
    assert2::assert!(a == b);
    assert2::assert!(c == d);
}

#[inline(never)]
pub fn test_assert2_asm_2_big(a: usize, b: usize, c: usize, d: usize) {
    assert2::assert!([a, c] == [b, d]);
}

#[inline(never)]
pub fn test_std_asm_1_med(a: usize, b: usize) {
    std::assert_eq!(a, b);
}

#[inline(never)]
pub fn test_assert2_asm_1_med(a: usize, b: usize) {
    assert2::assert!(a == b);
}

#[test]
#[should_panic]
pub fn test_big_fail() {
    let x = [core::ptr::null::<()>(); 2];
    assert!(x != x);
}

#[test]
pub fn test_big() {
    let x = [core::ptr::null::<()>(); 2];
    assert!(x == x);
}

#[test]
#[should_panic]
pub fn test_custom_fail() {
    let approx_eq = ApproxEq {
        tol: 0.01,
        symbol: "~",
    };

    let x = 0.1;
    assert!(all(x ~ 0.2, x ~ 0.1, x ~ 0.3));
}

#[test]
pub fn test_custom() {
    let approx_eq = ApproxEq {
        tol: 0.01,
        symbol: "~",
    };

    assert!(0.1 :approx_eq: 0.10001);
    assert!(0.1 ~ 0.10001);
}
