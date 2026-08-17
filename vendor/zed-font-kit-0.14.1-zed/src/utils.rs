// font-kit/src/utils.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Miscellaneous utilities for use in this crate.

#![allow(dead_code)]

use std::fs::File;
use std::io::{Error as IOError, Read};

pub(crate) static SFNT_VERSIONS: [[u8; 4]; 4] = [
    [0x00, 0x01, 0x00, 0x00],
    [b'O', b'T', b'T', b'O'],
    [b't', b'r', b'u', b'e'],
    [b't', b'y', b'p', b'1'],
];

pub(crate) fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[inline]
pub(crate) fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[inline]
pub(crate) fn div_round_up(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

pub(crate) fn slurp_file(file: &mut File) -> Result<Vec<u8>, IOError> {
    let mut data = match file.metadata() {
        Ok(metadata) => Vec::with_capacity(metadata.len() as usize),
        Err(_) => vec![],
    };
    file.read_to_end(&mut data)?;
    Ok(data)
}
