// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use syn::{self, BinOp as B, Expr as E, Lit as L, UnOp as U};

/// Adapted from https://docs.rs/syn/0.14.2/src/syn/lit.rs.html#943 to accept
/// u128.
fn parse_lit_int(mut s: &str) -> Option<u128> {
    /// Get the byte at offset idx, or a default of `b'\0'` if we're looking
    /// past the end of the input buffer.
    pub fn byte<S: AsRef<[u8]> + ?Sized>(s: &S, idx: usize) -> u8 {
        let s = s.as_ref();
        if idx < s.len() {
            s[idx]
        } else {
            0
        }
    }

    let base = match (byte(s, 0), byte(s, 1)) {
        (b'0', b'x') => {
            s = &s[2..];
            16
        }
        (b'0', b'o') => {
            s = &s[2..];
            8
        }
        (b'0', b'b') => {
            s = &s[2..];
            2
        }
        (b'0'..=b'9', _) => 10,
        _ => unreachable!(),
    };

    let mut value = 0u128;
    loop {
        let b = byte(s, 0);
        let digit = match b {
            b'0'..=b'9' => u128::from(b - b'0'),
            b'a'..=b'f' if base > 10 => 10 + u128::from(b - b'a'),
            b'A'..=b'F' if base > 10 => 10 + u128::from(b - b'A'),
            b'_' => {
                s = &s[1..];
                continue;
            }
            // NOTE: Looking at a floating point literal, we don't want to
            // consider these integers.
            b'.' if base == 10 => return None,
            b'e' | b'E' if base == 10 => return None,
            _ => break,
        };

        if digit >= base {
            panic!("Unexpected digit {:x} out of base range", digit);
        }

        value = value.checked_mul(base)?.checked_add(digit)?;
        s = &s[1..];
    }

    Some(value)
}

/// Parse a suffix of an integer literal.
fn parse_suffix(lit: &str) -> Option<&'static str> {
    [
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
        "u128", "usize",
    ]
    .iter()
    .find(|s| lit.ends_with(*s))
    .copied()
}

/// Interprets an integer literal in a string.
fn eval_str_int(lit: &str) -> Option<u128> {
    let val = parse_lit_int(lit)?;
    let checked_val = if let Some(suffix) = parse_suffix(lit) {
        match suffix {
            "i8" if val <= i8::MAX as u128 => val,
            "i16" if val <= i16::MAX as u128 => val,
            "i32" if val <= i32::MAX as u128 => val,
            "i64" if val <= i64::MAX as u128 => val,
            "u8" if val <= u128::from(u8::MAX) => val,
            "u16" if val <= u128::from(u16::MAX) => val,
            "u32" if val <= u128::from(u32::MAX) => val,
            "u64" if val <= u128::from(u64::MAX) => val,
            "usize" if val <= usize::MAX as u128 => val,
            "isize" if val <= isize::MAX as u128 => val,
            "u128" => val,
            "i128" if val <= i128::MAX as u128 => val,

            // Does not fit in suffix:
            _ => return None,
        }
    } else {
        val
    };

    Some(checked_val)
}

/// Interprets an integer literal.
fn eval_lit_int(lit: &syn::LitInt) -> Option<u128> {
    let lit = lit.to_string();
    eval_str_int(&lit)
}

/// Interprets a verbatim literal.
fn eval_lit_verbatim(lit: &proc_macro2::Literal) -> Option<u128> {
    let lit = lit.to_string();
    eval_str_int(&lit)
}

/// Interprets a literal.
fn eval_lit(lit: &syn::ExprLit) -> Option<u128> {
    match &lit.lit {
        L::Int(lit) => eval_lit_int(lit),
        L::Byte(lit) => Some(u128::from(lit.value())),
        L::Verbatim(lit) => eval_lit_verbatim(lit),
        _ => None,
    }
}

/// Interprets a binary operator on two expressions.
fn eval_binary(bin: &syn::ExprBinary) -> Option<u128> {
    use std::u32;

    let l = eval_expr(&bin.left)?;
    let r = eval_expr(&bin.right)?;
    Some(match bin.op {
        B::Add(_) => l.checked_add(r)?,
        B::Sub(_) => l.checked_sub(r)?,
        B::Mul(_) => l.checked_mul(r)?,
        B::Div(_) => l.checked_div(r)?,
        B::Rem(_) => l.checked_rem(r)?,
        B::BitXor(_) => l ^ r,
        B::BitAnd(_) => l & r,
        B::BitOr(_) => l | r,
        B::Shl(_) if r <= u128::from(u32::MAX) => l.checked_shl(r as u32)?,
        B::Shr(_) if r <= u128::from(u32::MAX) => l.checked_shr(r as u32)?,
        _ => return None,
    })
}

/// Interprets unary operator on an expression.
fn eval_unary(expr: &syn::ExprUnary) -> Option<u128> {
    if let U::Not(_) = expr.op {
        Some(!eval_expr(&expr.expr)?)
    } else {
        None
    }
}

