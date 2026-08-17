// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate num as num_renamed;
#[macro_use]
extern crate num_derive;

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
enum Color {
    Red,
    Blue,
    Green,
}

#[test]
fn test_from_primitive_for_trivial_case() {
    let v: [Option<Color>; 4] = [
        num_renamed::FromPrimitive::from_u64(0),
        num_renamed::FromPrimitive::from_u64(1),
        num_renamed::FromPrimitive::from_u64(2),
        num_renamed::FromPrimitive::from_u64(3),
    ];

    assert_eq!(
        v,
        [
            Some(Color::Red),
            Some(Color::Blue),
            Some(Color::Green),
            None
        ]
    );
}

#[test]
fn test_to_primitive_for_trivial_case() {
    let v: [Option<u64>; 3] = [
        num_renamed::ToPrimitive::to_u64(&Color::Red),
        num_renamed::ToPrimitive::to_u64(&Color::Blue),
        num_renamed::ToPrimitive::to_u64(&Color::Green),
    ];

    assert_eq!(v, [Some(0), Some(1), Some(2)]);
}

#[test]
fn test_reflexive_for_trivial_case() {
    let before: [u64; 3] = [0, 1, 2];
    let after: Vec<Option<u64>> = before
        .iter()
        .map(|&x| -> Option<Color> { num_renamed::FromPrimitive::from_u64(x) })
        .map(|x| x.and_then(|x| num_renamed::ToPrimitive::to_u64(&x)))
        .collect();
    let before = before.iter().cloned().map(Some).collect::<Vec<_>>();

    assert_eq!(before, after);
}
