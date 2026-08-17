jiff-static
===========
This is an optional dependency of `jiff` that embeds time zone data into your
binary via procedural macros. It unlocks the use case of creating a `TimeZone`
value in core-only environments without dynamic memory allocation.

Users should generally not depend on this directly, but instead use it
through Jiff. Namely, all of the procedural macros defined in this crate
are re-exported through Jiff's public API. For example, one can enable the
`static` or `static-tz` crate features in `jiff` to get `jiff::tz::get!` and
`jiff::tz::include!`.

**WARNING**: The `src/shared` directory in this crate is copied from the
`../src/shared` directory. This copy is managed by `jiff-cli generate shared`.
See the comments in the code for why this is done.

### Documentation

https://docs.rs/jiff-static
