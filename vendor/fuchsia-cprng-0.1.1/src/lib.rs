// Copyright 2019 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Type-safe bindings for the Zircon kernel's CPRNG.

#![no_std]
#![deny(warnings)]

/// Draw random bytes from the kernel's CPRNG to fill the given buffer.
///
/// Wraps the
/// [zx_cprng_draw](https://fuchsia.googlesource.com/fuchsia/+/master/zircon/docs/syscalls/cprng_draw.md)
/// syscall.
pub fn cprng_draw(buffer: &mut [u8]) {
    unsafe { zx_cprng_draw(buffer.as_mut_ptr(), buffer.len()) };
}

#[link(name = "zircon")]
extern "C" {
    fn zx_cprng_draw(buffer: *mut u8, length: usize);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cprng() {
        let mut buffer = [0; 20];
        cprng_draw(&mut buffer);
        let mut first_zero = 0;
        let mut last_zero = 0;
        for _ in 0..30 {
            let mut buffer = [0; 20];
            cprng_draw(&mut buffer);
            if buffer[0] == 0 {
                first_zero += 1;
            }
            if buffer[19] == 0 {
                last_zero += 1;
            }
        }
        assert_ne!(first_zero, 30);
        assert_ne!(last_zero, 30);
    }

    #[test]
    fn cprng_large() {
        let mut buffer = [0; 1024];
        cprng_draw(&mut buffer);

        for mut s in buffer.chunks_mut(256) {
            cprng_draw(&mut s);
        }
    }
}
