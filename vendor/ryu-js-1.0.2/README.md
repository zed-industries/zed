# Ryū-js

Ryū-js is a fork of the [ryu][ryu-crate] crate adjusted to comply to the ECMAScript [number-to-string][number-to-string] algorithm, also implementing [`Number.prototype.toFixed()`][number-to-fixed] compliant conversion.

[ryu-crate]: https://crates.io/crates/ryu
[number-to-string]: https://tc39.es/ecma262/#sec-numeric-types-number-tostring
[number-to-fixed]: https://tc39.es/ecma262/#sec-number.prototype.tofixed

Pure Rust implementation of Ryū, an algorithm to quickly convert floating point
numbers to decimal strings.

The PLDI'18 paper [*Ryū: fast float-to-string conversion*][paper] by Ulf Adams
includes a complete correctness proof of the algorithm. The paper is available
under the creative commons CC-BY-SA license.

This Rust implementation is a line-by-line port of Ulf Adams' implementation in
C, [https://github.com/ulfjack/ryu][upstream].

*Requirements: This crate supports any compiler version back to rustc 1.64; it
uses nothing from the Rust standard library so is usable from no_std crates.*

[paper]: https://dl.acm.org/citation.cfm?id=3192369
[upstream]: https://github.com/ulfjack/ryu/tree/abf76d252bc97300354857e64e80d4a2bf664291

```toml
[dependencies]
ryu-js = "1.0"
```

<br>

## Example

```rust
fn main() {
    let mut buffer = ryu_js::Buffer::new();
    let printed = buffer.format(1.234);
    assert_eq!(printed, "1.234");
}
```

## Performance 

<!--

## Performance (lower is better)

![performance](https://raw.githubusercontent.com/boa-dev/ryu-js/master/performance.png)

-->

You can run upstream's benchmarks with:

```console
$ git clone https://github.com/ulfjack/ryu c-ryu
$ cd c-ryu
$ bazel run -c opt //ryu/benchmark:ryu_benchmark
```

And the same benchmark against our implementation with:

```console
$ git clone https://github.com/boa-dev/ryu-js rust-ryu
$ cd rust-ryu
$ cargo run --example upstream_benchmark --release
```

The benchmarks measure the average time to print a 32-bit float and average
time to print a 64-bit float, where the inputs are distributed as uniform random
bit patterns 32 and 64 bits wide.

The upstream C code, the unsafe direct Rust port, and the safe pretty Rust API
all perform the same, taking around 21 nanoseconds to format a 32-bit float and
31 nanoseconds to format a 64-bit float.

There is also a Rust-specific benchmark comparing this implementation to the
standard library which you can run with:

```console
$ cargo bench
```

The benchmark shows Ryū approximately 2-5x faster than the standard library
across a range of f32 and f64 inputs. Measurements are in nanoseconds per
iteration; smaller is better.

## Formatting

This library tends to produce more human-readable output than the standard
library's to\_string, which never uses scientific notation. Here are two
examples:

- *ryu:* 1.23e40, *std:* 12300000000000000000000000000000000000000
- *ryu:* 1.23e-40, *std:* 0.000000000000000000000000000000000000000123

Both libraries print short decimals such as 0.0000123 without scientific
notation.

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-BOOST">Boost Software License 1.0</a> at your
option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
