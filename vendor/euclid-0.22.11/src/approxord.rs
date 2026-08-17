// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utilities for testing approximate ordering - especially true for
//! floating point types, where NaN's cannot be ordered.

pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y {
        x
    } else {
        y
    }
}

pub fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x >= y {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        assert!(min(0u32, 1u32) == 0u32);
        assert!(min(-1.0f32, 0.0f32) == -1.0f32);
    }

    #[test]
    fn test_max() {
        assert!(max(0u32, 1u32) == 1u32);
        assert!(max(-1.0f32, 0.0f32) == 0.0f32);
    }
}
