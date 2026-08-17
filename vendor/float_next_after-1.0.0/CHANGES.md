# Rust Float NextAfter Release Notes

A native Rust next after float function, which is provided as a trait for f32 and f64 types. It steps to the next representable floating point, even if it is subnormal. See [`README.md`](./README.md) for usage instructions.

## Known issues

## Changes

### Version 1.0.0

Breaking changes:

- The NextAfter trait is now provided for the types `f32` and `f64` and no longer provided as a generic constrained to the type `num_traits::Float`.
- The library is now `#![no_std]`.

New Features:

- Removed dependence on the std library, the library is now `#![no_std]`.
- Now uses macros to create the trait for `f32` and `f64` directly without any use of the generic contraint using `num_traits::Float`.
- CI/CD test running on git push.

Structural changes:

- Tests are written with a macro now so no code needs to be repeated to run tests for `f32` and `f64`.
- Code cleanup with:
  - 0 equality check integrated into `short_circuit_operands`.
  - Early return is used throughout.
  - Single check for infinite source number (it had been a separate check for positive and for negative infinity).

Performance:

- Removed one unneccesary creation of a stack variable.

### Version 0.7.0

Bug fixes:

- Updated the examples in `README.md`
