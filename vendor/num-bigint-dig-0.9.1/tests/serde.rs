//! Test serialization and deserialization of `BigUint` and `BigInt`
//!
//! The serialized formats should not change, even if we change our
//! internal representation, because we want to preserve forward and
//! backward compatibility of serialized data!

#![cfg(feature = "serde")]

extern crate num_bigint_dig as num_bigint;
extern crate num_traits;
extern crate serde_test;

use crate::num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use serde_test::{assert_tokens, Token};

#[test]
fn biguint_zero() {
    let tokens = [Token::Seq { len: Some(0) }, Token::SeqEnd];
    assert_tokens(&BigUint::zero(), &tokens);
}

#[test]
fn bigint_zero() {
    let tokens = [
        Token::Tuple { len: 2 },
        Token::I8(0),
        Token::Seq { len: Some(0) },
        Token::SeqEnd,
        Token::TupleEnd,
    ];
    assert_tokens(&BigInt::zero(), &tokens);
}

#[test]
fn biguint_one() {
    let tokens = [Token::Seq { len: Some(1) }, Token::U32(1), Token::SeqEnd];
    assert_tokens(&BigUint::one(), &tokens);
}

#[test]
fn bigint_one() {
    let tokens = [
        Token::Tuple { len: 2 },
        Token::I8(1),
        Token::Seq { len: Some(1) },
        Token::U32(1),
        Token::SeqEnd,
        Token::TupleEnd,
    ];
    assert_tokens(&BigInt::one(), &tokens);
}

#[test]
fn bigint_negone() {
    let tokens = [
        Token::Tuple { len: 2 },
        Token::I8(-1),
        Token::Seq { len: Some(1) },
        Token::U32(1),
        Token::SeqEnd,
        Token::TupleEnd,
    ];
    assert_tokens(&-BigInt::one(), &tokens);
}

// Generated independently from python `hex(factorial(100))`
const FACTORIAL_100: &'static [u32] = &[
    0x00000000, 0x00000000, 0x00000000, 0x2735c61a, 0xee8b02ea, 0xb3b72ed2, 0x9420c6ec, 0x45570cca,
    0xdf103917, 0x943a321c, 0xeb21b5b2, 0x66ef9a70, 0xa40d16e9, 0x28d54bbd, 0xdc240695, 0x964ec395,
    0x1b30,
];

#[test]
fn biguint_factorial_100() {
    let n: BigUint = (1u8..101).product();

    let mut tokens = vec![];
    tokens.push(Token::Seq {
        len: Some(FACTORIAL_100.len()),
    });
    tokens.extend(FACTORIAL_100.iter().map(|&u| Token::U32(u)));
    tokens.push(Token::SeqEnd);

    assert_tokens(&n, &tokens);
}

#[test]
fn bigint_factorial_100() {
    let n: BigInt = (1i8..101).product();

    let mut tokens = vec![];
    tokens.push(Token::Tuple { len: 2 });
    tokens.push(Token::I8(1));
    tokens.push(Token::Seq {
        len: Some(FACTORIAL_100.len()),
    });
    tokens.extend(FACTORIAL_100.iter().map(|&u| Token::U32(u)));
    tokens.push(Token::SeqEnd);
    tokens.push(Token::TupleEnd);

    assert_tokens(&n, &tokens);
}
