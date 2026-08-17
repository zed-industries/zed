# Release 0.2.2 (2018-12-14)

- [The `Roots` implementations now use better initial guesses][71].
- [Fixed `to_signed_bytes_*` for some positive numbers][72], where the
  most-significant byte is `0x80` and the rest are `0`.

[71]: https://github.com/rust-num/num-bigint/pull/71
[72]: https://github.com/rust-num/num-bigint/pull/72

**Contributors**: @cuviper, @leodasvacas

# Release 0.2.1 (2018-11-02)

- [`RandBigInt` now uses `Rng::fill_bytes`][53] to improve performance, instead
  of repeated `gen::<u32>` calls.  The also affects the implementations of the
  other `rand` traits.  This may potentially change the values produced by some
  seeded RNGs on previous versions, but the values were tested to be stable
  with `ChaChaRng`, `IsaacRng`, and `XorShiftRng`.
- [`BigInt` and `BigUint` now implement `num_integer::Roots`][56].
- [`BigInt` and `BigUint` now implement `num_traits::Pow`][54].
- [`BigInt` and `BigUint` now implement operators with 128-bit integers][64].

**Contributors**: @cuviper, @dignifiedquire, @mancabizjak, @Robbepop,
@TheIronBorn, @thomwiggers

[53]: https://github.com/rust-num/num-bigint/pull/53
[54]: https://github.com/rust-num/num-bigint/pull/54
[56]: https://github.com/rust-num/num-bigint/pull/56
[64]: https://github.com/rust-num/num-bigint/pull/64

# Release 0.2.0 (2018-05-25)

### Enhancements

- [`BigInt` and `BigUint` now implement `Product` and `Sum`][22] for iterators
  of any item that we can `Mul` and `Add`, respectively.  For example, a
  factorial can now be simply: `let f: BigUint = (1u32..1000).product();`
- [`BigInt` now supports two's-complement logic operations][26], namely
  `BitAnd`, `BitOr`, `BitXor`, and `Not`.  These act conceptually as if each
  number had an infinite prefix of `0` or `1` bits for positive or negative.
- [`BigInt` now supports assignment operators][41] like `AddAssign`.
- [`BigInt` and `BigUint` now support conversions with `i128` and `u128`][44],
  if sufficient compiler support is detected.
- [`BigInt` and `BigUint` now implement rand's `SampleUniform` trait][48], and
  [a custom `RandomBits` distribution samples by bit size][49].
- The release also includes other miscellaneous improvements to performance.

### Breaking Changes

- [`num-bigint` now requires rustc 1.15 or greater][23].
- [The crate now has a `std` feature, and won't build without it][46].  This is
  in preparation for someday supporting `#![no_std]` with `alloc`.
- [The `serde` dependency has been updated to 1.0][24], still disabled by
  default.  The `rustc-serialize` crate is no longer supported by `num-bigint`.
- [The `rand` dependency has been updated to 0.5][48], now disabled by default.
  This requires rustc 1.22 or greater for `rand`'s own requirement.
- [`Shr for BigInt` now rounds down][8] rather than toward zero, matching the
  behavior of the primitive integers for negative values.
- [`ParseBigIntError` is now an opaque type][37].
- [The `big_digit` module is no longer public][38], nor are the `BigDigit` and
  `DoubleBigDigit` types and `ZERO_BIG_DIGIT` constant that were re-exported in
  the crate root.  Public APIs which deal in digits, like `BigUint::from_slice`,
  will now always be base-`u32`.

**Contributors**: @clarcharr, @cuviper, @dodomorandi, @tiehuis, @tspiteri

[8]: https://github.com/rust-num/num-bigint/pull/8
[22]: https://github.com/rust-num/num-bigint/pull/22
[23]: https://github.com/rust-num/num-bigint/pull/23
[24]: https://github.com/rust-num/num-bigint/pull/24
[26]: https://github.com/rust-num/num-bigint/pull/26
[37]: https://github.com/rust-num/num-bigint/pull/37
[38]: https://github.com/rust-num/num-bigint/pull/38
[41]: https://github.com/rust-num/num-bigint/pull/41
[44]: https://github.com/rust-num/num-bigint/pull/44
[46]: https://github.com/rust-num/num-bigint/pull/46
[48]: https://github.com/rust-num/num-bigint/pull/48
[49]: https://github.com/rust-num/num-bigint/pull/49

# Release 0.1.44 (2018-05-14)

- [Division with single-digit divisors is now much faster.][42]
- The README now compares [`ramp`, `rug`, `rust-gmp`][20], and [`apint`][21].

**Contributors**: @cuviper, @Robbepop

[20]: https://github.com/rust-num/num-bigint/pull/20
[21]: https://github.com/rust-num/num-bigint/pull/21
[42]: https://github.com/rust-num/num-bigint/pull/42

# Release 0.1.43 (2018-02-08)

- [The new `BigInt::modpow`][18] performs signed modular exponentiation, using
  the existing `BigUint::modpow` and rounding negatives similar to `mod_floor`.

**Contributors**: @cuviper

[18]: https://github.com/rust-num/num-bigint/pull/18


# Release 0.1.42 (2018-02-07)

- [num-bigint now has its own source repository][num-356] at [rust-num/num-bigint][home].
- [`lcm` now avoids creating a large intermediate product][num-350].
- [`gcd` now uses Stein's algorithm][15] with faster shifts instead of division.
- [`rand` support is now extended to 0.4][11] (while still allowing 0.3).

**Contributors**: @cuviper, @Emerentius, @ignatenkobrain, @mhogrefe

[home]: https://github.com/rust-num/num-bigint
[num-350]: https://github.com/rust-num/num/pull/350
[num-356]: https://github.com/rust-num/num/pull/356
[11]: https://github.com/rust-num/num-bigint/pull/11
[15]: https://github.com/rust-num/num-bigint/pull/15


# Prior releases

No prior release notes were kept.  Thanks all the same to the many
contributors that have made this crate what it is!

