# float-next-after

[![codecov](https://codecov.io/gl/bronsonbdevost/next_afterf/branch/main/graph/badge.svg?token=CK1N3IYKFG)](https://codecov.io/gl/bronsonbdevost/next_afterf)
[![Documentation](https://docs.rs/float_next_after/badge.svg)](https://docs.rs/float_next_after)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://gitlab.com/bronsonbdevost/next_afterf/-/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/float_next_after)](https://crates.io/crates/float_next_after)
[![Crates.io](https://img.shields.io/crates/d/float_next_after)](https://crates.io/crates/float_next_after)

A native Rust next after float function, which is provided as a trait for f32 and f64 types. It steps to the next representable floating point, even if it is subnormal.  If a subnormal is undesired, users should handle that eventuality themselves. See [`CHANGES.md`](./CHANGES.md) for the versioned changelog.

In specific edge cases the following decisions have been made:

* self == y -> return y
* self >= positive infinity -> return positive infinity
* self <= negative infinity -> return negative infinity
* self or y == NaN -> return NaN
* self = -0.0 and y = 0.0 -> return positive 0.0
* self == -0.0 and y == positive infinity -> 5e-324

Please see the unit tests for the actual behavior in various other exceptional cases.

This code uses the ToBits and FromBits functions from f32 and f64. Those both simply wrap `unsafe { mem::transmute(self) }` / `unsafe { mem::transmute(v) }` to convert a f32/f64 to u32/u64.  The docs for those functions, however, claim that they are safe and that "the safety issues with sNaN were overblown!"

PR's and other helpful input are welcome.

## Usage

```rust
use float_next_after::NextAfter;

// Large numbers
let big_num = 16237485966.00000437586943_f64;
let next = big_num.next_after(std::f64::INFINITY);
assert_eq!(next, 16237485966.000006_f64);

// Expected handling of 1.0
let one = 1_f64;
let next = one.next_after(std::f64::INFINITY);
assert_eq!(next, 1_f64 + std::f64::EPSILON);

// Tiny (negative) numbers
let zero = 0_f32;
let next = zero.next_after(std::f32::NEG_INFINITY);
assert_eq!(next, -0.000000000000000000000000000000000000000000001_f32);

// Equal source/dest (even -0 == 0)
let zero = 0_f64;
let next = zero.next_after(-0_f64);
assert_eq!(next, -0_f64);
```
