#### Note: `rust_decimal::Decimal` Has a Smaller Range than `NUMERIC`
`NUMERIC` is can have up to 131,072 digits before the decimal point, and 16,384 digits after it. 
See [Section 8.1, Numeric Types] of the Postgres manual for details.

However, `rust_decimal::Decimal` is limited to a maximum absolute magnitude of 2<sup>96</sup> - 1, 
a number with 67 decimal digits, and a minimum absolute magnitude of 10<sup>-28</sup>, a number with, unsurprisingly,
28 decimal digits.

Thus, in contrast with `BigDecimal`, `NUMERIC` can actually represent every possible value of `rust_decimal::Decimal`,
but not the other way around. This means that encoding should never fail, but decoding can.
