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
    Blue = 5,
    Green,
    Alpha = (-3 - (-5isize)) - 10,
}

#[test]
fn test_from_primitive_for_enum_with_custom_value() {
    let v: [Option<Color>; 5] = [
        num_renamed::FromPrimitive::from_u64(0),
        num_renamed::FromPrimitive::from_u64(5),
        num_renamed::FromPrimitive::from_u64(6),
        num_renamed::FromPrimitive::from_u64(-8isize as u64),
        num_renamed::FromPrimitive::from_u64(3),
    ];

    assert_eq!(
        v,
        [
            Some(Color::Red),
            Some(Color::Blue),
            Some(Color::Green),
            Some(Color::Alpha),
            None
        ]
    );
}

#[test]
fn test_to_primitive_for_enum_with_custom_value() {
    let v: [Option<u64>; 4] = [
        num_renamed::ToPrimitive::to_u64(&Color::Red),
        num_renamed::ToPrimitive::to_u64(&Color::Blue),
        num_renamed::ToPrimitive::to_u64(&Color::Green),
        num_renamed::ToPrimitive::to_u64(&Color::Alpha),
    ];

    assert_eq!(v, [Some(0), Some(5), Some(6), Some(-8isize as u64)]);
}

#[test]
fn test_reflexive_for_enum_with_custom_value() {
    let before: [u64; 3] = [0, 5, 6];
    let after: Vec<Option<u64>> = before
        .iter()
        .map(|&x| -> Option<Color> { num_renamed::FromPrimitive::from_u64(x) })
        .map(|x| x.and_then(|x| num_renamed::ToPrimitive::to_u64(&x)))
        .collect();
    let before = before.iter().cloned().map(Some).collect::<Vec<_>>();

    assert_eq!(before, after);
}
