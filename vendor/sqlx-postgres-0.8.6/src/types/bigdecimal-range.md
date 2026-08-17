#### Note: `BigDecimal` Has a Larger Range than `NUMERIC`
`BigDecimal` can represent values with a far, far greater range than the `NUMERIC` type in Postgres can.

`NUMERIC` is limited to 131,072 digits before the decimal point, and 16,384 digits after it. 
See [Section 8.1, Numeric Types] of the Postgres manual for details.

Meanwhile, `BigDecimal` can theoretically represent a value with an arbitrary number of decimal digits, albeit
with a maximum of 2<sup>63</sup> significant figures.

Because encoding in the current API design _must_ be infallible, 
when attempting to encode a `BigDecimal` that cannot fit in the wire representation of `NUMERIC`, 
SQLx may instead encode a sentinel value that falls outside the allowed range but is still representable.

This will cause the query to return a `DatabaseError` with code `22P03` (`invalid_binary_representation`)
and the error message `invalid scale in external "numeric" value` (though this may be subject to change).

However, `BigDecimal` should be able to decode any `NUMERIC` value except `NaN`, 
for which it has no representation.

[Section 8.1, Numeric Types]: https://www.postgresql.org/docs/current/datatype-numeric.html
