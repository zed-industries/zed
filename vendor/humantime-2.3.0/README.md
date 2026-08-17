Human Time
==========

**Status: stable**

[Documentation](https://docs.rs/humantime) |
[Github](https://github.com/tailhook/humantime) |
[Crate](https://crates.io/crates/humantime)


Features:

* Parses durations in free form like `15days 2min 2s`
* Formats durations in similar form `2years 2min 12us`
* Parses and formats timestamp in `rfc3339` format: `2018-01-01T12:53:00Z`
* Parses timestamps in a weaker format: `2018-01-01 12:53:00`

Timestamp parsing/formatting is super-fast because format is basically
fixed.

Here are some micro-benchmarks:

```
test result: ok. 0 passed; 0 failed; 26 ignored; 0 measured; 0 filtered out

     Running target/release/deps/datetime_format-8facb4ac832d9770

running 2 tests
test rfc3339_chrono            ... bench:         737 ns/iter (+/- 37)
test rfc3339_humantime_seconds ... bench:          73 ns/iter (+/- 2)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out

     Running target/release/deps/datetime_parse-342628f877d7867c

running 6 tests
test datetime_utc_parse_millis  ... bench:         228 ns/iter (+/- 11)
test datetime_utc_parse_nanos   ... bench:         236 ns/iter (+/- 10)
test datetime_utc_parse_seconds ... bench:         204 ns/iter (+/- 18)
test rfc3339_humantime_millis   ... bench:          28 ns/iter (+/- 1)
test rfc3339_humantime_nanos    ... bench:          36 ns/iter (+/- 2)
test rfc3339_humantime_seconds  ... bench:          24 ns/iter (+/- 1)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 0 filtered out
```

See [humantime-serde] for serde integration (previous crate [serde-humantime] looks unmaintained).

[serde-humantime]: https://docs.rs/serde-humantime/0.1.1/serde_humantime/
[humantime-serde]: https://docs.rs/humantime-serde

License
=======

Licensed under either of

* Apache License, Version 2.0, (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
