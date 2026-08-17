// Copyright 2019 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Support for lookups based on minimal perfect hashing.

// This function is based on multiplication being fast and is "good enough". Also
// it can share some work between the unsalted and salted versions.
#[inline]
fn my_hash(key: u32, salt: u32, n: usize) -> usize {
    let y = key.wrapping_add(salt).wrapping_mul(2654435769);
    let y = y ^ key.wrapping_mul(0x31415926);
    (((y as u64) * (n as u64)) >> 32) as usize
}

/// Do a lookup using minimal perfect hashing.
///
/// The table is stored as a sequence of "salt" values, then a sequence of
/// values that contain packed key/value pairs. The strategy is to hash twice.
/// The first hash retrieves a salt value that makes the second hash unique.
/// The hash function doesn't have to be very good, just good enough that the
/// resulting map is unique.
#[inline]
pub(crate) fn mph_lookup<KV, V, FK, FV>(
    x: u32,
    salt: &[u16],
    kv: &[KV],
    fk: FK,
    fv: FV,
    default: V,
) -> V
where
    KV: Copy,
    FK: Fn(KV) -> u32,
    FV: Fn(KV) -> V,
{
    let s = salt[my_hash(x, 0, salt.len())] as u32;
    let key_val = kv[my_hash(x, s, salt.len())];
    if x == fk(key_val) {
        fv(key_val)
    } else {
        default
    }
}
