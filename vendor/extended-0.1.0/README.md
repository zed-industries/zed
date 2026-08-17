# 80-bit Extended-Precision Floating-Point Numbers

This is a Rust library that provides a type for representing 80-bit extended-precision floating-point numbers. It is licensed under the terms of the MIT license, see [LICENSE.txt](LICENSE.txt) for details.

## Rounding, Infinity, and NaN

This library uses round-to-even when converting from 80-bit floats to 64-bit floats. This should be what you’re used to, and what you expect! In round-to-even, when an 80-bit float is exactly half-way between two possible `float64` values, the value with a zero in the least-significant bit is chosen (or the value with the larger exponent is chosen, if the values have different exponents).

Values which are outside the range of possible `float64` values are rounded to infinity.

Infinity and NaN are preserved. Different types of NaN values are not distinguished from each other, but the sign of NaN values is preserved during conversion.