/// A **very** simple CTFE interpreter for some basic arithmetic:
pub fn eval_expr(expr: &E) -> Option<u128> {
    match expr {
        E::Lit(expr) => eval_lit(expr),
        E::Binary(expr) => eval_binary(expr),
        E::Unary(expr) => eval_unary(expr),
        E::Paren(expr) => eval_expr(&expr.expr),
        E::Group(expr) => eval_expr(&expr.expr),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn eval(expr: &str) -> Option<u128> {
        use syn::parse_str;
        eval_expr(&parse_str(expr).expect("not a valid expression"))
    }

    macro_rules! test {
        ($($name: ident, $case: expr => $result:expr;)*) => {$(
            #[test] fn $name() { assert_eq!(eval($case), $result); }
        )*};
    }

    test! {
        accept_lit_bare, "1" => Some(1);
        accept_lit_bare_max, "340282366920938463463374607431768211455"
            => Some(340282366920938463463374607431768211455);
        reject_lit_bare_overflow, "340282366920938463463374607431768211456" => None;
        accept_lit_u8_max, "255u8" => Some(255);
        accept_lit_u16_max, "65535u16" => Some(65535);
        accept_lit_u32_max, "4294967295u32" => Some(4294967295);
        accept_lit_u64_max, "18446744073709551615u64" => Some(18446744073709551615);
        accept_lit_u128_max, "340282366920938463463374607431768211455u128"
            => Some(340282366920938463463374607431768211455);
        reject_lit_u8_overflow, "256u8" => None;
        reject_lit_u16_overflow, "65536u16" => None;
        reject_lit_u32_overflow, "4294967296u32" => None;
        reject_lit_u64_overflow, "18446744073709551616u64" => None;
        reject_lit_u128_overflow, "340282366920938463463374607431768211456u128" => None;
        accept_lit_i8_max, "127i8" => Some(127);
        accept_lit_i16_max, "32767i16" => Some(32767);
        accept_lit_i32_max, "2147483647i32" => Some(2147483647);
        accept_lit_i64_max, "9223372036854775807i64" => Some(9223372036854775807);
        accept_lit_i128_max, "170141183460469231731687303715884105727i128"
            => Some(170141183460469231731687303715884105727);
        reject_lit_i8_overflow, "128i8" => None;
        reject_lit_i16_overflow, "32768i16" => None;
        reject_lit_i32_overflow, "2147483648i32" => None;
        reject_lit_i64_overflow, "9223372036854775808i64" => None;
        reject_lit_i128_overflow, "170141183460469231731687303715884105728i128" => None;
        accept_lit_usize, "42usize" => Some(42);
        accept_lit_isize, "42isize" => Some(42);
        accept_lit_byte, "b'0'" => Some(48);
        reject_lit_negative, "-42" => None;
        accept_add_10_20, "10 + 20" => Some(30);
        accept_add_10u8_20u16, "10u8 + 20u16" => Some(30);
        reject_add_overflow, "340282366920938463463374607431768211456u128 + 1" => None;
        accept_add_commutes, "20 + 10" => Some(30);
        accept_add_5_numbers, "(10 + 20) + 30 + (40 + 50)" => Some(150);
        accept_add_10_0, "10 + 0" => Some(10);
        accept_sub_20_10, "20 - 10" => Some(10);
        reject_sub_10_20, "10 - 20" => None;
        reject_sub_10_11, "10 - 11" => None;
        accept_sub_10_10, "10 - 10" => Some(0);
        accept_mul_42_0, "42 * 0" => Some(0);
        accept_mul_0_42, "0 * 42" => Some(0);
        accept_mul_42_1, "42 * 1" => Some(42);
        accept_mul_1_42, "1 * 42" => Some(42);
        accept_mul_3_4, "3 * 4" => Some(12);
        accept_mul_4_3, "4 * 3" => Some(12);
        accept_mul_1_2_3_4_5, "(1 * 2) * 3 * (4 * 5)" => Some(120);
        reject_div_with_0, "10 / 0" => None;
        accept_div_42_1, "42 / 1" => Some(42);
        accept_div_42_42, "42 / 42" => Some(1);
        accept_div_20_10, "20 / 10" => Some(2);
        accept_div_10_20, "10 / 20" => Some(0);
        reject_rem_with_0, "10 % 0" => None;
        accept_rem_0_4, "0 % 4" => Some(0);
        accept_rem_4_4, "4 % 4" => Some(0);
        accept_rem_8_4, "8 % 4" => Some(0);
        accept_rem_1_4, "1 % 4" => Some(1);
        accept_rem_5_4, "5 % 4" => Some(1);
        accept_rem_2_4, "2 % 4" => Some(2);
        accept_rem_3_4, "3 % 4" => Some(3);
        accept_xor_1, "0b0000 ^ 0b1111" => Some(0b1111);
        accept_xor_2, "0b1111 ^ 0b0000" => Some(0b1111);
        accept_xor_3, "0b1111 ^ 0b1111" => Some(0b0000);
        accept_xor_4, "0b0000 ^ 0b0000" => Some(0b0000);
        accept_xor_5, "0b1100 ^ 0b0011" => Some(0b1111);
        accept_xor_6, "0b1001 ^ 0b1111" => Some(0b0110);
        accept_and_1, "0b0000 & 0b0000" => Some(0b0000);
        accept_and_2, "0b1001 & 0b0101" => Some(0b0001);
        accept_and_3, "0b1111 & 0b1111" => Some(0b1111);
        accept_or_1, "0b0000 | 0b0000" => Some(0b0000);
        accept_or_2, "0b1001 | 0b0101" => Some(0b1101);
        accept_or_3, "0b1111 | 0b1111" => Some(0b1111);
        accept_shl, "0b001000 << 2" => Some(0b100000);
        accept_shr, "0b001000 >> 2" => Some(0b000010);
        accept_shl_zero, "0b001000 << 0" => Some(0b001000);
        accept_shr_zero, "0b001000 >> 0" => Some(0b001000);
        reject_shl_rhs_not_u32, "0b001000 << 4294967296" => None;
        reject_shl_overflow, "0b001000 << 429496" => None;
        reject_shr_rhs_not_u32, "0b001000 >> 4294967296" => None;
        reject_shr_underflow, "0b001000 >> 429496" => None;
        accept_complex_arith, "(3 + 4 * 2 - 5) / 6" => Some(1);
    }
}
